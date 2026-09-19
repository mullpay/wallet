<script lang="ts">
	import { goto } from "$app/navigation";
	import { login } from "$lib/auth";
	import { generateMnemonic, validateMnemonic } from "@scure/bip39";
	import { wordlist } from "@scure/bip39/wordlists/english.js";
	import InputWords from "$lib/components/inputWords.svelte";
	import { push } from "$lib/components/toast.svelte";

	const SIZE = 12;

	let words: string[] = $state([]);
	let scene: "none" | "create" | "import" = $state("none");

	function createWallet() {
		const mnemonic = generateMnemonic(wordlist);
		words = mnemonic.split(" ");
		scene = "create";
	}

	function importWallet() {
		scene = "import";
	}

	function submit() {
		if (!validateMnemonic(words.join(" "), wordlist)) {
			push("Invalid mnemonics");
			return;
		}
		login(words);
		goto("/");
	}
</script>

<div class="flex h-full w-full flex-col items-center justify-center gap-6">
	<h2 class="text-2xl font-bold">Mullpay Wallet</h2>
	{#if scene == "none"}
		<div class="flex w-full flex-col gap-3">
			<button
				class="w-full cursor-pointer rounded-lg bg-black px-4 py-3 font-medium text-white enabled:hover:bg-neutral-800 disabled:opacity-50 dark:bg-white dark:text-black dark:enabled:hover:bg-neutral-200"
				onclick={createWallet}
			>
				Create a new wallet
			</button>
			<button
				class="w-full cursor-pointer rounded-lg border border-neutral-300 px-4 py-3 font-medium enabled:hover:bg-neutral-50 dark:border-neutral-700 dark:enabled:hover:bg-neutral-900"
				onclick={importWallet}
			>
				Import an existing wallet
			</button>
		</div>
	{/if}
	{#if scene == "create"}
		<p class="text-center text-sm text-neutral-500 dark:text-neutral-400">
			Write down these 12 words in order and keep them somewhere safe. Anyone with this phrase can access your funds.
		</p>
		<InputWords {words} {wordlist} disabled={true} />
		<button
			class="w-full cursor-pointer rounded-lg bg-black px-4 py-3 font-medium text-white enabled:hover:bg-neutral-800 disabled:opacity-50 dark:bg-white dark:text-black dark:enabled:hover:bg-neutral-200"
			onclick={submit}
		>
			I've saved my recovery phrase
		</button>
	{/if}
	{#if scene == "import"}
		<p class="text-center text-sm text-neutral-500 dark:text-neutral-400">
			Please enter your recovery words to import your wallet.
		</p>
		<InputWords {words} {wordlist} disabled={false} />
		<button
			class="w-full cursor-pointer rounded-lg bg-black px-4 py-3 font-medium text-white enabled:hover:bg-neutral-800 disabled:opacity-50 dark:bg-white dark:text-black dark:enabled:hover:bg-neutral-200"
			onclick={submit}
			disabled={words.length !== SIZE}
		>
			Import wallet
		</button>
	{/if}
</div>
