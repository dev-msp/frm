<script lang="ts">
	export let pending: boolean;
	export let start: number;
	export let end: number | undefined = undefined;
</script>

<div class="frame" class:pending>
	<slot name="label" />
	<img src={`/api/frame/${start}`} alt={`${start} - ${end ?? '?'}`} />
</div>

<style lang="css">
	.frame {
		width: auto;
		position: relative;
	}

	.frame img {
		width: 100%;
		object-fit: fill;
	}

	.pending {
		border: 1px solid yellow;
	}
</style>
