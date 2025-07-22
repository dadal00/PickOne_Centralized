<script lang="ts">
	import PostingForm from '$lib/swap/components/posting/PostingForm.svelte'
	import PostingHeader from '$lib/swap/components/posting/PostingHeader.svelte'
	import PostingTips from '$lib/swap/components/posting/PostingTips.svelte'
</script>

<svelte:head>
	<title>Post Item - BoilerSwap</title>
	<meta name="description" content="Posting new item." />
</svelte:head>

<div class="container mx-auto px-6 py-8 max-w-2xl">
	<PostingHeader />
	<PostingForm />
	<PostingTips />
</div>
