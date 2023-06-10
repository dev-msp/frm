<script lang="ts" context="module">
	export type Baseline =
		| 'auto'
		| 'text-bottom'
		| 'alphabetic'
		| 'ideographic'
		| 'middle'
		| 'central'
		| 'mathematical'
		| 'hanging'
		| 'text-top'
		| undefined;
</script>

<script lang="ts">
	import { offsetBoundingBox, type OffsetParams } from '$lib/utils/svg';
	import type { Vector, Rect } from '$lib/types/graphics';
	import Debug from '../Debug.svelte';
	import { add, subtract } from '$lib/graphics/math';

	const zero: Vector = { x: 0, y: 0 };

	// Computed
	export let center = false;

	// Passthrough
	export let x: number | string | null = null;
	export let y: number | string | null = null;
	export let transform: string | null = null;
	export let fill: string | null = null;
	export let fontSize: number | string = 'unset';
	export let fontFamily: string | null = null;

	export let baseline: Baseline = undefined;
	let nobo: Rect | undefined;
	let glyph: Rect | undefined;

	const params: OffsetParams = {
		center,
		onChange: ({ nodeRect, glyphRect }) => {
			nobo = nodeRect;
			glyph = glyphRect;
		}
	};

	let glyphNodeOffset: Vector;
	$: glyphNodeOffset = glyph && nobo ? subtract(nobo, glyph) : zero;

	$: pending = center && !nobo;

	let centerOffset = zero;
	$: glyphCenter = glyph ? { x: -glyph.width / 2, y: -glyph.height / 2 } : zero;
	$: centerOffset = add(glyphCenter, glyphNodeOffset);
</script>

<Debug enabled={false} label="text outlines">
	<g transform="translate({centerOffset.x}, {centerOffset.y})">
		<rect {...nobo} fill="none" stroke="red" />
		<rect {...glyph} fill="limegreen" />
	</g>
</Debug>

<g data-type="node" data-node-type="text" transform="translate({centerOffset.x}, {centerOffset.y})">
	<text
		{x}
		{y}
		{transform}
		dominant-baseline={baseline}
		use:offsetBoundingBox={params}
		opacity={pending ? 0 : 1}
		style:font-size={fontSize}
		style:font-family={fontFamily}
		{fill}
	>
		<slot />
	</text>
</g>

<style>
	text {
		stroke: none;
	}

	rect {
		opacity: 0.5;
		stroke-width: 0.5;
	}
</style>
