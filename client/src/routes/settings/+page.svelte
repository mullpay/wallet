<script lang="ts">
	import { goto } from "$app/navigation";
	import { logout } from "$lib/auth";
	import Loading from "$lib/components/loading.svelte";
	import { error } from "$lib/log";
	import { Wallet } from "$lib/wallet";
	import { faArrowLeft } from "@fortawesome/free-solid-svg-icons";
	import { onMount } from "svelte";
	import Fa from "svelte-fa";

	let wallet = $state<Wallet | undefined>();
	let isLoading = $state(true);

	function onRemoveWallet() {
		logout();
		goto("/login");
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
				<h1 class="w-full text-center text-xl font-semibold">Settings</h1>
			</div>
			<button
				class="cursor-pointer rounded-lg border border-red-300 px-4 py-3 text-sm font-medium text-red-600 hover:bg-red-50 dark:border-red-900 dark:text-red-400 dark:hover:bg-red-950"
				onclick={onRemoveWallet}
			>
				Remove Wallet
			</button>
		</div>
	{/if}
</div>
