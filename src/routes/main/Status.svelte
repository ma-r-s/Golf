<script>
	export let status = "off";
</script>

{#if status === "on"}
	<div class="rounded-full py-2 w-40 text-center bg-green-500 text-white font-bold m-3 text-xl shadow-lg">Conectado</div>
{:else if status === "error"}
	<div class="rounded-full py-2 w-40 text-center bg-orange-500 text-white font-bold m-3 text-xl shadow-lg">Error</div>
{:else}
	<div class="rounded-full py-2 w-40 text-center bg-red-500 text-white font-bold m-3 text-xl shadow-lg">Desconectado</div>
{/if}
