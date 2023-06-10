<script lang="ts">
	import { writable } from 'svelte/store';
	import Debug from './Debug.svelte';
	import { incDepth, setParentDim, setRootDim, setSvgRoot } from './svg/Container.svelte';

	export let id: string | null = null;
	export let width = 0;
	export let height = 0;

	const controlled = !!width && !!height;

	const dim = writable({ width, height });
	$: dim.set({ width, height });

	setSvgRoot();
	setRootDim(dim);
	setParentDim(dim);
	incDepth();
</script>

{#if controlled}
	<div {id} class:controlled style="width: {width}px; height: {height}px">
		<svg {width} {height} viewBox="0 0 {width} {height}">
			<slot svgWidth={width} svgHeight={height} />
		</svg>
	</div>
{:else}
	<div {id} data-x-svg-root bind:clientWidth={width} bind:clientHeight={height}>
		{#if width && height}
			<svg {width} {height} viewBox="0 0 {width} {height}">
				<slot svgWidth={width} svgHeight={height} />
			</svg>
		{/if}
	</div>
{/if}

<Debug
	label="#{id}"
	values={[
		{ label: 'controlled', value: controlled },
		{ label: 'width', value: width },
		{ label: 'height', value: height }
	]}
/>

<style lang="postcss">
	div {
		position: relative;
		overflow: hidden;
		margin: 0;
		padding: 0;

		&:not(.controlled) {
			width: 100%;
			height: 100%;
		}
	}

	div > svg {
		position: absolute;

		width: 100%;
		height: 100%;
		margin: 0;
		padding: 0;
	}
</style>
