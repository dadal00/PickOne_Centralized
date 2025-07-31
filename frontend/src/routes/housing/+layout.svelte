<script lang="ts">
	import { page } from '$app/state'
	import Footer from '$lib/housing/components/layout/Footer.svelte'
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

	{#if !page.url.pathname.includes('/submit-review')}
		<Footer />
	{/if}
</main>
