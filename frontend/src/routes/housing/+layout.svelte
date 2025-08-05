<script lang="ts">
	import { page } from '$app/state'
	import Footer from '$lib/housing/components/layout/Footer.svelte'
	import Header from '$lib/housing/components/layout/Header.svelte'
	import { flushThumbs } from '$lib/housing/helpers/housing'
	import '$lib/housing/housing.css'
	import { onMount } from 'svelte'

	let { children } = $props()

	onMount(() => {
		const flush = () => {
			flushThumbs()
		}

		window.addEventListener('beforeunload', flush)
		window.addEventListener('pagehide', flush)

		return () => {
			window.removeEventListener('beforeunload', flush)
			window.removeEventListener('pagehide', flush)
		}
	})
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
