<script>
	import "roslib/build/roslib";
	import Status from "./Status.svelte";
	import LiveLidar from "./LiveLIDAR.svelte";

	let ros = new ROSLIB.Ros({
		url: "ws://localhost:9090",
	});
</script>

<div class="flex h-screen items-center  bg-slate-800">
	<LiveLidar {ros} />
	<div class="absolute bottom-0 right-0 m-3">
		<Status bind:ros />
	</div>
</div>
