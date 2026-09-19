import data from "$lib/assets/assets.json";
import type { Bech32mTokenIdentifier } from "@buildonspark/spark-sdk";
import { BTC_ASSET_PUBKEY } from "@flashnet/sdk";

export interface Asset {
	name: string;
	color: string;
	decimal: number;
	icon: string;
}

export const BTC_ASSET_ID = BTC_ASSET_PUBKEY;
export const DEPIX_ASSET_ID = "btkn1sdqgp9904d620fmgs6p9669fghzfq06hvpqmy2wg8c9xtw4qms3qpf4aty";

export type BTCAssetId = typeof BTC_ASSET_ID;
export type AssetId = BTCAssetId | Bech32mTokenIdentifier;

export const ASSETS: Record<AssetId, Asset> = data;

export function formatAmount(amount: bigint, assetId: AssetId): string {
	const asset = ASSETS[assetId];
	const decimal = asset.decimal;
	const divisor = 10n ** BigInt(asset.decimal);

	const integer = amount / divisor;
	const fraction = amount % divisor;

	const fractionStr = fraction.toString().padStart(decimal, "0");

	return `${integer}.${fractionStr}`;
}

export function parseAmount(amount: string, assetId: AssetId): bigint {
	const asset = ASSETS[assetId];
	const decimal = asset.decimal;

	const [integerPart, fractionPart = ""] = amount.trim().split(".");

	const divisor = 10n ** BigInt(decimal);
	const integer = BigInt(integerPart) * divisor;
	const fraction = BigInt(fractionPart.padEnd(decimal, "0") || "0");

	return integer + fraction;
}
