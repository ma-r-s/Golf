<script>
	import * as THREE from 'three';
	import * as THRELTE from '@threlte/core';
	import PointCloud2 from '$lib/newPointCloud.svelte';
	export let ros;
</script>

<div class="relative h-full  w-full">
	<THRELTE.Canvas>
		<PointCloud2 topicName={'/points_raw'} {ros} />
		<THRELTE.AmbientLight color={0xffffff} intensity={0.3} />

		<THRELTE.PerspectiveCamera position={{ x: -3, y: 4, z: 6 }}>
			<THRELTE.OrbitControls enableZoom={true} />
		</THRELTE.PerspectiveCamera>
	</THRELTE.Canvas>
</div>
