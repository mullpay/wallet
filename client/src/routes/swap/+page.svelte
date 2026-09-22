<script lang="ts">
	import { goto } from "$app/navigation";
	import { ASSETS, BTC_ASSET_ID, DEPIX_ASSET_ID, formatAmount, parseAmount, type AssetId } from "$lib/assets";
	import Loading from "$lib/components/loading.svelte";
	import { push } from "$lib/components/toast.svelte";
	import { error } from "$lib/log";
	import { Wallet } from "$lib/wallet";
	import { faArrowLeft, faLeftRight } from "@fortawesome/free-solid-svg-icons";
	import { onMount } from "svelte";
	import Fa from "svelte-fa";

	let wallet = $state<Wallet | undefined>();
	let isLoading = $state(true);
	let assetPay = $state<AssetId>(DEPIX_ASSET_ID);
	let assetReceive = $state<AssetId>(BTC_ASSET_ID);
	let selectedPoolId = $state("");
	let payAmount = $state("");
	let receiveAmount = $state("");

	async function refreshPool() {
		try {
			const response = await wallet!.listPools({
				assetA: assetPay,
				assetB: assetReceive,
				limit: 10,
				sort: "TVL_DESC"
			});

			const pool = response.pools.pop();

			selectedPoolId = pool?.lpPublicKey ?? "";
		} catch (err) {
			error(err);
			push("Error loading swap pool");
		}
	}

	async function refreshQuote() {
		try {
			const amountIn = parseAmount(payAmount, assetPay);
			const quote = await wallet!.simulateSwap(selectedPoolId, assetPay, assetReceive, amountIn);
			receiveAmount = formatAmount(BigInt(quote.amountOut), assetReceive);
		} catch (err) {
			error(err);
			receiveAmount = "";
		}
	}

	async function flipDirection() {
		[assetPay, assetReceive] = [assetReceive, assetPay];
		[receiveAmount, payAmount] = [payAmount, receiveAmount];

		await refreshPool();
		await refreshQuote();
	}

	async function submit(event: SubmitEvent) {
		try {
			event.preventDefault();
			isLoading = true;

			const amountIn = parseAmount(payAmount, assetPay);
			const quote = await wallet!.simulateSwap(selectedPoolId, assetPay, assetReceive, amountIn);

			await wallet!.swap(selectedPoolId, assetPay, assetReceive, amountIn, quote.amountOut);

			push("Swap executed");
			goto("/");
		} catch (err) {
			error(err);
			push("Error executing swap");
		} finally {
			isLoading = false;
		}
	}

	onMount(async () => {
		try {
			wallet = await Wallet.initialize();

			if (!wallet) {
				goto("/login");
				return;
			}

			await refreshPool();
		} catch (err) {
			error(err);
		} finally {
			isLoading = false;
		}
	});
</script>

<div class="flex h-full w-full flex-col items-center">
	{#if isLoading}
		<div class="flex h-full w-full flex-col items-center justify-center">
			<Loading />
		</div>
	{:else}
		<div class="flex h-full w-full flex-col gap-6">
			<div class="relative flex items-center">
				<a
					class="absolute left-0 flex items-center gap-1 text-sm text-neutral-500 hover:text-neutral-700 dark:text-neutral-400 dark:hover:text-neutral-200"
					href="/"
				>
					<Fa icon={faArrowLeft}></Fa>
					<p>Back</p>
				</a>
				<h1 class="w-full text-center text-xl font-semibold">Swap</h1>
			</div>
			<form onsubmit={submit} class="flex w-full flex-col gap-6">
				<div class="relative flex w-full flex-col items-center justify-center gap-1">
					<div class="w-full rounded-xl border border-neutral-200 p-4 dark:border-neutral-800">
						<span class="text-sm text-neutral-500 dark:text-neutral-400">You pay</span>
						<div class="mt-2 flex items-center justify-between gap-2">
							<input
								type="text"
								placeholder="0"
								bind:value={payAmount}
								readonly={false}
								oninput={() => refreshQuote()}
								class="w-full text-2xl font-semibold outline-none"
							/>
							<span class="shrink-0 text-sm font-medium" style="color: {ASSETS[assetPay].color};">
								{ASSETS[assetPay].name}
							</span>
						</div>
					</div>
					<div class="w-full rounded-xl border border-neutral-200 p-4 dark:border-neutral-800">
						<span class="text-sm text-neutral-500 dark:text-neutral-400">You receive</span>
						<div class="mt-2 flex items-center justify-between gap-2">
							<input
								type="text"
								placeholder="0"
								bind:value={receiveAmount}
								readonly={true}
								class="w-full text-2xl font-semibold outline-none"
							/>
							<span class="shrink-0 text-sm font-medium" style="color: {ASSETS[assetReceive].color};">
								{ASSETS[assetReceive].name}
							</span>
						</div>
					</div>
					<button
						type="button"
						class="absolute top-1/2 left-1/2 flex h-9 w-9 -translate-x-1/2 -translate-y-1/2 cursor-pointer items-center justify-center rounded-full border border-neutral-300 bg-white text-neutral-600 shadow-sm hover:bg-neutral-50 dark:border-neutral-700 dark:bg-neutral-800 dark:text-neutral-300 dark:hover:bg-neutral-700"
						onclick={flipDirection}
					>
						<Fa icon={faLeftRight} rotate={90}></Fa>
					</button>
				</div>
				<button
					type="submit"
					class="cursor-pointer rounded-lg bg-black px-4 py-3 font-medium text-white enabled:hover:bg-neutral-800 disabled:opacity-50 dark:bg-white dark:text-black dark:enabled:hover:bg-neutral-200"
					disabled={!receiveAmount}
				>
					Swap
				</button>
			</form>
		</div>
	{/if}
</div>
