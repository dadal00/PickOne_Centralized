<script lang="ts">
	import Results from '$lib/housing/components/search/Results.svelte'
	import SearchComponent from '$lib/housing/components/search/Search.svelte'
</script>

<div
	class="min-h-screen bg-gradient-to-br from-slate-50 via-white to-yellow-50 dark:from-gray-900 dark:via-black dark:to-yellow-900/20"
>
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<SearchComponent />
		<Results />
	</div>
</div>
