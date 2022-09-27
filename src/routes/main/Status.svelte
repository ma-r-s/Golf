<script>
	export let ros;
	let status = 'Desconectado';
	let color = 'bg-red-500';
	ros.on('connection', () => {
		status = 'Conectado';
		color = 'bg-green-500';
	});
	ros.on('error', () => {
		status = 'Error';
		color = 'bg-orange-500';
		reconect();
	});
	ros.on('close', () => {
		status = 'Desconectado';
		color = 'bg-red-500';
		reconect();
	});

	let reconect = () => {
		if (status != 'Conectado') {
			ros.connect('ws://Mario:11311');
			setTimeout(reconect, 500);
		}
	};
</script>

<div class="overflow-hidden">
	<div
		class="h-6 w-6 animate-pulse rounded-full transition-all  hover:animate-none {color} text-clip py-2 text-center text-xl font-bold text-transparent shadow-lg hover:h-11 hover:w-40 hover:text-white hover:ease-in-out"
	>
		{status}
	</div>
</div>
