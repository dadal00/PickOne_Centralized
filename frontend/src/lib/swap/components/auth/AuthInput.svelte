<script lang="ts">
	import { TabOptionsIterable, type TabOptions } from '$lib/swap/models'
	import AuthFlow from './subcomponents/AuthFlow.svelte'
	import TabButton from './subcomponents/TabButton.svelte'

	let activeTab: TabOptions = $state('Login')

	function showTab(tab: TabOptions) {
		activeTab = tab
	}
</script>

<div class="container mx-auto px-6 py-16 max-w-md">
	<div class="text-center mb-8">
		<h1 class="text-3xl font-bold text-gray-900 mb-4">Welcome to BoilerSwap</h1>
		<p class="text-gray-600">Join the Purdue community of sustainable sharing</p>
	</div>

	<div class="bg-white rounded-lg shadow-sm border p-6">
		<div class="flex border-b mb-6">
			{#each TabOptionsIterable as tab}
				<TabButton bind:activeTabValue={activeTab} tabName={tab} {showTab} />
			{/each}
		</div>
		<AuthFlow bind:activeTabValue={activeTab} {showTab} />
	</div>
</div>
