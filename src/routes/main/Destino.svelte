<script>
	import 'roslib/build/roslib';
	let status = 'Nuevo Destino';
	let color = 'bg-purple-500';
	let record = () => {
		if (status == 'Grabar') {
			status = 'Detener';
			color = 'bg-purple-500';
		} else {
			status = 'Grabar';
			color = 'bg-sky-500';
		}
	};
</script>

<div class="overflow-hidden">
	<button
		on:click={record}
		class=" rounded-full  {color} h-11 w-80 py-2 text-center text-xl font-bold text-white shadow-lg ease-in-out hover:bg-sky-600"
	>
		{status}
	</button>
</div>
