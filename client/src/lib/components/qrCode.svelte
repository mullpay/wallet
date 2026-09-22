<script lang="ts">
	import QRCode from "qrcode";

	type Args = { value: string; width?: number; margin?: number };

	let { value, width = 256, margin = 0 }: Args = $props();

	let url = $state("");

	async function generate() {
		url = await QRCode.toDataURL(value, { margin, width });
	}

	$effect(() => {
		if (!value) {
			url = "";
			return;
		}

		generate();
	});
</script>

{#if url}
	<img src={url} alt="QR code" class="flex aspect-square w-full" />
{/if}
