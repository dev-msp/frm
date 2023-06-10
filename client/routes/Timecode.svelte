<script lang="ts">
	import { formatTimecode } from '$lib/display';
	export let value: number;
	export let useHours = false;
</script>

<span>{formatTimecode(value, useHours)}</span>

<style lang="postcss">
	span {
		padding: 4px;

		font-family: 'Courier New', monospace;
		font-size: 11px;
		color: var(--fg);

		background-color: var(--bg);
		border: 0.3px solid var(--fg);
	}
</style>
