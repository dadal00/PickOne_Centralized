<script lang="ts">
	import Search from '$lib/housing/components/home/Search.svelte'
	import Stats from '$lib/housing/components/home/Stats.svelte'
	import PreviewCards from '$lib/housing/components/home/PreviewCards.svelte'
</script>

<div
	class="min-h-screen bg-gradient-to-br from-slate-50 via-white to-yellow-50 dark:from-gray-900 dark:via-black dark:to-yellow-900/20"
>
	<Search />
	<Stats />
	<PreviewCards />
</div>
