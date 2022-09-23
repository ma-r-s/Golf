<script>
	export let ros;
	let status = "Desconectado";
	let color = "bg-red-500";
	ros.on("connection", () => {
		status = "Conectado";
		color = "bg-green-500";
	});
	ros.on("error", () => {
		status = "Error";
		color = "bg-oragne-500";
		reconect();
	});
	ros.on("close", () => {
		status = "Desconectado";
		color = "bg-red-500";
		reconect();
	});

	let reconect = () => {
		if (status != "Conectado") {
			ros = new ROSLIB.Ros({
				url: "ws://localhost:9090",
			});
			setTimeout(reconect, 1000);
		}
	};
</script>

<div class="w-40 rounded-full {color} py-2 text-center text-xl font-bold  text-white shadow-lg">{status}</div>
