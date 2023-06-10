<script lang="ts">
	import _ from 'lodash';
	import { imageCache } from '$lib/state/image-cache';

	export let pending: boolean;

	export let start: number;
	export let cursor: number;
	export let end: number;

	export let aspectRatio: number | undefined = undefined;

	$: source = imageCache.get$(cursor);
	$: image = $source;

	$: arStyle = aspectRatio ? `aspect-ratio: ${aspectRatio}` : null;
</script>

<div class="frame" class:loading={!image} class:pending style={arStyle}>
	{#if $$slots.label}
		<div class="label">
			<slot name="label" />
		</div>
	{/if}
	{#if $$slots.meta}
		<div class="meta">
			<slot name="meta" {start} {end} {cursor} />
		</div>
	{/if}
	{#key image}
		{#if _.isNumber(image)}
			<img
				style={`--opacity: ${Math.min(1, 1.4 - Math.abs(cursor - image) / 10e3)}`}
				class:deemph={cursor !== image}
				on:load={({ currentTarget }) => {
					if (!aspectRatio) {
						aspectRatio = (currentTarget.clientWidth ?? 16) / (currentTarget.clientHeight ?? 9);
					}
				}}
				src={`/api/frame/${image}`}
				alt={cursor.toLocaleString()}
			/>
		{/if}
	{/key}
</div>

<style lang="postcss">
	img {
		transition: opacity;
	}

	.deemph {
		opacity: var(--opacity);
	}

	.loading {
		visibility: hidden;
		opacity: 0;
	}

	.frame:not(.loading) {
		opacity: 1;
	}

	.frame {
		position: relative;
		box-sizing: border-box;
	}

	.label,
	.meta {
		position: absolute;
		z-index: 99;
	}

	.label {
		top: 8px;
		right: 8px;
		margin: 0;
		padding: 0;
	}

	.meta {
		top: 8px;
		left: 8px;
		margin: 0;
		padding: 0;
	}

	.frame img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		object-position: center;
	}

	.pending {
		border: 1px solid yellow;
	}
</style>
