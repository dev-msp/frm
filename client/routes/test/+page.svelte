<script lang="ts">
	import Bisect from './Bisect.svelte';
	import StoreProvider from '../StoreProvider.svelte';
</script>

<div class="container">
	<StoreProvider let:state>
		<Bisect {state} />
	</StoreProvider>
</div>

<style lang="postcss">
	.container {
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
