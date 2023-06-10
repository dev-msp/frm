<script lang="ts">
	import { store } from '$lib/keys';
	import { distinctBy } from '$lib/stream';

	const source = store.runEpic((_a, s) => s.pipe(distinctBy()));
</script>

<slot state={$source} />
