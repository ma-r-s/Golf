<script>
	import LeafletMap from '$lib/LeafletMap.svelte';
	import ButtonLIDAR from './ButtonLIDAR.svelte';
</script>

<main>
	<div class="absolute top-0 right-0 m-3">
		<ButtonLIDAR />
	</div>
	<LeafletMap />
</main>
