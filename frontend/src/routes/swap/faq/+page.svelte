<script lang="ts">
	import Header from '$lib/swap/components/faq/Header.svelte'
	import Questions from '$lib/swap/components/faq/Questions.svelte'
</script>

<svelte:head>
	<title>FAQ - BoilerSwap</title>
	<meta name="description" content="Frequently asked questions about BoilerSwap." />
</svelte:head>

<div class="container mx-auto px-6 py-16 max-w-4xl">
	<Header />
	<Questions />
</div>
