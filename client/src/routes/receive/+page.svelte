<script lang="ts">
	import { goto } from "$app/navigation";
	import Loading from "$lib/components/loading.svelte";
	import QrCode from "$lib/components/qrCode.svelte";
	import { push } from "$lib/components/toast.svelte";
	import { error } from "$lib/log";
	import { Wallet } from "$lib/wallet";
	import { faPix } from "@fortawesome/free-brands-svg-icons";
	import { faArrowLeft, faBolt, faBurst, faLink, type IconDefinition } from "@fortawesome/free-solid-svg-icons";
	import { onMount } from "svelte";
	import Fa from "svelte-fa";

	let wallet = $state<Wallet | undefined>();
	let isLoading = $state(true);
	let tab = $state<"SPARK" | "LIGHTNING" | "ON-CHAIN" | "PIX">("SPARK");

	let sparkAddress = $state("");
	let onChainAdress = $state("");
	let lightningAddress = $state("");
	let pixBRCode = $state("");

	let lightningAmount = $state("0");
	let lightningMemo = $state("");

	let pixAmount = $state(0);

	let pixMax = $state(8);
	let pixMin = $state(2);

	let address = $derived(
		tab == "SPARK" ? sparkAddress : tab == "ON-CHAIN" ? onChainAdress : tab == "LIGHTNING" ? lightningAddress : pixBRCode
	);

	async function lightningSubmit(event: SubmitEvent) {
		event.preventDefault();
		try {
			isLoading = true;

			const amount = Number(lightningAmount);
			const memo = lightningMemo || undefined;

			lightningAddress = await wallet!.createLightningInvoice(amount, memo);
		} catch (err) {
			error(err);
			push("Error generating invoice");
		} finally {
			isLoading = false;
		}
	}

	async function pixSubmit(event: SubmitEvent) {
		event.preventDefault();
		try {
			isLoading = true;

			const amount = BigInt(Math.round(Number(pixAmount) * 100));

			pixBRCode = await wallet!.createPixDeposit(amount);
		} catch (err) {
			error(err);
			push("Error generating pix");
			pixBRCode = "";
		} finally {
			isLoading = false;
		}
	}

	async function copyAddress() {
		try {
			await navigator.clipboard.writeText(address);
		} catch (err) {
			error(err);
			push("Error copying address");
		} finally {
			push("Address copied to clipboard");
		}
	}

	onMount(async () => {
		try {
			wallet = await Wallet.initialize();

			if (!wallet) {
				goto("/login");
				return;
			}

			onChainAdress = await wallet.getStaticDepositAddress();
			sparkAddress = await wallet.getSparkAddress();

			const limit = await wallet.getDepositPixLimit();

			pixMax = limit.max_payout_amount_in_cents / 100;
			pixMin = limit.min_payout_amount_in_cents / 100;
		} catch (err) {
			error(err);
		} finally {
			isLoading = false;
		}
	});
</script>

{#snippet tabItem(item: "SPARK" | "LIGHTNING" | "ON-CHAIN" | "PIX", text: string, icon: IconDefinition)}
	<button
		class="flex flex-1 cursor-pointer items-center justify-center gap-1.5 rounded-md py-2 {tab === item
			? 'bg-white font-medium shadow-sm dark:bg-neutral-700'
			: 'hover:bg-neutral-200 dark:hover:bg-neutral-800'}"
		onclick={() => {
			tab = item;
		}}
	>
		<Fa {icon}></Fa>
		<p>{text}</p>
	</button>
{/snippet}

<div class="flex h-full w-full flex-col items-center gap-6">
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
				<h1 class="w-full text-center text-xl font-semibold">Receive</h1>
			</div>
			<div class="flex gap-2 rounded-lg bg-neutral-100 p-1 text-sm dark:bg-neutral-900">
				{@render tabItem("SPARK", "Spark", faBurst)}
				{@render tabItem("LIGHTNING", "Lightning", faBolt)}
				{@render tabItem("ON-CHAIN", "On-chain", faLink)}
				{@render tabItem("PIX", "Pix", faPix)}
			</div>
			{#if address}
				<div class="flex rounded-xl bg-white p-2">
					<QrCode value={address} />
				</div>
				<p class="text-center font-mono text-xs break-all">{address}</p>
				{#if tab == "LIGHTNING"}
					<div class="flex flex-col gap-1.5">
						<button
							type="button"
							class="w-full cursor-pointer rounded-lg border border-neutral-300 px-4 py-2 text-sm font-medium hover:bg-neutral-50 dark:border-neutral-700 dark:hover:bg-neutral-900"
							onclick={copyAddress}
						>
							Copy invoice
						</button>
						<button
							type="button"
							class="w-full cursor-pointer rounded-lg border border-neutral-300 px-4 py-2 text-sm font-medium hover:bg-neutral-50 dark:border-neutral-700 dark:hover:bg-neutral-900"
							onclick={() => (lightningAddress = "")}
						>
							New invoice
						</button>
					</div>
				{:else if tab == "PIX"}
					<div class="flex flex-col gap-1.5">
						<button
							type="button"
							class="w-full cursor-pointer rounded-lg border border-neutral-300 px-4 py-2 text-sm font-medium hover:bg-neutral-50 dark:border-neutral-700 dark:hover:bg-neutral-900"
							onclick={copyAddress}
						>
							Copy brcode
						</button>
						<button
							type="button"
							class="w-full cursor-pointer rounded-lg border border-neutral-300 px-4 py-2 text-sm font-medium hover:bg-neutral-50 dark:border-neutral-700 dark:hover:bg-neutral-900"
							onclick={() => (pixBRCode = "")}
						>
							New pix
						</button>
					</div>
				{:else}
					<button
						type="button"
						class="w-full cursor-pointer rounded-lg border border-neutral-300 px-4 py-2 text-sm font-medium hover:bg-neutral-50 dark:border-neutral-700 dark:hover:bg-neutral-900"
						onclick={copyAddress}
					>
						Copy address
					</button>
				{/if}
			{/if}
			{#if tab == "LIGHTNING" && !address}
				<form class="flex w-full flex-col gap-2" onsubmit={lightningSubmit}>
					<div class="w-full gap-1">
						<label for="lightning-amount" class="flex flex-col gap-1 text-sm">Amount (sats)</label>
						<input
							id="lightning-amount"
							type="number"
							min="1"
							required
							class="no-spinner w-full rounded-lg border border-neutral-300 p-3 dark:border-neutral-700"
							bind:value={lightningAmount}
						/>
					</div>
					<div class="w-full gap-1">
						<label for="memo" class="flex flex-col gap-1 text-sm">Memo (optional)</label>
						<input
							id="memo"
							type="text"
							class="w-full rounded-lg border border-neutral-300 p-3 dark:border-neutral-700"
							bind:value={lightningMemo}
						/>
					</div>
					<button
						type="submit"
						class="w-full cursor-pointer rounded-lg bg-black px-4 py-3 font-medium text-white enabled:hover:bg-neutral-800 disabled:opacity-50 dark:bg-white dark:text-black dark:enabled:hover:bg-neutral-200"
					>
						Generate
					</button>
				</form>
			{/if}
			{#if tab == "PIX" && !address}
				<form class="flex w-full flex-col gap-2" onsubmit={pixSubmit}>
					<div class="w-full gap-1">
						<label for="pix-amount" class="flex flex-col gap-1 text-sm">Amount (real)</label>
						<input
							id="pix-amount"
							type="number"
							min={pixMin}
							max={pixMax}
							step="0.01"
							class="no-spinner w-full rounded-lg border border-neutral-300 p-3 dark:border-neutral-700"
							required
							bind:value={pixAmount}
						/>
					</div>
					<button
						type="submit"
						class="w-full cursor-pointer rounded-lg bg-black px-4 py-3 font-medium text-white enabled:hover:bg-neutral-800 disabled:opacity-50 dark:bg-white dark:text-black dark:enabled:hover:bg-neutral-200"
					>
						Generate
					</button>
				</form>
			{/if}
		</div>
	{/if}
</div>
