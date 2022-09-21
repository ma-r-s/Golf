<script>
	import "roslib/build/roslib";
	import * as THREE from "three";
	import * as SC from "svelte-cubed";

	export let ros;
	export let topicName = "/velodyne_ points";
	export let max_pts = 1000000;
	export let pointRatio = 1;
	let geometry = new THREE.BufferGeometry();
	export const material = new THREE.PointsMaterial({ size: 0.005, color: 0x00aaff });
	let buffer = null;
	let fields = {};

	let decode64 = (inbytes, outbytes, record_size, pointRatio) => {
		let x,
			b = 0,
			l = 0,
			j = 0,
			L = inbytes.length,
			A = outbytes.length;
		record_size = record_size || A; // default copies everything (no skipping)
		pointRatio = pointRatio || 1; // default copies everything (no skipping)
		let bitskip = (pointRatio - 1) * record_size * 8;
		for (x = 0; x < L && j < A; x++) {
			b = (b << 6) + decode64.e[inbytes.charAt(x)];
			l += 6;
			if (l >= 8) {
				l -= 8;
				outbytes[j++] = (b >>> l) & 0xff;
				if (j % record_size === 0) {
					x += Math.ceil((bitskip - l) / 6);
					l = l % 8;

					if (l > 0) {
						b = decode64.e[inbytes.charAt(x)];
					}
				}
			}
		}
		return Math.floor(j / record_size);
	};
	// initialize decoder with static lookup table 'e'
	decode64.S = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
	decode64.e = {};

	for (let i = 0; i < 64; i++) {
		decode64.e[decode64.S.charAt(i)] = i;
	}

	let positions = [];
	let processMessage = (msg) => {
		if (true) {
			for (var i = 0; i < msg.fields.length; i++) {
				fields[msg.fields[i].name] = msg.fields[i];
			}
		}
		let bufSz = max_pts * msg.point_step;

		if (!buffer || buffer.byteLength < bufSz) {
			buffer = new Uint8Array(bufSz);
		}
		console.log("flag");
		console.log(buffer);
		let n = decode64(msg.data, buffer, msg.point_step, pointRatio);
		let dv = new DataView(buffer.buffer);
		let littleEndian = !msg.is_bigendian;
		let x = fields["x"].offset;
		let y = fields["y"].offset;
		let z = fields["z"].offset;
		let base;
		for (let i = 0; i < n; i++) {
			base = i * msg.point_step;
			positions[3 * i] = dv.getFloat32(base + x, littleEndian);
			positions[3 * i + 1] = dv.getFloat32(base + z, littleEndian);
			positions[3 * i + 2] = dv.getFloat32(base + y, littleEndian);
		}

		if (positions.length > 0) geometry.setAttribute("position", new THREE.Float32BufferAttribute(positions, 3));
		geometry = geometry;
	};

	// subscribe to the topic
	let listener = new ROSLIB.Topic({
		ros: ros,
		name: topicName,
		messageType: "sensor_msgs/PointCloud2",
	});
	listener.subscribe(processMessage.bind(this));
</script>

<SC.Primitive object={new THREE.Points(geometry, material)} position={[0, 0, 0]} />
