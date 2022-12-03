<script>
	import 'roslib/build/roslib';
	import Status from './Status.svelte';
	import LiveLidar from './LiveLIDAR.svelte';
	import Record from './Record.svelte';
	import Controls from './Controls.svelte';
	import Destino from './Destino.svelte';
	import Mapa from './mapa/Mapa.svelte';
	// import Menu from './Menu.svelte';
	//eslint-disable-next-line
	let ros = new ROSLIB.Ros({
		url: 'ws://localhost:9090'
	});
	let estado = 'lidar';
</script>

{#if estado == 'lidar'}
	<div>
		<LiveLidar {ros} />
		<div class="absolute bottom-0 right-0 m-3">
			<Status bind:ros />
		</div>
		<div class="absolute bottom-0 left-0 m-3">
			<Record bind:ros />
		</div>
		<div class="absolute bottom-0 left-40 m-3">
			<Controls bind:ros />
		</div>
		<div class="absolute top-0 left-0 m-3">
			<Destino bind:estado />
		</div>
	</div>
{:else}
	<div>
		<Mapa bind:estado />
	</div>
{/if}
