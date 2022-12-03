<script>
	import 'roslib/build/roslib';
	export let ros;
	let color = 'bg-orange-500';
	let direccion = 0;
	let vel = 200;
	let adelante = () => {
		vel = 200;
		let twist = new ROSLIB.Message({
			x: direccion,
			y: vel,
			z: 0.0
		});
		let cmdVel = new ROSLIB.Topic({
			ros: ros,
			name: '/dir',
			messageType: 'geometry_msgs/Vector3'
		});
		cmdVel.publish(twist);
	};
	let atras = () => {
		vel = 0;
		let twist = new ROSLIB.Message({
			x: direccion,
			y: vel,
			z: 0.0
		});
		let cmdVel = new ROSLIB.Topic({
			ros: ros,
			name: '/dir',
			messageType: 'geometry_msgs/Vector3'
		});
		cmdVel.publish(twist);
	};
	let izquierda = () => {
		if (direccion < 1) {
			direccion += 0.1;
		}

		let twist = new ROSLIB.Message({
			x: direccion,
			y: vel,
			z: 0.0
		});
		let cmdVel = new ROSLIB.Topic({
			ros: ros,
			name: '/dir',
			messageType: 'geometry_msgs/Vector3'
		});
		cmdVel.publish(twist);
	};
	let derecha = () => {
		if (direccion > -1) {
			direccion -= 0.1;
		}
		let twist = new ROSLIB.Message({
			x: direccion,
			y: vel,
			z: 0.0
		});
		let cmdVel = new ROSLIB.Topic({
			ros: ros,
			name: '/dir',
			messageType: 'geometry_msgs/Vector3'
		});
		cmdVel.publish(twist);
	};
</script>

<div class="flex gap-10 overflow-hidden">
	<button
		on:click={adelante}
		class=" rounded-full  {color} h-11 w-11 py-2 text-center text-xl font-bold text-white shadow-lg ease-in-out hover:bg-red-600"
	>
		↑
	</button>
	<button
		on:click={atras}
		class=" rounded-full  {color} h-11 w-11 py-2 text-center text-xl font-bold text-white shadow-lg ease-in-out hover:bg-red-600"
	>
		↓
	</button>
	<button
		on:click={izquierda}
		class=" rounded-full  {color} h-11 w-11 py-2 text-center text-xl font-bold text-white shadow-lg ease-in-out hover:bg-red-600"
	>
		←
	</button>
	<button
		on:click={derecha}
		class=" rounded-full  {color} h-11 w-11 py-2 text-center text-xl font-bold text-white shadow-lg ease-in-out hover:bg-red-600"
	>
		→
	</button>
	<div class="font-bold text-white">
		<p>Vel: {vel}</p>
	</div>
	<div class="font-bold text-white">
		<p>Dir: {direccion}</p>
	</div>
</div>
