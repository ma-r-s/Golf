<script>
	import "roslib/build/roslib";
	import Status from "./Status.svelte";
	import * as SC from "svelte-cubed";
	import * as THREE from "three";
	import ReusableGLTF from "$lib/ReusableGLTF.svelte";

	import PointCloud2 from "$lib/PointCloud2.svelte";

	let ros = new ROSLIB.Ros({
		url: "ws://localhost:9090",
	});

	let status = "off";
	ros.on("connection", () => (status = "on"));
	ros.on("error", () => (status = "error"));
	ros.on("close", () => (status = "off"));
</script>

<div class="flex h-screen justify-center bg-slate-400  flex-col items-center">
	<div class="relative h-4/5  w-11/12">
		<SC.Canvas antialias background={new THREE.Color("black")}>
			<PointCloud2 topicName={"/points_raw"} {ros} />
			<SC.DirectionalLight color={new THREE.Color(0xffffdf)} position={[10, 10, 10]} intensity={0.9} shadow={false} />
			<ReusableGLTF modelURL={"./golf_cart/scene.gltf"} name="golfCart" scale={[1.2, 1.2, 1.2]} rotation={[0, Math.PI, 0]} position={[0, -0.6, 0]} />
			<SC.PerspectiveCamera position={[-3, 4, 6]} />
			<SC.OrbitControls enableZoom={true} />
			<SC.Primitive object={new THREE.GridHelper(70, 70, 0x333333, 0x444444)} position={[0, -0.6, 0]} />
		</SC.Canvas>
	</div>
	<Status {status} />
</div>
