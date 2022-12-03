<script>
	import * as THRELTE from '@threlte/core';
	import * as THREE from 'three';
	import PointCloud2 from '$lib/PointCloud.svelte';
	export let ros;
</script>

<div class="relative h-screen w-full bg-black">
	<THRELTE.Canvas>
		<PointCloud2 {ros} permanent={false} topicName={'/points_raw'} />
		<THRELTE.Mesh
			receiveShadow
			rotation={{ x: -90 * (Math.PI / 180) }}
			geometry={new THREE.CircleGeometry(0.5, 72)}
			material={new THREE.MeshStandardMaterial({ color: 'white' })}
		/>
		<THRELTE.AmbientLight color={0xffffff} intensity={1} />
		<THRELTE.PerspectiveCamera position={{ x: -3, y: 4, z: 6 }}>
			<THRELTE.OrbitControls enableZoom={true} />
		</THRELTE.PerspectiveCamera>
	</THRELTE.Canvas>
</div>
