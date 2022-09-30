<script>
	import 'roslib/build/roslib';
	import * as THREE from 'three';
	import * as THRELTE from '@threlte/core';

	export let ros;
	export let topicName = '/velodyne_points';

	let geometry = new THREE.BufferGeometry();
	const material = new THREE.PointsMaterial({ size: 0.07, vertexColors: true });

	$: points = new THREE.Points(geometry, material);

	//eslint-disable-next-line
	let listener = new ROSLIB.Topic({
		ros: ros,
		name: topicName,
		messageType: 'sensor_msgs/PointCloud2'
	});

	listener.subscribe((msg) => {
		processMessage(msg);
	});

	function base64ToArrayBuffer(base64) {
		var binary_string = window.atob(base64);
		var len = binary_string.length;
		var bytes = new Uint8Array(len);
		for (var i = 0; i < len; i++) {
			bytes[i] = binary_string.charCodeAt(i);
		}
		return bytes.buffer;
	}

	let processMessage = (cloud) => {
		const buffer = base64ToArrayBuffer(cloud.data);
		let vertices = [];
		let colors = [];
		const color = new THREE.Color();
		for (let i = 0; i <= buffer.byteLength - cloud.point_step; i += cloud.point_step) {
			const data = new DataView(buffer);
			vertices.push(
				data.getFloat32(i, true),
				data.getFloat32(i + 8, true),
				data.getFloat32(i + 4, true)
			);
			color.setHSL(i / buffer.byteLength, 1, 0.5);
			colors.push(color.r, color.g, color.b);
		}
		geometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3));
		geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3));
		geometry = geometry;
	};
</script>

<THRELTE.Object3DInstance position={{ x: 0 }} object={points} />
