<script lang="ts">
	import { clamp } from "$lib/utils";

	const {
		words = $bindable(),
		wordlist,
		disabled = false
	}: {
		words: string[];
		wordlist: string[];
		disabled?: boolean;
	} = $props();

	let input = $state("");
	let selected = $state(0);

	const suggestions = $derived(wordlist.filter((s) => s.startsWith(input) && !words.includes(s)));

	$effect(() => {
		selected = clamp(selected, 0, suggestions.length - 1);
	});

	function addSelected(index = selected) {
		const word = suggestions[index];
		if (!word || !input.trim()) return;

		input = "";
		selected = 0;

		words.push(word);
	}

	function handlePaste(event: ClipboardEvent) {
		event.preventDefault();

		const pasted = event.clipboardData?.getData("text/plain") ?? "";
		const parsed = pasted
			.trim()
			.toLowerCase()
			.replace(/\n|\r|\t|,/g, " ")
			.split(/\s+/)
			.map((word) => word.trim())
			.filter((word) => word.length > 0)
			.filter((word) => wordlist.includes(word));

		if (!parsed.length) return;

		words.splice(0, words.length, ...parsed);

		input = "";
		selected = 0;
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === "Enter" && suggestions.length > 0) {
			event.preventDefault();
			addSelected();
			return;
		}

		if (event.key === "ArrowDown") {
			event.preventDefault();
			selected = (selected + 1) % suggestions.length;
			return;
		}

		if (event.key === "ArrowUp") {
			event.preventDefault();
			selected = (selected - 1) % suggestions.length;
			return;
		}

		if (event.key === "Backspace" && input === "") {
			event.preventDefault();
			words.pop();
			return;
		}
	}
</script>

<div class="w-full rounded-xl bg-black text-white">
	<div class="flex flex-wrap items-center gap-2 rounded-xl border p-3 shadow-sm">
		{#each words as word}
			<span class="rounded-lg bg-gray-700 px-2 py-1 text-sm text-blue-100">
				{word}
			</span>
		{/each}
		{#if !disabled}
			<input
				bind:value={input}
				onkeydown={handleKeydown}
				onpaste={handlePaste}
				class="min-w-30 flex-1 p-1 outline-none"
				placeholder="Type the words..."
			/>
		{/if}
	</div>

	{#if suggestions.length && input}
		<div class="mt-2 max-h-40 overflow-auto rounded-xl border bg-zinc-900 shadow-sm">
			{#each suggestions as suggestion, i}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div
					class="cursor-pointer px-3 py-2 {i === selected ? 'bg-zinc-700' : 'hover:bg-zinc-800'}"
					onclick={() => addSelected(i)}
				>
					{suggestion}
				</div>
			{/each}
		</div>
	{/if}
</div>
