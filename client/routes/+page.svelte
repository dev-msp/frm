<script lang="ts">
	import App from './App.svelte';
	import StoreProvider from './StoreProvider.svelte';
</script>

<div class="container">
	<StoreProvider let:state>
		<App {state} />
	</StoreProvider>
</div>

<style lang="postcss">
	.container {
		--ncols: 3;
		--nrows: 3;

		display: flex;
		flex-direction: column;
		justify-content: center;

		box-sizing: border-box;
		width: 100%;
		height: 100%;
		padding: 1%;

		color: var(--fg);

		background-color: var(--bg);
	}
</style>
