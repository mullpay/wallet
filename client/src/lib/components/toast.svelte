<script lang="ts" module>
	import { faX } from "@fortawesome/free-solid-svg-icons";
	import Fa from "svelte-fa";
	import { SvelteMap } from "svelte/reactivity";
	import { fade } from "svelte/transition";

	interface Toast {
		duration: number;
		msg: string;
		onEnd?: () => void;
		timer: number;
	}

	interface Options {
		duration?: number;
		onEnd?: () => void;
	}

	let toasts = new SvelteMap<number, Toast>();
	let counter = $state(0);

	export function push(msg: string, options?: Options) {
		const id = counter++;
		const duration = options?.duration ?? 5000;

		const timer = window.setTimeout(() => {
			cancel(id);
		}, duration);

		toasts.set(id, { msg, duration, timer, onEnd: options?.onEnd });

		return id;
	}
	export function cancel(id: number) {
		return toasts.delete(id);
	}

	function toastWait(id: number) {
		const toast = toasts.get(id);
		if (!toast) return;

		window.clearTimeout(toast.timer);
	}
	function toastContinue(id: number) {
		const toast = toasts.get(id);
		if (!toast) return;

		const timer = window.setTimeout(() => {
			cancel(id);
		}, toast.duration);
		toast.timer = timer;
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="fixed top-0 right-0 z-50 flex flex-col gap-2 p-4">
	{#each toasts as [id, toast]}
		<div
			transition:fade
			onmouseenter={() => toastWait(id)}
			onmouseleave={() => toastContinue(id)}
			class="flex gap-2 rounded-sm border border-neutral-700 bg-neutral-950 p-2 text-white"
		>
			<p class="truncate">{toast.msg}</p>
			<button onclick={() => cancel(id)} class="flex h-6 w-6 items-center justify-center rounded-sm bg-neutral-800">
				<Fa icon={faX}></Fa></button
			>
		</div>
	{/each}
</div>
