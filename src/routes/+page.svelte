<script>
	import LoadScreen from "./LoadScreen.svelte";
	import Main from "./main/Main.svelte";
	let loading = true;
</script>

{#if loading}
	<LoadScreen />
{:else}
	<Main />
{/if}
