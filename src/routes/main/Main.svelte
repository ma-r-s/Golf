<script>
	import "roslib/build/roslib";
	import Status from "./Status.svelte";

	let ros = new ROSLIB.Ros({
		url: "ws://localhost:9090",
	});

	let message = "No hay mensajes";

	let listener = new ROSLIB.Topic({
		ros: ros,
		name: "/velodyne_points",
		messageType: "sensor_msgs/PointCloud2",
	});

	listener.subscribe((parMessage) => {
		message = parMessage;
		listener.unsubscribe();
	});

	let status = "off";
	ros.on("connection", () => (status = "on"));
	ros.on("error", () => (status = "error"));
	ros.on("close", () => (status = "off"));
</script>

<div class="h-screen bg-slate-400 flex justify-center flex-col items-center">
	<Status {status} />
	<div class="text-white">{message}</div>
</div>
