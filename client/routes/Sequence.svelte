<script lang="ts">
	import { alpha } from '$lib/command';
	import { index } from '$lib/state/seq';
	import Frame from './Frame.svelte';
	// import Timecode from './Timecode.svelte';

	export let data: [number, number][];
	export let dim: number;
	export let selection: string[];

	$: dataWithCursor = data.map(([a, b], i, arr) => [a, index(a, b, i, arr.length - 1), b]);
	// $: extent = data[data.length - 1]?.[1] - data[0]?.[0];
	// $: useHours = extent > 60 * 60 * 1000;

	let aspectRatio: number | undefined = undefined;
	$: arStyle = aspectRatio ? `aspect-ratio: ${aspectRatio}` : null;
	$: varStyle = `--ncols: ${dim}; --nrows: ${dim}`;
</script>

<div class="frames" style={[arStyle, varStyle].join('; ')}>
	{#each dataWithCursor as [start, cursor, end], i (start)}
		{@const c = alpha[i]}
		{@const isHighlighted = selection.includes(alpha[i])}
		<div class="frames-item">
			<Frame {start} {end} {cursor} pending={false} bind:aspectRatio>
				<span slot="label" class="frame-label" class:isHighlighted>
					{c}
				</span>
				<!-- <span slot="meta" class="frame-meta" let:cursor> -->
				<!-- 	<Timecode value={cursor} {useHours} /> -->
				<!-- </span> -->
			</Frame>
		</div>
	{/each}
</div>

<style lang="postcss">
	.frames {
		position: relative;

		overflow: hidden;
		display: grid;
		grid-template-columns: repeat(var(--ncols), minmax(0, 1fr));
		grid-template-rows: repeat(var(--nrows), minmax(0, 1fr));
		gap: 8px;

		width: 100%;
		max-height: 100%;
	}

	.frames-item {
		position: relative;
		overflow: hidden;
		min-height: 0;
	}

	span.frame-label.isHighlighted {
		opacity: 1;
		background-color: yellow;
	}

	span.frame-label {
		display: inline-flex;
		align-items: center;
		justify-content: center;

		width: 17px;
		height: 17px;

		font-family: 'Courier New', monospace;
		font-size: 12px;
		color: var(--fg);
		text-transform: uppercase;

		background-color: var(--bg);
	}
</style>
