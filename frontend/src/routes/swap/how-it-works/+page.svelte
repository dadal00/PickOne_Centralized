<script lang="ts">
	import Header from '$lib/home/components/Header.svelte'
	import Guide from '$lib/swap/components/how-it-works/Guide.svelte'
	import Instructions from '$lib/swap/components/how-it-works/Instructions.svelte'
	import SwapBrowse from '$lib/swap/components/how-it-works/SwapBrowse.svelte'
</script>

<svelte:head>
	<title>How It Works - BoilerSwap</title>
	<meta name="description" content="The process behind BoilerSwap." />
</svelte:head>

<div class="container mx-auto px-6 py-16 max-w-4xl">
	<Header />
	<Instructions />
	<Guide />
	<SwapBrowse />
</div>
