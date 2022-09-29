<script>
	export let loading = true;
	export let commandList = [];
	import * as THRELTE from '@threlte/core';
	import { Command } from '@tauri-apps/api/shell';
	import { GLTF } from '@threlte/extras';
	let percentage = 0;
	let currentOperation = '';

	const delay = (ms) => new Promise((res) => setTimeout(res, ms));
	const commands = async () => {
		for (let command of commandList) {
			let parts = command.split(' ');
			currentOperation = 'Cargando: ' + parts[0];
			new Command(parts[0], parts.slice(1)).execute();
			await delay(1000);
			percentage += 1 / commandList.length;
		}
		currentOperation = 'Listo';
	};

	commands();
</script>

<div
	class="flex h-screen flex-col items-center justify-center bg-gradient-to-b from-amber-400 to-pink-500 "
>
	<div class="relative h-3/5  w-full">
		<THRELTE.Canvas>
			<GLTF interactive position={{ y: -0.5 }} url={'./golf_cart/scene.gltf'} />
			<THRELTE.AmbientLight color={0xffffff} intensity={0.3} />
			<THRELTE.PerspectiveCamera position={{ x: 1.7, y: 0, z: 1.7 }}>
				<THRELTE.OrbitControls autoRotate />
			</THRELTE.PerspectiveCamera>
		</THRELTE.Canvas>
	</div>

	<div class="m-4 h-2.5 w-3/4 rounded-full bg-gray-700">
		<div class="h-2.5 rounded-full bg-blue-500 transition-all" style="width: {percentage * 100}%" />
	</div>
	{#if percentage === 1}
		<button
			on:click={() => (loading = !loading)}
			class="m-3 rounded-full bg-green-500 py-2 px-6 text-xl font-bold text-white shadow-lg hover:bg-green-700"
			>Iniciar</button
		>
	{:else}
		<div class="m-5 text-lg font-bold text-white">{currentOperation}</div>
	{/if}
</div>
