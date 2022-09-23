<script>
	import * as SC from "svelte-cubed";
	import * as THREE from "three";
	import ReusableGLTF from "$lib/ReusableGLTF.svelte";

	import PointCloud2 from "$lib/PointCloud2.svelte";
	export let ros;
</script>

<div class="relative h-full  w-full">
	<SC.Canvas antialias background={new THREE.Color("black")}>
		<PointCloud2 topicName={"/points_raw"} {ros} />
		<SC.DirectionalLight color={new THREE.Color(0xffffdf)} position={[10, 10, 10]} intensity={0.9} shadow={false} />
		<ReusableGLTF modelURL={"./golf_cart/scene.gltf"} name="golfCart" scale={[1.2, 1.2, 1.2]} rotation={[0, Math.PI, 0]} position={[0, -0.6, 0]} />
		<SC.PerspectiveCamera position={[-3, 4, 6]} />
		<SC.OrbitControls enableZoom={true} />
		<SC.Primitive object={new THREE.GridHelper(70, 70, 0x330033, 0x440044)} position={[0, -0.6, 0]} />
	</SC.Canvas>
</div>
