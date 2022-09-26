<script>
	let butons = ['island operations', 'finances', 'omnidroid metatraining', 'supers'];
</script>

<div class="flex h-screen flex-col">
	<div class="basis-1/5 bg-[#5e8384]" />
	<div
		class="flex basis-3/5 flex-col items-start justify-evenly bg-[#c5e2e3] px-4 font-mono text-xl   tracking-widest "
	>
		{#each butons as title}
			<button
				class="w-full rounded-lg px-4 text-left uppercase transition hover:bg-zinc-500/30 hover:text-white"
				>{title}</button
			>
		{/each}
	</div>
	<div class="basis-1/5 bg-[#5e8384]" />
</div>
