<div class="root">
	<slot />
</div>

<style lang="postcss">
	.root {
		--fg: #f2e3d5;
		--bg: #222;
		--svg-stroke-color: var(--fg);

		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: space-evenly;

		width: 100vw;
		height: 100vh;

		color: var(--fg);

		background-color: var(--bg);
	}
</style>
