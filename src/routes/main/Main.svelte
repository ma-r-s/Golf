<script>
	import * as ROSLIB from 'roslib/build/roslib';
	import Status from './Status.svelte';
	import LiveLidar from './LiveLIDAR.svelte';
	import Menu from './Menu.svelte';
	let ros = new ROSLIB.Ros({
		url: 'ws://localhost:9090'
	});
</script>

<div class="">
	<Menu />
	<LiveLidar {ros} />
	<div class="absolute bottom-0 right-0 m-3">
		<Status bind:ros />
	</div>
</div>
