<script>
	import * as THREE from "three";
	import * as SC from "svelte-cubed";
	import ReusableGLTF from "$lib/ReusableGLTF.svelte";
	let spin = 60;
	SC.onFrame(() => {
		spin += 0.01;
	});
</script>

<div class="flex flex-col items-center justify-center h-screen">
	<div class="relative w-full  h-3/5">
		<SC.Canvas antialias alpha>
			<ReusableGLTF modelURL={"./golf_cart/scene.gltf"} name="golfCart" scale={[1, 1, 1]} rotation={[0, spin, 0]} position={[0, -0.6, 0]} />
			<SC.DirectionalLight color={new THREE.Color(0xffffdf)} position={[10, 10, 10]} intensity={0.9} shadow={false} />
			<SC.PerspectiveCamera position={[2.5, 0, 0]} />
		</SC.Canvas>
	</div>
	<div class="font-bold text-white m-3">Cargando</div>
	<div class="w-3/4 bg-gray-700 rounded-full h-2.5 mb-4">
		<div class="bg-blue-500 h-2.5 rounded-full" style="width: 60%" />
	</div>
</div>
