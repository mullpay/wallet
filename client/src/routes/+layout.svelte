<script lang="ts">
	import "./layout.css";
	import favicon from "$lib/assets/favicon.svg";
	import Toast from "$lib/components/toast.svelte";

	let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<Toast />

<div
	class="flex h-screen w-screen items-center justify-center overflow-hidden bg-[url(/background.png)] bg-size-[500px_500px] bg-repeat"
>
	<div class="flex h-screen w-screen bg-white p-8 text-neutral-900 md:md:w-110 dark:bg-neutral-950 dark:text-neutral-100">
		{@render children()}
	</div>
</div>
