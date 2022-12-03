<script>
	import 'roslib/build/roslib';
	import Status from './Status.svelte';
	import LiveLidar from './LiveLIDAR.svelte';
	import Record from './Record.svelte';
	import Controls from './Controls.svelte';
	// import Menu from './Menu.svelte';
	//eslint-disable-next-line
	let ros = new ROSLIB.Ros({
		url: 'ws://localhost:9090'
	});
</script>

<div>
	<!-- <Menu /> -->
	<LiveLidar {ros} />
	<div class="absolute bottom-0 right-0 m-3">
		<Status bind:ros />
	</div>
	<div class="absolute bottom-0 left-0 m-3">
		<Record bind:ros />
	</div>
	<div class="absolute bottom-0 left-40 m-3 self-center">
		<Controls bind:ros />
	</div>
</div>
