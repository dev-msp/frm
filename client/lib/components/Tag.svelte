<script lang="ts">
	export let label: string;
	export let value: string;

	export let color = '#000';
</script>

<div class="tag">
	<div class="label" style={`background-color: ${color}`}>
		{label}
	</div>
	<div class="value">
		{value}
	</div>
</div>

<style lang="postcss">
	.label,
	.value {
		padding: 4px;
	}
</style>
