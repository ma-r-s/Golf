<script>
	import 'roslib/build/roslib';
	import * as THREE from 'three';
	import * as THRELTE from '@threlte/core';

	export let ros;
	export let topicName = '/velodyne_ points';

	let geometry = new THREE.BufferGeometry();
	const material = new THREE.PointsMaterial({ size: 0.005, color: 0x00aaff });
	let points = new THREE.Points(geometry, material);

	//eslint-disable-next-line
	let listener = new ROSLIB.Topic({
		ros: ros,
		name: topicName,
		messageType: 'sensor_msgs/PointCloud2'
	});

	listener.subscribe((msg) => {
		processMessage(msg);
	});

	let flag = true;
	let processMessage = (cloud) => {
		if (flag) {
			console.log(cloud);
		}
	};
</script>

<THRELTE.Object3DInstance position={{ x: 0 }} object={points} />
