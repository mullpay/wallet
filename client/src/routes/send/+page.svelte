<script lang="ts">
	import { goto } from "$app/navigation";
	import Loading from "$lib/components/loading.svelte";
	import { push } from "$lib/components/toast.svelte";
	import { error } from "$lib/log";
	import { identifyAddress, type AddressType } from "$lib/validation";
	import { ExitSpeed, Wallet } from "$lib/wallet";
	import { faArrowLeft, type IconDefinition } from "@fortawesome/free-solid-svg-icons";
	import { onMount } from "svelte";
	import Fa from "svelte-fa";
	import { decode as decodeInvoice } from "light-bolt11-decoder";
	import { ASSETS, BTC_ASSET_ID, DEPIX_ASSET_ID, parseAmount, type AssetId } from "$lib/assets";
	import { faBtc, faPix } from "@fortawesome/free-brands-svg-icons";

	let wallet = $state<Wallet | undefined>();

	let isLoading = $state(true);
	let addressType = $state<AddressType | undefined>();

	let address = $state("");
	let asset = $state<AssetId>(BTC_ASSET_ID);
	let amount = $state("");

	const isZeroAmountInvoice = $derived.by(() => {
		try {
			return !decodeInvoice(address).sections.some((section) => section.name === "amount");
		} catch {
			return false;
		}
	});

	$effect(() => {
		addressType = identifyAddress(address);
	});

	async function submit(event: SubmitEvent) {
		try {
			event.preventDefault();
			isLoading = true;

			switch (addressType) {
				case "LIGHTNING": {
					await wallet!.payLightningInvoice(address, 100n, isZeroAmountInvoice ? BigInt(amount) : undefined);
					break;
				}
				case "SPARK": {
					const value = asset == BTC_ASSET_ID ? BigInt(amount) : parseAmount(amount, asset);
					await wallet!.transfer(address, value, asset);
					break;
				}
				case "ON-CHAIN": {
					await wallet!.withdraw(address, BigInt(amount), ExitSpeed.FAST);
					break;
				}
			}

			push("Transaction sent");
			goto("/");
		} catch (err) {
			error(err);
			push("Error sending transaction");
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
		} catch (err) {
			error(err);
		} finally {
			isLoading = false;
		}
	});
</script>

{#snippet tabItem(item: AssetId, text: string, icon: IconDefinition)}
	<button
		type="button"
		class="flex flex-1 cursor-pointer items-center justify-center gap-1.5 rounded-md py-2 {asset == item
			? 'bg-white font-medium shadow-sm dark:bg-neutral-700'
			: 'hover:bg-neutral-200 dark:hover:bg-neutral-800'}"
		onclick={() => {
			asset = item;
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
				<h1 class="w-full text-center text-xl font-semibold">Send</h1>
			</div>
			<form onsubmit={submit} class="flex flex-col gap-4">
				<div class="w-full gap-1">
					{#if !addressType}
						<label for="address" class="flex flex-col gap-1 text-sm">Address</label>
					{/if}
					{#if addressType == "SPARK"}
						<label for="address" class="flex flex-col gap-1 text-sm">Spark address</label>
					{/if}
					{#if addressType == "LIGHTNING"}
						<label for="address" class="flex flex-col gap-1 text-sm">Lightning invoice</label>
					{/if}
					{#if addressType == "ON-CHAIN"}
						<label for="address" class="flex flex-col gap-1 text-sm">Bitcoin address</label>
					{/if}
					<input
						id="address"
						type="text"
						class="w-full rounded-lg border border-neutral-300 p-3 dark:border-neutral-700"
						bind:value={address}
					/>
				</div>
				{#if addressType == "SPARK"}
					<div class="flex gap-2 rounded-lg bg-neutral-100 p-1 text-sm dark:bg-neutral-900">
						{@render tabItem(BTC_ASSET_ID, "Bitcoin", faBtc)}
						{@render tabItem(DEPIX_ASSET_ID, "DePix", faPix)}
					</div>
				{/if}
				<div class="w-full gap-1">
					{#if addressType != "LIGHTNING" || isZeroAmountInvoice}
						{#if asset == BTC_ASSET_ID}
							<label for="address" class="flex flex-col gap-1 text-sm">Amount (sats)</label>
							<input
								id="address"
								type="text"
								step="1"
								class="no-spinner w-full rounded-lg border border-neutral-300 p-3 dark:border-neutral-700"
								bind:value={amount}
							/>
						{:else}
							<label for="address" class="flex flex-col gap-1 text-sm">Amount ({ASSETS[asset].name.toLowerCase()})</label>
							<input
								id="address"
								type="text"
								step="0.00000001"
								class="no-spinner w-full rounded-lg border border-neutral-300 p-3 dark:border-neutral-700"
								bind:value={amount}
							/>
						{/if}
					{/if}
				</div>
				<button
					type="submit"
					class="w-full cursor-pointer rounded-lg bg-black px-4 py-3 font-medium text-white enabled:hover:bg-neutral-800 disabled:opacity-50 dark:bg-white dark:text-black dark:enabled:hover:bg-neutral-200"
					disabled={!((addressType && amount) || (!isZeroAmountInvoice && addressType == "LIGHTNING"))}
				>
					Send
				</button>
			</form>
		</div>
	{/if}
</div>
