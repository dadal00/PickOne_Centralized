<script lang="ts">
	import Header from '$lib/housing/components/layout/Header.svelte'
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
	<Header />
	{@render children()}
</main>
