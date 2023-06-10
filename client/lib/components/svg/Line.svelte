<script lang="ts">
	import Point, { type LabelSlot } from './Point.svelte';
	import type { Vector } from '$lib/types/graphics';
	import type { SvgTheme, ThemeRefs } from '$lib/types/theme';
	import type { Contains } from '$lib/types/template';

	interface $$Slots {
		endpoint: LabelSlot;
	}

	type LabelledVector = Vector & { label?: string };
	type ThemeVarRef<K extends string> = Contains<ThemeRefs<SvgTheme>, K>;

	export let a: LabelledVector;
	export let b: LabelledVector;
	export let endpoints = false;

	export let r = 1;
	export let color: string | null = 'var(--svg-stroke-color)';
	export let strokeWidth: number | ThemeVarRef<'width'> = 'var(--svg-stroke-width)';
	export let lineCap: 'butt' | 'round' | 'square' | ThemeVarRef<'linecap'> =
		'var(--svg-stroke-linecap)';

	export let style: 'solid' | 'dashed' | 'unset' = 'unset';
	$: dashArray = { solid: null, unset: 'unset', dashed: 'var(--svg-stroke-dashFormat)' }[style];
</script>

<g data-type="node" data-node-type="line">
	<line
		x1={a.x}
		y1={a.y}
		x2={b.x}
		y2={b.y}
		style:stroke={color}
		style:stroke-width={strokeWidth}
		style:stroke-linecap={lineCap}
		style:stroke-dasharray={dashArray}
	/>

	{#if endpoints}
		<Point label={a.label} coord={a} {r} fill={color}>
			<slot name="endpoint" label={a.label} />
		</Point>
		<Point label={b.label} coord={b} {r} fill={color}>
			<slot name="endpoint" label={b.label} />
		</Point>
	{/if}
</g>
