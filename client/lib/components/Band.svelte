<script lang="ts">
	import Container from './svg/Container.svelte';
	import G from './svg/G.svelte';
	import Line from './svg/Line.svelte';

	export let points: number[];
	export let min: number;
	export let max: number;
</script>

<div class="band">
	<Container>
		<G let:width let:height>
			{@const mid = height / 2}
			<Line a={{ x: 0, y: mid }} b={{ x: width, y: mid }} strokeWidth={2} />

			{#each points as point, i (i)}
				{@const actual = (point - min) / (max - min)}
				{@const coord = 10 + actual * (width - 20)}
				{@const [y1, y2] = [height * 0.3, height * 0.7]}
				<Line a={{ x: coord, y: y1 }} b={{ x: coord, y: y2 }} strokeWidth={1} />
			{/each}
		</G>
	</Container>
</div>

<style lang="postcss">
	.band {
		width: 100%;
		height: 40px;
	}
</style>
