<script>
	import 'roslib/build/roslib';
	export let ros;
	let status = 'Grabar';
	let color = 'bg-red-500';
	let record = () => {
		if (status == 'Grabar') {
			status = 'Detener';
			color = 'bg-green-500';
		} else {
			status = 'Grabar';
			color = 'bg-red-500';
		}
	};
</script>

<div class="overflow-hidden">
	<button
		on:click={record}
		class=" rounded-full  {color} h-11 w-32 py-2 text-center text-xl font-bold text-white shadow-lg ease-in-out hover:bg-red-600"
	>
		{status}
	</button>
</div>
