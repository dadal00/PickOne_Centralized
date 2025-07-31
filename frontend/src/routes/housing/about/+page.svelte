<script lang="ts">
	import Title from '$lib/housing/components/about/Title.svelte'
	import Why from '$lib/housing/components/about/Why.svelte'
	import Values from '$lib/housing/components/about/Values.svelte'
	import Join from '$lib/housing/components/about/Join.svelte'
</script>

<div
	class="min-h-screen bg-gradient-to-br from-slate-50 via-white to-yellow-50 dark:from-gray-900 dark:via-black dark:to-yellow-900/20"
>
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
		<Title />
		<Why />
		<Values />
		<Join />
	</div>
</div>
