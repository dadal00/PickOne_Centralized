<script lang="ts">
	import BrowseSearch from '$lib/swap/components/browse/BrowseSearch.svelte'
	import Items from '$lib/swap/components/browse/Items.svelte'
	import PageButtons from '$lib/swap/components/browse/PageButtons.svelte'
</script>

<svelte:head>
	<title>Browse Items - BoilerSwap</title>
	<meta name="description" content="Find new items!" />
</svelte:head>

<div class="container mx-auto px-6 py-8">
	<BrowseSearch />
	<Items />
	<PageButtons />
</div>
