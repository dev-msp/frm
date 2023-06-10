<script lang="ts">
	import { getDebug } from '$lib/utils';

	const debug = getDebug();
	export let enabled = true;
	export let label: string;
	export let values: { label: string; value: unknown }[] = [];
</script>

{#if debug && enabled}
	<slot {label} {values}>
		<div class="root">
			<h3>{label}</h3>
			<div class="values">
				{#each values as { label, value }}
					<slot name="item" {label} {value}>
						<div data-debug>
							{label}:
							<span class="thing">{value}</span>
						</div>
					</slot>
				{/each}
			</div>
		</div>
	</slot>
{/if}

<style lang="postcss">
	h3 {
		font-family: monospace;
	}

	.root {
		--size: calc(max(9pt, min(1rem, 14pt)));

		position: fixed;
		right: var(--size);
		bottom: var(--size);

		display: flex;
		flex-direction: column;

		font-size: var(--size);
	}
</style>
