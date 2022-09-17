<script>
	export let loading = true;
	export let commandList = [];
	import ReusableGLTF from "$lib/ReusableGLTF.svelte";
	import * as THREE from "three";
	import * as SC from "svelte-cubed";
	import { Command } from "@tauri-apps/api/shell";

	let spin = 60;
	let percentage = 0;
	let currentOperation = "";

	const delay = (ms) => new Promise((res) => setTimeout(res, ms));
	const commands = async () => {
		for (let command of commandList) {
			let parts = command.split(" ");
			currentOperation = "Cargando: " + parts[0];
			new Command(parts[0], parts.slice(1)).execute();
			await delay(1000);
			percentage += 1 / commandList.length;
		}
		currentOperation = "Listo";
	};

	commands();

	SC.onFrame(() => {
		spin += 0.01;
	});
</script>

<div class="flex h-screen flex-col items-center justify-center bg-gradient-to-b from-amber-400 to-pink-500 ">
	<div class="relative h-3/5  w-full">
		<SC.Canvas antialias alpha>
			<ReusableGLTF modelURL={"./golf_cart/scene.gltf"} name="golfCart" scale={[1, 1, 1]} rotation={[0, spin, 0]} position={[0, -0.6, 0]} />
			<SC.DirectionalLight color={new THREE.Color(0xffffdf)} position={[10, 10, 10]} intensity={0.9} shadow={false} />
			<SC.PerspectiveCamera position={[2.5, 0, 0]} />
		</SC.Canvas>
	</div>

	<div class="m-4 h-2.5 w-3/4 rounded-full bg-gray-700">
		<div class="h-2.5 rounded-full bg-blue-500 transition-all" style="width: {percentage * 100}%" />
	</div>
	{#if percentage === 1}
		<button on:click={() => (loading = !loading)} class="rounded-full py-2 px-6 bg-green-500 text-white font-bold m-3 text-xl hover:bg-green-700 shadow-lg">Iniciar</button>
	{:else}
		<div class="m-5 text-lg font-bold text-white">{currentOperation}</div>
	{/if}
</div>
