<script>
	import 'roslib/build/roslib';
	export let ros;
	let status = 'Grabar';
	let color = 'bg-red-500';
	let record = () => {
		if (status == 'Grabar') {
			status = 'Grabando';
			color = 'bg-green-500';
			//eslint-disable-next-line
			let record = new ROSLIB.Topic({
				ros: ros,
				name: '/record',
				messageType: 'std_msgs/String'
			});
			//eslint-disable-next-line
			record.publish(new ROSLIB.Message({ data: 'start' }));
		} else {
			status = 'Grabar';
			color = 'bg-red-500';
			//eslint-disable-next-line
			let record = new ROSLIB.Topic({
				ros: ros,
				name: '/record',
				//MessageType 'geometry_msgs/Vector3'
				messageType: 'geometry_msgs/Vector3'
			});
			//eslint-disable-next-line
			record.publish(
				new ROSLIB.Message({
					x: 0,

					y: 0,
					z: 0
				})
			);
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
