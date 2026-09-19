<script lang="ts">
	import { onMount } from "svelte";
	import { goto } from "$app/navigation";
	import Fa from "svelte-fa";
	import { faAnglesDown, faAnglesUp, faArrowRightArrowLeft, faCog } from "@fortawesome/free-solid-svg-icons";
	import Loading from "$lib/components/loading.svelte";
	import { ASSETS, formatAmount, type AssetId } from "$lib/assets";
	import { Wallet, type Balance, type Transaction } from "$lib/wallet";
	import type { SvelteMap } from "svelte/reactivity";
	import { error } from "$lib/log";

	let isLoading = $state(true);
	let wallet = $state<Wallet | undefined>();
	let balance = $state<SvelteMap<AssetId, Balance> | undefined>();
	let transactions = $state<Transaction[]>([]);

	function formatDate(date: Date): string {
		return date.toLocaleString("pt-BR", {
			day: "2-digit",
			month: "2-digit",
			year: "numeric",
			hour: "2-digit",
			minute: "2-digit"
		});
	}

	onMount(async () => {
		try {
			wallet = await Wallet.initialize();

			if (!wallet) {
				goto("/login");
				return;
			}

			balance = await wallet.getBalance();
			transactions = await wallet.getTransactions();
		} catch (err) {
			error(err);
		} finally {
			isLoading = false;
		}
	});
</script>

{#snippet home()}
	<div class="flex w-full items-center justify-between">
		<h1 class="text-xl font-semibold">Mullpay Wallet</h1>
		<a
			href="/settings"
			class="flex items-center gap-1 text-xs text-neutral-400 hover:text-neutral-600 dark:text-neutral-500 dark:hover:text-neutral-300"
		>
			<Fa icon={faCog} />
			<p>Settings</p>
		</a>
	</div>

	<div class="grid w-full grid-cols-2 gap-3">
		{#each Object.entries(ASSETS) as [id, asset]}
			{@const assetId = id as AssetId}
			{@const amount = balance?.get(assetId)?.ownedBalance ?? 0n}
			<div
				class="flex w-full flex-col justify-center gap-3 rounded-lg border border-neutral-200 p-4 dark:border-neutral-800"
			>
				<div class="flex items-center gap-3">
					<img alt="asset" class="h-8 w-8" src={asset.icon} />

					<p class="text-sm font-medium" style="color: {asset.color}">{asset.name}</p>
				</div>
				<p class="text-xl font-semibold">{formatAmount(amount, assetId)}</p>
			</div>
		{/each}
	</div>

	<div class="flex w-full gap-2">
		<a
			href="/receive"
			class="flex flex-1 items-center justify-center gap-1 rounded-lg border border-neutral-300 px-2 py-2 text-center text-xs font-medium whitespace-nowrap shadow-sm transition hover:bg-neutral-50 active:shadow-none sm:gap-1.5 sm:px-4 sm:text-sm dark:border-neutral-700 dark:hover:bg-neutral-900"
		>
			<Fa icon={faAnglesDown} />
			<p class="text-xs">Receive</p>
		</a>
		<a
			href="/swap"
			class="flex flex-1 items-center justify-center gap-1 rounded-lg border border-neutral-300 px-2 py-2 text-center text-xs font-medium whitespace-nowrap shadow-sm transition hover:bg-neutral-50 active:shadow-none sm:gap-1.5 sm:px-4 sm:text-sm dark:border-neutral-700 dark:hover:bg-neutral-900"
		>
			<Fa icon={faArrowRightArrowLeft} />
			<p class="text-xs">Swap</p>
		</a>
		<a
			href="/send"
			class="flex flex-1 items-center justify-center gap-1 rounded-lg border border-neutral-300 px-2 py-2 text-center text-xs font-medium whitespace-nowrap shadow-sm transition hover:bg-neutral-50 active:shadow-none sm:gap-1.5 sm:px-4 sm:text-sm dark:border-neutral-700 dark:hover:bg-neutral-900"
		>
			<Fa icon={faAnglesUp} />
			<p class="text-xs">Send</p>
		</a>
	</div>
	<div
		class="flex h-full w-full flex-col gap-2 overflow-auto rounded-lg border border-neutral-200 p-2 dark:border-neutral-700"
	>
		{#if transactions.length > 0}
			{#each transactions as transaction}
				{@const asset = ASSETS[transaction.asset]}
				<div class="flex w-full flex-col gap-2 rounded-md border border-neutral-200 p-2 dark:border-neutral-700">
					<div class="flex items-center gap-2">
						<div class="flex items-center gap-1">
							<img alt="asset" class="h-8 w-8" src={asset.icon} />
							<p>{asset.name}</p>
						</div>
						{#if transaction.direction == "OUTGOING"}
							<p class="font-mono text-red-500">
								-{formatAmount(transaction.amount, transaction.asset)}
							</p>
						{:else}
							<p class="font-mono text-green-500">
								+{formatAmount(transaction.amount, transaction.asset)}
							</p>
						{/if}
					</div>
					<div>
						<p>{formatDate(transaction.createdAt)}</p>
					</div>
				</div>
			{/each}
		{:else}
			<div class="flex h-full w-full items-center justify-center">
				<p>No transactions yet</p>
			</div>
		{/if}
	</div>
{/snippet}

<div class="flex h-full w-full flex-col items-center gap-6">
	{#if isLoading}
		<div class="flex h-full w-full flex-col items-center justify-center">
			<Loading />
		</div>
	{:else}
		{@render home()}
	{/if}
</div>
