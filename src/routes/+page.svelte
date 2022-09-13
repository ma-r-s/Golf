<script>
	import Loading from "./Loading.svelte";
	import Main from "./main/Main.svelte";
	let loading = true;
</script>

{#if loading}
	<Loading />
{:else}
	<Main />
{/if}
