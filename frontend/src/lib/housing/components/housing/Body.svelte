<script lang="ts">
	import Overview from '$lib/housing/components/housing/subcomponents/Overview.svelte'
	import TabPiece from '$lib/housing/components/templates/TabPiece.svelte'
	import Reviews from '$lib/housing/components/housing/subcomponents/Reviews.svelte'
	import SearchComponent from '$lib/housing/components/search/Search.svelte'
	import { type Housing } from '$lib/housing/models/housing'

	const tabs = ['Overview', 'Reviews']

	const { housing } = $props<{
		housing: Housing
	}>()

	let activeTab = $state(tabs[0])
</script>

<div class="space-y-8">
	<TabPiece
		className="grid w-full grid-cols-2 bg-white/80 backdrop-blur-sm rounded-xl shadow-lg border border-gray-100 p-1 dark:bg-gray-800/80 dark:border-gray-700"
		tabPiece="list"
	>
		{#each tabs as tab}
			<TabPiece
				className="rounded-lg font-semibold"
				tabValue={tab}
				tabPiece="trigger"
				activeClasses="bg-gradient-to-r from-yellow-500 to-yellow-600 text-white"
				bind:activeTab
			>
				{tab}
			</TabPiece>
		{/each}
	</TabPiece>

	<TabPiece tabPiece="content" className="space-y-8">
		{#if activeTab === 'Overview'}
			<Overview {housing} />
		{:else if activeTab === 'Reviews'}
			<SearchComponent searchFor="Reviews" />
			<Reviews />
		{/if}
	</TabPiece>
</div>
