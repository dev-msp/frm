<script lang="ts">
	import Band from '$lib/components/Band.svelte';
	import { browserOnly } from '$lib/stream';
	import { sortBy } from 'lodash';
	import { defer, filter, fromEvent, map, tap } from 'rxjs';
	import { onDestroy } from 'svelte';
	import Frame from '../Frame.svelte';
	import { formatTimecode } from '$lib/display';
	import type { Bisect } from '$lib/state/bisect';

	const mappedKeys = ['r', 'h', 'l', 'n'] as const;
	type Key = (typeof mappedKeys)[number];
	const isKey = (key: string): key is Key =>
		mappedKeys.includes(key as (typeof mappedKeys)[number]);

	const clamp = (n: number, lo: number, hi: number) => Math.max(lo, Math.min(n, hi));

	type Pair = [number, number];

	export let state: Bisect;

	$: ({ from, to, totalDuration } = state.current);

	$: bounds = [clamp(from, 1e3, Infinity), to] as Pair;

	const set = ([s, e]: Pair) => {
		bounds = [clamp(s, 1e3, totalDuration), clamp(e, 1e3, totalDuration)];
	};

	$: midpoint = Math.floor(bounds[0] + (bounds[1] - bounds[0]) / 2);

	const handleInput = (key: Key) => {
		const [a, b] = sortBy(bounds);
		const currentMidpoint = a + (b - a) / 2;
		if (key === 'r') {
			set([from, to]);
		} else if (key === 'h') {
			set([bounds[0], currentMidpoint]);
		} else if (key === 'l') {
			// forward
			set([currentMidpoint, bounds[1]]);
		} else if (key === 'n') {
			const delta = bounds[1] - bounds[0];
			const left = bounds[0] + delta;
			const right = bounds[1] + delta;
			set([left, right]);
		}
	};

	const input = browserOnly(defer(() => fromEvent<KeyboardEvent>(document, 'keydown'))).pipe(
		map(({ key }) => key),
		filter(isKey),
		tap(handleInput)
	);

	const sub = input.subscribe();
	onDestroy(() => {
		sub.unsubscribe();
	});

	$: [a, b] = bounds.map((n) => Math.floor(n));

	let aspectRatio: number | undefined;
	$: half = midpoint - a;
</script>

<div class="test">
	<div>Bisect</div>

	<div class="band">
		<Band points={[bounds[0], midpoint, bounds[1]]} min={0} max={totalDuration} />
	</div>
	<div class="frames">
		{#each [a, midpoint, b] as f, i}
			<div class="frame">
				<Frame start={f} cursor={f} end={f} pending={false} bind:aspectRatio />
				{#if i !== 1}
					<div class="timecode">
						{formatTimecode(half)}
					</div>
				{/if}
			</div>
		{/each}
	</div>
</div>

<style lang="postcss">
	.test {
		position: relative;

		display: flex;
		flex-direction: column;
		gap: 1rem;
		align-self: center;
		justify-self: center;

		width: 90%;
		padding: 4rem;
	}

	.band {
		--svg-stroke-color: var(--fg);
	}
	.frames {
		display: flex;
		flex-direction: row;
		gap: 1rem;
		justify-content: space-between;

		height: max-content;
	}

	.frame {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		align-items: center;
	}
</style>
