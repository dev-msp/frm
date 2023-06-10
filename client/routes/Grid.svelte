<script lang="ts">
	import type { Command } from '$lib/command';
	import { formatTimecode } from '$lib/display';
	import { highlightedChars, type State } from '$lib/state';
	import * as select from '$lib/state/select';
	import * as seq from '$lib/state/seq';
	import Sequence from './Sequence.svelte';
	import { data, store } from '$lib/keys';
	import Band from '$lib/components/Band.svelte';
	import type { Grid } from '$lib/state/grid';
	import { mapEpicState, type EpicTransform } from '$lib/state/stream';

	export let state: Grid;
	// let savedSeqs: [number, number][] = [];

	$: ({ from, to } = state.range);

	$: useHours = select.useHours(state.range);
	const epicTx: EpicTransform<Grid, Command, number[], State> = mapEpicState(
		(s: State): Grid | undefined => (s.view.type === 'grid' ? s.view : undefined)
	);
	const dataSource = store.runEpic(epicTx(data), []);
	$: latestData = $dataSource;
</script>

<!-- <div class="debug"> -->
<!-- 	{#each savedSeqs as [s0, e0] (`${s0}|${e0}`)} -->
<!-- 		{@const span = e0 - s0} -->
<!-- 		{@const [s, e] = [Math.floor(s0 + span * 0.05), Math.floor(e0 - span * 0.05)]} -->
<!-- 		<div class="row"> -->
<!-- 			<span>{formatTimecode(s)} to {formatTimecode(e)}</span> -->
<!-- 			<div class="saved"> -->
<!-- 				{#each seq.pairwise(seq.range(s, e, 4)) as [si, ei], i (si)} -->
<!-- 					<Frame pending={false} start={si} end={ei} cursor={seq.index(si, ei, i, 3)} /> -->
<!-- 				{/each} -->
<!-- 			</div> -->
<!-- 		</div> -->
<!-- 	{/each} -->
<!-- </div> -->

<div class="other-stuff">
	<h1>frm</h1>
	<span>
		{formatTimecode(from, useHours)} - {formatTimecode(to, useHours)}
	</span>
</div>

<div class="band">
	<Band points={latestData} min={0} max={state.range.totalDuration} />
</div>
<div class="results">
	<Sequence
		data={seq.pairwise(latestData)}
		dim={state.dim}
		selection={highlightedChars(state.selection)}
	/>
</div>

<!-- .debug { -->
<!--   position: fixed; -->
<!--   z-index: 99; -->
<!--   right: 0; -->
<!--   bottom: 0; -->
<!---->
<!--   width: 50%; -->
<!--   padding: 10px; -->
<!--   padding: 1em; -->
<!---->
<!--   font-size: 14px; -->
<!-- } -->
<!-- .row { -->
<!--   display: grid; -->
<!--   grid-template-columns: max-content 1fr; -->
<!--   gap: 1rem; -->
<!--   align-items: center; -->
<!-- } -->
<!---->
<!-- .saved { -->
<!--   display: flex; -->
<!--   flex-direction: row; -->
<!-- } -->
<style lang="postcss">
	.results {
		overflow: hidden;
		justify-self: flex-end;
	}

	.other-stuff {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
	}

	.other-stuff span {
		font-size: 32px;
	}
</style>
