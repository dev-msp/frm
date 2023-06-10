<script lang="ts" context="module">
	import { getContext, setContext } from 'svelte';
	import { readable, type Readable } from 'svelte/store';
	type DimStore = Readable<{ width: number; height: number } | undefined>;

	const emptyDimStore: DimStore = readable({ width: 0, height: 0 });

	const CONTAINER_DEPTH = Symbol();
	export const getDepth = () => getContext<number>(CONTAINER_DEPTH) ?? -1;
	export const incDepth = () => {
		return setContext<number>(CONTAINER_DEPTH, getDepth() + 1);
	};

	const PARENT_DIM = Symbol();
	export const getParentDim = () => {
		return getContext<DimStore>(PARENT_DIM) ?? emptyDimStore;
	};
	export const setParentDim = (dim: DimStore) => setContext<DimStore>(PARENT_DIM, dim);

	const ROOT_DIM = Symbol();
	export const getRootDim = () => {
		return getContext<DimStore>(ROOT_DIM);
	};
	export const setRootDim = (dim: DimStore) => {
		setParentDim(dim);
		setContext<DimStore>(ROOT_DIM, dim);
	};

	const SVG_ROOT = Symbol();
	export const setSvgRoot = () => setContext<boolean>(SVG_ROOT, true);
	export const hasSvgRoot = () => getContext<boolean>(SVG_ROOT);
</script>

<script lang="ts">
	import Svg from '../Svg.svelte';
	import G from './G.svelte';

	interface $$Slots {
		default: {
			width: number;
			height: number;
		};
	}

	export let id: string | null = null;
	export let className: string | null = null;

	// layout/origin props
	export let center = false;
	export let centerOrigin = false;
	export let xOffset = 0;
	export let yOffset = 0;
	export let rotation = 0;

	// desired width and height
	export let width: number | null = null;
	export let height: number | null = null;

	const hasRoot = hasSvgRoot();
</script>

{#if hasRoot}
	<G
		{id}
		{className}
		{centerOrigin}
		{center}
		{xOffset}
		{yOffset}
		width={width ?? 0}
		height={height ?? 0}
		{rotation}
		let:width={selfWidth}
		let:height={selfHeight}
	>
		<slot width={selfWidth} height={selfHeight} />
	</G>
{:else}
	<Svg id={id && `svg_root_${id}`} width={width ?? 0} height={height ?? 0}>
		<G
			{id}
			{className}
			{centerOrigin}
			{center}
			{xOffset}
			{yOffset}
			{rotation}
			let:width={selfWidth}
			let:height={selfHeight}
		>
			<slot width={selfWidth} height={selfHeight} />
		</G>
	</Svg>
{/if}
