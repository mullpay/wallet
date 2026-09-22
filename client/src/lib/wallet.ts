import { SparkWallet, encodeBech32mTokenIdentifier, type NetworkType } from "@buildonspark/spark-sdk";
import { auth } from "$lib/auth";
import { PUBLIC_NETWORK } from "$env/static/public";
import { SvelteMap } from "svelte/reactivity";
import { ExitSpeed } from "@buildonspark/spark-sdk/types";
import type { TokenTransaction } from "@buildonspark/spark-sdk/proto/spark_token";
export { ExitSpeed } from "@buildonspark/spark-sdk/types";
import { FlashnetClient, decodeSparkHumanReadableTokenIdentifier } from "@flashnet/sdk";
import { bytesToNumberBE } from "@noble/curves/utils.js";
import type { AssetId } from "$lib/assets";
import { BTC_ASSET_ID } from "$lib/assets";
import { confirmChallenge, createDeposit, getDepositLimit, requestChallenge } from "$lib/api";
import { deriveKeys, sha256, sign } from "$lib/crypto";
import { bytesToHex, stringToUint8Array } from "$lib/utils";
import { identifyTransactionId } from "./validation";

export type Balance = {
	ownedBalance: bigint;
	availableToSendBalance: bigint;
};

export interface TransactionInput {
	asset: AssetId;
	amount: bigint;
}

export interface TransactionOutput {
	asset: AssetId;
	amount: bigint;
}

export interface Transaction {
	id: string;
	inputs: TransactionInput[];
	outputs: TransactionOutput[];
	createdAt: Date;
}

export type PoolSortOrder =
	"CREATED_AT_DESC" | "CREATED_AT_ASC" | "VOLUME24H_DESC" | "VOLUME24H_ASC" | "TVL_DESC" | "TVL_ASC";

export interface ListPoolsQuery {
	assetA?: AssetId;
	assetB?: AssetId;
	hostNames?: string[];
	minVolume24h?: number;
	minTvl?: number;
	curveTypes?: string[];
	sort?: PoolSortOrder;
	limit?: number;
	offset?: number;
	afterUpdatedAt?: string;
}

export interface SimulateSwapResponse {
	amountOut: bigint;
	executionPrice?: string;
	feePaidAssetIn?: bigint;
	priceImpactPct?: string;
	warningMessage?: string;
}

export class Wallet {
	private readonly DEPOSIT_CLAIM_INTERVAL = 1 * 60 * 1000;
	private depositClaimTimer?: ReturnType<typeof setInterval>;

	private readonly sparkWallet: SparkWallet;
	private readonly flashnet: FlashnetClient;

	private apiToken?: string;

	public readonly mnemonic: string;

	private constructor(sparkWallet: SparkWallet, flashnet: FlashnetClient, mnemonic: string) {
		this.sparkWallet = sparkWallet;
		this.flashnet = flashnet;
		this.mnemonic = mnemonic;

		this.startDepositClaimJob();
	}

	static async initialize() {
		let mnemonic = auth();

		if (!mnemonic) {
			return undefined;
		}

		if (cache && cache.mnemonic === mnemonic) {
			return cache;
		}

		const { wallet: sparkWallet } = await SparkWallet.initialize({
			mnemonicOrSeed: mnemonic,
			options: {
				network: PUBLIC_NETWORK as NetworkType
			}
		});

		await sparkWallet.setPrivacyEnabled(true);

		const flashnet = new FlashnetClient(sparkWallet);
		await flashnet.initialize();

		cache = new Wallet(sparkWallet, flashnet, mnemonic);

		return cache;
	}

	private startDepositClaimJob() {
		if (this.depositClaimTimer) {
			clearInterval(this.depositClaimTimer);
		}

		void this.claimDeposits();

		this.depositClaimTimer = setInterval(() => {
			void this.claimDeposits();
		}, this.DEPOSIT_CLAIM_INTERVAL);
	}

	async transfer(receiverSparkAddress: string, amount: bigint, asset: AssetId = BTC_ASSET_ID) {
		if (asset == BTC_ASSET_ID) {
			return await this.sparkWallet.transfer({ receiverSparkAddress, amountSats: Number(amount) });
		} else {
			return await this.sparkWallet.transferTokens({ receiverSparkAddress, tokenAmount: amount, tokenIdentifier: asset });
		}
	}

	async withdraw(onchainAddress: string, amountSats: bigint, exitSpeed: ExitSpeed) {
		const feeQuote = await this.sparkWallet.getWithdrawalFeeQuote({
			amountSats: Number(amountSats),
			withdrawalAddress: onchainAddress
		});

		if (!feeQuote) {
			throw new Error("No withdrawal fee quote available");
		}

		return await this.sparkWallet.withdraw({
			onchainAddress,
			amountSats: Number(amountSats),
			exitSpeed,
			feeQuote
		});
	}

	async payLightningInvoice(invoice: string, maxFeeSats: bigint, amountSatsToSend?: bigint) {
		return await this.sparkWallet.payLightningInvoice({
			invoice,
			maxFeeSats: Number(maxFeeSats),
			amountSatsToSend: amountSatsToSend ? Number(amountSatsToSend) : undefined
		});
	}

	async getBitcoinTransactions(): Promise<Transaction[]> {
		const transfers = await this.sparkWallet.getTransfers();

		return transfers.transfers.map((tx) => {
			const id = tx.id;
			const inputs: TransactionInput[] = tx.valueSentByWallet
				? [{ asset: BTC_ASSET_ID, amount: BigInt(tx.valueSentByWallet) }]
				: [];
			const outputs: TransactionOutput[] = tx.valueReceivedByWallet
				? [{ asset: BTC_ASSET_ID, amount: BigInt(tx.valueReceivedByWallet) }]
				: [];
			const createdAt = tx.createdTime!;

			return {
				id,
				inputs,
				outputs,
				createdAt
			};
		});
	}

	async getTokenTransactions(): Promise<Transaction[]> {
		const [address, walletIdentityPublicKey] = await Promise.all([
			this.sparkWallet.getSparkAddress(),
			this.sparkWallet.getIdentityPublicKey()
		]);

		const { tokenTransactionsWithStatus: entries } = await this.sparkWallet.queryTokenTransactionsWithFilters({
			sparkAddresses: [address]
		});

		const transactionsByHash = new Map<string, TokenTransaction | undefined>();

		for (const { tokenTransactionHash, tokenTransaction } of entries) {
			if (tokenTransaction) transactionsByHash.set(bytesToHex(tokenTransactionHash), tokenTransaction);
			if (tokenTransaction?.tokenInputs?.$case !== "transferInput") {
				continue;
			}

			for (const input of tokenTransaction.tokenInputs.transferInput.outputsToSpend) {
				const hash = bytesToHex(input.prevTokenTransactionHash);

				if (!transactionsByHash.has(hash)) {
					transactionsByHash.set(hash, undefined);
				}
			}
		}

		const missingHashes = [...transactionsByHash].filter(([_, tx]) => !tx).map(([hash, _]) => hash);

		if (missingHashes.length) {
			const previous = await this.sparkWallet.queryTokenTransactionsByTxHashes(missingHashes);
			for (const { tokenTransactionHash, tokenTransaction } of previous.tokenTransactionsWithStatus) {
				transactionsByHash.set(bytesToHex(tokenTransactionHash), tokenTransaction);
			}
		}

		const tokenTransactions: Transaction[] = [];

		for (const { tokenTransactionHash, tokenTransaction } of entries) {
			if (!tokenTransaction) continue;

			const inputs = [];
			const outputs = [];

			for (const output of tokenTransaction.tokenOutputs) {
				const ownerPublicKey = bytesToHex(output.ownerPublicKey);

				if (ownerPublicKey !== walletIdentityPublicKey || !output.tokenIdentifier) {
					continue;
				}

				const asset = encodeBech32mTokenIdentifier({
					tokenIdentifier: output.tokenIdentifier,
					network: PUBLIC_NETWORK as NetworkType
				});
				const amount = bytesToNumberBE(output.tokenAmount);

				outputs.push({
					asset,
					amount
				});
			}

			if (tokenTransaction.tokenInputs?.$case === "transferInput") {
				for (const input of tokenTransaction.tokenInputs.transferInput.outputsToSpend) {
					const prevTokenTransactionHash = bytesToHex(input.prevTokenTransactionHash);
					const previous = transactionsByHash.get(prevTokenTransactionHash);
					const output = previous!.tokenOutputs[input.prevTokenTransactionVout];
					const ownerPublicKey = bytesToHex(output.ownerPublicKey);

					if (ownerPublicKey !== walletIdentityPublicKey || !output.tokenIdentifier) {
						continue;
					}

					const asset = encodeBech32mTokenIdentifier({
						tokenIdentifier: output.tokenIdentifier,
						network: PUBLIC_NETWORK as NetworkType
					});
					const amount = bytesToNumberBE(output.tokenAmount);

					inputs.push({
						asset,
						amount
					});
				}
			}

			if (inputs.length || outputs.length) {
				const id = bytesToHex(tokenTransactionHash);
				const createdAt = tokenTransaction.clientCreatedTimestamp!;

				const transaction: Transaction = {
					id,
					inputs,
					outputs,
					createdAt
				};

				tokenTransactions.push(transaction);
			}
		}

		return tokenTransactions;
	}

	async getTransactions(): Promise<Transaction[]> {
		const [btcTransactions, tokenTransactions] = await Promise.all([
			this.getBitcoinTransactions(),
			this.getTokenTransactions()
		]);

		return [...btcTransactions, ...tokenTransactions].sort((a, b) => b.createdAt.getTime() - a.createdAt.getTime());
	}

	async getTransaction(txid: string): Promise<Transaction | undefined> {
		const type = identifyTransactionId(txid);

		switch (type) {
			case "BTC": {
				const tx = await this.sparkWallet.getTransfer(txid);
				if (!tx) return undefined;

				const id = tx.id;
				const inputs: TransactionInput[] = tx.valueSentByWallet
					? [{ asset: BTC_ASSET_ID, amount: BigInt(tx.valueSentByWallet) }]
					: [];
				const outputs: TransactionOutput[] = tx.valueReceivedByWallet
					? [{ asset: BTC_ASSET_ID, amount: BigInt(tx.valueReceivedByWallet) }]
					: [];
				const createdAt = tx.createdTime!;

				return {
					id,
					inputs,
					outputs,
					createdAt
				};
			}
			case "ASSET": {
				const [{ tokenTransactionsWithStatus }, walletIdentityPublicKey] = await Promise.all([
					this.sparkWallet.queryTokenTransactionsByTxHashes([txid]),
					this.sparkWallet.getIdentityPublicKey()
				]);

				const transaction = tokenTransactionsWithStatus.pop();
				const tokenTransaction = transaction?.tokenTransaction;

				if (!tokenTransaction) {
					return undefined;
				}

				const inputs = [];
				const outputs = [];

				for (const output of tokenTransaction.tokenOutputs) {
					const ownerPublicKey = bytesToHex(output.ownerPublicKey);

					if (ownerPublicKey !== walletIdentityPublicKey || !output.tokenIdentifier) {
						continue;
					}

					const asset = encodeBech32mTokenIdentifier({
						tokenIdentifier: output.tokenIdentifier,
						network: PUBLIC_NETWORK as NetworkType
					});
					const amount = bytesToNumberBE(output.tokenAmount);

					outputs.push({
						asset,
						amount
					});
				}

				if (tokenTransaction.tokenInputs?.$case === "transferInput") {
					const outputsToSpend = tokenTransaction.tokenInputs.transferInput.outputsToSpend;
					const transactionsByHash = new Map<string, TokenTransaction | undefined>();

					for (const input of outputsToSpend) {
						transactionsByHash.set(bytesToHex(input.prevTokenTransactionHash), undefined);
					}

					const previous = await this.sparkWallet.queryTokenTransactionsByTxHashes([...transactionsByHash.keys()]);
					for (const { tokenTransactionHash, tokenTransaction } of previous.tokenTransactionsWithStatus) {
						transactionsByHash.set(bytesToHex(tokenTransactionHash), tokenTransaction);
					}

					for (const input of outputsToSpend) {
						const previousTx = transactionsByHash.get(bytesToHex(input.prevTokenTransactionHash));
						const output = previousTx!.tokenOutputs[input.prevTokenTransactionVout];
						const ownerPublicKey = bytesToHex(output.ownerPublicKey);

						if (ownerPublicKey !== walletIdentityPublicKey || !output.tokenIdentifier) {
							continue;
						}

						const asset = encodeBech32mTokenIdentifier({
							tokenIdentifier: output.tokenIdentifier,
							network: PUBLIC_NETWORK as NetworkType
						});
						const amount = bytesToNumberBE(output.tokenAmount);

						inputs.push({
							asset,
							amount
						});
					}
				}

				if (!inputs.length && !outputs.length) {
					return undefined;
				}

				const id = bytesToHex(transaction.tokenTransactionHash);
				const createdAt = tokenTransaction.clientCreatedTimestamp!;

				return {
					id,
					inputs,
					outputs,
					createdAt
				};
			}
		}
	}

	async createLightningInvoice(amountSats: number, memo?: string): Promise<string> {
		const request = await this.sparkWallet.createLightningInvoice({ amountSats, memo });

		return request.invoice.encodedInvoice;
	}

	async getStaticDepositAddress(): Promise<string> {
		return await this.sparkWallet.getStaticDepositAddress();
	}

	async getSparkAddress(): Promise<string> {
		return await this.sparkWallet.getSparkAddress();
	}

	async getDepositPixLimit() {
		this.apiToken = this.apiToken ?? (await this.authWithMull());

		return await getDepositLimit(this.apiToken);
	}

	async createPixDeposit(payoutAmountInCents: bigint) {
		this.apiToken = this.apiToken ?? (await this.authWithMull());
		const address = await this.getSparkAddress();

		const request = await createDeposit(this.apiToken, { payout_amount_in_cents: Number(payoutAmountInCents), address });

		return request.br_code;
	}

	async authWithMull() {
		const { privateKey, publicKey } = deriveKeys(this.mnemonic);
		const request = await requestChallenge(bytesToHex(publicKey));
		const hash = sha256(stringToUint8Array(request.token));
		const signature = sign(privateKey, hash);
		const confirm = await confirmChallenge(request.token, bytesToHex(signature));

		return confirm.token;
	}

	async claimDeposits(maxFee: number = 1000) {
		const addresses = await this.sparkWallet.queryStaticDepositAddresses();

		for (const address of addresses) {
			const utxos = await this.sparkWallet.getUtxosForDepositAddress(address, 256, 0, true);

			for (const utxo of utxos) {
				await this.sparkWallet.claimStaticDepositWithMaxFee({
					transactionId: utxo.txid,
					outputIndex: utxo.vout,
					maxFee
				});
			}
		}
	}

	async getBalance(): Promise<SvelteMap<AssetId, Balance>> {
		const map = new SvelteMap<AssetId, Balance>();
		const response = await this.sparkWallet.getBalance();

		for (const [token, value] of response.tokenBalances) {
			map.set(token, {
				ownedBalance: value.ownedBalance,
				availableToSendBalance: value.availableToSendBalance
			});
		}

		map.set(BTC_ASSET_ID, {
			ownedBalance: response.satsBalance.owned,
			availableToSendBalance: response.satsBalance.available
		});

		return map;
	}

	async listPools(query?: ListPoolsQuery) {
		const assetAAddress = query?.assetA ? tokenIdentifier(query.assetA) : undefined;
		const assetBAddress = query?.assetB ? tokenIdentifier(query.assetB) : undefined;

		return await this.flashnet.listPools({ ...query, assetAAddress, assetBAddress });
	}

	async simulateSwap(poolId: string, assetIn: AssetId, assetOut: AssetId, amountIn: bigint): Promise<SimulateSwapResponse> {
		const assetInAddress = tokenIdentifier(assetIn);
		const assetOutAddress = tokenIdentifier(assetOut);

		const response = await this.flashnet.simulateSwap({
			poolId,
			assetInAddress,
			assetOutAddress,
			amountIn: String(amountIn)
		});

		return {
			...response,
			amountOut: BigInt(response.amountOut),
			feePaidAssetIn: response.feePaidAssetIn ? BigInt(response.feePaidAssetIn) : undefined
		};
	}

	async swap(
		poolId: string,
		assetIn: AssetId,
		assetOut: AssetId,
		amountIn: bigint,
		minAmountOut: bigint,
		maxSlippageBps: number = 100
	) {
		const assetInAddress = tokenIdentifier(assetIn);
		const assetOutAddress = tokenIdentifier(assetOut);

		return await this.flashnet.executeSwap({
			poolId,
			assetInAddress,
			assetOutAddress,
			amountIn: String(amountIn),
			minAmountOut: String(minAmountOut),
			maxSlippageBps
		});
	}
}

let cache: Wallet | undefined;

function tokenIdentifier(asset: AssetId) {
	if (asset == BTC_ASSET_ID) {
		return asset;
	}

	return decodeSparkHumanReadableTokenIdentifier(asset, PUBLIC_NETWORK as NetworkType).tokenIdentifier.toLowerCase();
}
