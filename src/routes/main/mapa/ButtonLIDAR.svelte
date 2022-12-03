<script>
	import 'roslib/build/roslib';
	export let estado;
	let status = 'Nuevo Destino';
	let color = 'bg-purple-500';
	let destino = () => {
		estado = 'mapa';
	};
</script>

<div class="overflow-hidden">
	<button
		on:click={destino}
		class=" rounded-full  {color} h-11 w-72 py-2 text-center text-xl font-bold text-white shadow-lg ease-in-out hover:bg-sky-600"
	>
		{status}
	</button>
</div>
