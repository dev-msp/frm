<script lang="ts">
	import type { State } from '$lib/state';
	import Bisect from './test/Bisect.svelte';
	import Grid from './Grid.svelte';

	export let state: State;
</script>

{#if state.view.type === 'loading'}
	<span>Loading</span>
{:else if state.view.type === 'bisect'}
	<Bisect state={state.view} />
{:else if state.view.type === 'grid'}
	<Grid state={state.view} />
{/if}
