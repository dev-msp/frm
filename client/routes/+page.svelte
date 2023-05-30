<script lang="ts">
	import { alpha, state, initial } from '$lib/keys';
	import Frame from './frame.svelte';

	const q = parseInt(new URLSearchParams(window.location.search).get('q') ?? '20');

	let currentState = initial;

	$: currentState = $state;
</script>

<div class="root">
	<h1>frm</h1>
	<div class="results">
		{#each currentState?.data ?? [] as [s], i (s)}
			{@const c = alpha[i]}
			<Frame start={s} pending={false}>
				<span slot="label" class="label">
					{c}
				</span>
			</Frame>
		{/each}
	</div>
</div>

<style lang="css">
	div {
		--bg: #222;
		--fg: #ccc;
		background-color: var(--bg);
		color: var(--fg);
	}

	.root {
		box-sizing: border-box;
		width: 100vw;
		height: 100vh;
		padding: 2% 2% 0 2%;
		display: flex;
		flex-direction: column-reverse;
	}

	.results {
		width: 100%;
		height: 100%;
		display: grid;
		grid-template-columns: repeat(5, 1fr);
		grid-auto-rows: min-content;
		gap: 1rem;
	}

	span.label {
		position: absolute;
		bottom: 4px;
		right: 4px;
		z-index: 99;
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 17px;
		height: 17px;
		border: 1px solid var(--fg);
		text-transform: uppercase;
		color: var(--fg);
		background-color: var(--bg);
	}
</style>
