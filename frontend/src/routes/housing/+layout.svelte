<script lang="ts">
	import '$lib/housing/housing.css'

	let { children } = $props()
</script>

<svelte:head>
	<title>RateMyPWLHousing</title>
	<meta
		name="description"
		content="Find the best housing at Purdue University with student reviews."
	/>
</svelte:head>

<main>
	{@render children()}
</main>
