<script lang="ts">
	import { page } from '$app/state'
	import HousingTitle from '$lib/housing/components/housing/HousingTitle.svelte'
	import { appState } from '$lib/housing/app-state.svelte'
	import Body from '$lib/housing/components/housing/Body.svelte'
	import { onMount } from 'svelte'
	import { housingSearch } from '$lib/housing/meili-client'
	import { defaultHousingSortBy, type HousingID } from '$lib/housing/models/housing'
	import { HousingIDIterable } from '$lib/housing/constants/housing'

	/*
		$derive will load in housing even if does not exist initially
		- such as when the loadHousing finishes in onMount
	*/
	let housing = $derived(appState.fetchHousing(page.params.id!))

	onMount(() => {
		// Use a temporary variable to ensure id exists
		const id = page.params.id

		// Verify that this housing is a valid option before pulling
		if (!housing && id && HousingIDIterable.includes(id as HousingID)) {
			housingSearch(id, '', '', '', defaultHousingSortBy, 0)
		}
	})
</script>

{#if !housing}
	<div>Housing not found</div>
{:else}
	<div
		class="min-h-screen bg-gradient-to-br from-slate-50 via-white to-yellow-50 dark:from-gray-900 dark:via-black dark:to-yellow-900/20"
	>
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
			<HousingTitle {housing} />
			<Body {housing} />
		</div>
	</div>
{/if}
