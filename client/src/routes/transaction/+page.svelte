<script lang="ts">
	import { goto } from "$app/navigation";
	import { ASSETS, formatAmount } from "$lib/assets";
	import Loading from "$lib/components/loading.svelte";
	import { push } from "$lib/components/toast.svelte";
	import { error } from "$lib/log";
	import { Wallet, type Transaction, type TransactionInput, type TransactionOutput } from "$lib/wallet";
	import { faArrowLeft } from "@fortawesome/free-solid-svg-icons";
	import { onMount } from "svelte";
	import Fa from "svelte-fa";

	type Entry = TransactionInput | TransactionOutput;

	let wallet = $state<Wallet | undefined>();
	let transaction = $state<Transaction>();
	let isLoading = $state(true);

	async function copyTxId() {
		try {
			await navigator.clipboard.writeText(transaction?.id ?? "");
			push("Transaction ID copied to clipboard");
		} catch (err) {
			error(err);
			push("Error copying transaction ID");
		}
	}

	onMount(async () => {
		try {
			wallet = await Wallet.initialize();

			if (!wallet) {
				goto("/login");
				return;
			}

			const params = new URLSearchParams(window.location.search);
			const id = params.get("id");

			if (!id) {
				goto("/");
				return;
			}

			transaction = await wallet.getTransaction(id);

			if (!transaction) {
				push("Failed to load the transaction");
				goto("/");
				return;
			}
		} catch (err) {
			error(err);
		} finally {
			isLoading = false;
		}
	});
</script>

{#snippet entry(value: Entry, sign: string)}
	{@const asset = ASSETS[value.asset]}
	<dl class="flex flex-col gap-3 rounded-lg border border-neutral-200 p-3 dark:border-neutral-800">
		<div class="flex flex-col gap-1.5">
			<dt class="text-xs text-neutral-500 dark:text-neutral-400">Asset</dt>
			<dd class="flex items-center gap-3">
				<img src={asset.icon} alt={asset.name} class="h-8 w-8" />
				<span class="text-sm font-medium">{asset.name}</span>
			</dd>
		</div>
		<div class="flex flex-col gap-1.5">
			<dt class="text-xs text-neutral-500 dark:text-neutral-400">Amount</dt>
			<dd class="font-mono text-lg font-semibold break-all text-red-500">
				{sign}{formatAmount(value.amount, value.asset)}
			</dd>
		</div>
	</dl>
{/snippet}

<div class="flex h-full w-full flex-col items-center gap-6">
	{#if isLoading}
		<div class="flex h-full w-full flex-col items-center justify-center">
			<Loading />
		</div>
	{:else}
		<div class="flex h-full min-h-0 w-full flex-col gap-6 overflow-y-auto">
			<div class="relative flex items-center">
				<a
					class="absolute left-0 flex items-center gap-1 text-sm text-neutral-500 hover:text-neutral-700 dark:text-neutral-400 dark:hover:text-neutral-200"
					href="/"
				>
					<Fa icon={faArrowLeft}></Fa>
					<p>Back</p>
				</a>
				<h1 class="w-full text-center text-xl font-semibold">Transaction</h1>
			</div>

			{#if transaction}
				<dl class="flex flex-col gap-4 rounded-lg border border-neutral-200 p-4 dark:border-neutral-800">
					<div class="flex flex-col gap-1.5">
						<dt class="text-sm text-neutral-500 dark:text-neutral-400">Transaction ID</dt>
						<dd class="font-mono text-xs break-all select-all">{transaction.id}</dd>
					</div>
					<div class="flex flex-col gap-1.5">
						<dt class="text-sm text-neutral-500 dark:text-neutral-400">Date</dt>
						<dd class="text-sm">{transaction.createdAt?.toLocaleString("pt-BR") ?? "Unavailable"}</dd>
					</div>
				</dl>

				<section class="flex flex-col gap-3 overflow-auto">
					<h2 class="text-sm font-semibold">Balance changes</h2>
					<div class="flex flex-col gap-3 overflow-auto">
						{#each transaction.inputs as input}
							{@render entry(input, "-")}
						{/each}
						{#each transaction.outputs as output}
							{@render entry(output, "+")}
						{/each}
					</div>
				</section>

				<button
					type="button"
					class="w-full cursor-pointer rounded-lg border border-neutral-300 px-4 py-2 text-sm font-medium hover:bg-neutral-50 dark:border-neutral-700 dark:hover:bg-neutral-900"
					onclick={copyTxId}
				>
					Copy txid
				</button>
			{/if}
		</div>
	{/if}
</div>
