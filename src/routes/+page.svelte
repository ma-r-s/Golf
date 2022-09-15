<script>
	import LoadScreen from "./LoadScreen.svelte";
	import Main from "./main/Main.svelte";
	let loading = true;
	let commandList = ["roscore", "rviz"];
</script>

{#if loading}
	<LoadScreen {commandList} />
{:else}
	<Main />
{/if}
