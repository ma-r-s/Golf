<script>
	import * as THRELTE from '@threlte/core';
	import * as THREE from 'three';
	import PointCloud2 from '$lib/newPointCloud.svelte';
	export let ros;
</script>

<div class="relative h-screen w-full  bg-black">
	<THRELTE.Canvas h>
		<PointCloud2 {ros} />
		<THRELTE.Mesh
			receiveShadow
			rotation={{ x: -90 * (Math.PI / 180) }}
			geometry={new THREE.CircleBufferGeometry(3, 72)}
			material={new THREE.MeshStandardMaterial({ color: 'white' })}
		/>
		<THRELTE.AmbientLight color={0xffffff} intensity={0.3} />
		<THRELTE.PerspectiveCamera position={{ x: -3, y: 4, z: 6 }}>
			<THRELTE.OrbitControls enableZoom={true} />
		</THRELTE.PerspectiveCamera>
	</THRELTE.Canvas>
</div>
