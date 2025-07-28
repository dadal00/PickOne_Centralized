<script lang="ts">
	import { page } from '$app/state'
	import HousingTitle from '$lib/housing/components/housing/HousingTitle.svelte'
	import { appState } from '$lib/housing/AppState.svelte'
	import Body from '$lib/housing/components/housing/Body.svelte'

	let housing = $derived(appState.getHousingDetails(page.params.id))
</script>

{#if !housing}
	<div>Housing not found</div>
{:else}
	<div
		class="min-h-screen bg-gradient-to-br from-slate-50 via-white to-yellow-50 dark:from-gray-900 dark:via-black dark:to-yellow-900/20"
	>
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
			<HousingTitle {housing} />
			<Body />
		</div>
	</div>
{/if}
