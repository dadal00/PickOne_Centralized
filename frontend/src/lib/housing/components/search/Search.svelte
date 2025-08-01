<script lang="ts">
	import { Funnel, Search } from '@lucide/svelte'
	import CustomSelect from '../templates/CustomSelect.svelte'
	import {
		housingFilterCampusSelect,
		housingFilterCostSelect,
		housingFilterTypeSelect
	} from '$lib/housing/models/housing'
	import { onDestroy, onMount } from 'svelte'
	import { appState } from '$lib/housing/AppState.svelte'

	// Query state variable to sync with central state
	let query: string = $state('')

	onMount(() => {
		console.log(appState.getHousingTypeFilter())

		const fullQuery = appState.getFullHousingQuery()

		query = fullQuery.query
	})

	$effect(() => {
		appState.setHousingQuery(query)
	})

	onDestroy(() => {
		appState.setHousingQuery('')

		appState.setOffset(0)
	})
</script>

<div
	class="bg-white/80 backdrop-blur-sm rounded-2xl shadow-xl border border-gray-100 p-8 mb-8 dark:bg-gray-800/80 dark:border-gray-700"
>
	<div class="flex items-center space-x-3 mb-6">
		<Funnel class="h-5 w-5 text-yellow-600" />
		<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">Search & Filter</h2>
	</div>
	<div class="flex flex-wrap items-center gap-4">
		<div class="md:col-span-2 flex-grow">
			<div class="relative">
				<Search class="absolute left-4 top-1/2 transform -translate-y-1/2 text-gray-400 h-5 w-5" />
				<input
					type="text"
					bind:value={query}
					placeholder="Search housing..."
					class={'flex h-10 w-full border border-gray-200 bg-background px-3 pl-12 py-3 text-base md:text-sm file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground rounded-xl focus-visible:outline-none focus:border-yellow-400 focus:ring-2 focus:ring-yellow-400 disabled:cursor-not-allowed disabled:opacity-50 dark:bg-gray-700 dark:border-gray-600 dark:text-gray-200'}
				/>
			</div>
		</div>
		<CustomSelect
			selectOptions={housingFilterTypeSelect}
			getAction={appState.getHousingTypeFilter.bind(appState)}
			setAction={appState.setHousingTypeFilter.bind(appState)}
		/>
		<CustomSelect
			selectOptions={housingFilterCampusSelect}
			getAction={appState.getCampusTypeFilter.bind(appState)}
			setAction={appState.setCampusTypeFilter.bind(appState)}
		/>
		<CustomSelect
			selectOptions={housingFilterCostSelect}
			getAction={appState.getCostSymbolFilter.bind(appState)}
			setAction={appState.setCostSymbolFilter.bind(appState)}
		/>
	</div>
</div>
