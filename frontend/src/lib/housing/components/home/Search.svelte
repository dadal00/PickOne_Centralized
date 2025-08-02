<script lang="ts">
	import { PUBLIC_SVELTE_HOUSING_ROOT } from '$env/static/public'
	import { Home, MapPin, Search } from '@lucide/svelte'
	import Button from '../templates/Button.svelte'
	import Input from '../templates/Input.svelte'
	import { goto } from '$app/navigation'
	import { appState } from '$lib/housing/app-state.svelte'
	import type { CampusType, HousingType } from '$lib/housing/models/housing'

	let query: string = $state('')

	/*
		Reactively updating the query
	*/
	$effect(() => {
		appState.setHousingQuery(query)
	})

	// Go to search with specified filters
	function goToSearch(campusTypeFilter: CampusType | '', housingTypeFilter: HousingType | '') {
		appState.setCampusTypeFilter(campusTypeFilter)

		appState.setHousingTypeFilter(housingTypeFilter)

		goto(PUBLIC_SVELTE_HOUSING_ROOT + '/search')
	}
</script>

<section class="py-24 px-4 sm:px-6 lg:px-8">
	<div class="max-w-5xl mx-auto text-center">
		<div class="mb-8">
			<h2 class="text-6xl font-bold text-gray-900 dark:text-gray-100 mb-6 leading-tight">
				Find Your Perfect
				<span
					class="block bg-gradient-to-r from-yellow-500 to-yellow-600 bg-clip-text text-transparent"
					>Purdue Housing</span
				>
			</h2>
			<p class="text-xl text-gray-600 dark:text-gray-400 max-w-2xl mx-auto leading-relaxed">
				How to choose the best place to live in the land of corns 🌽
			</p>
		</div>

		<div class="max-w-2xl mx-auto mb-16">
			<div class="relative group">
				<div
					class="absolute inset-0 bg-gradient-to-r from-yellow-400 to-yellow-600 rounded-2xl blur opacity-25 group-hover:opacity-40 transition-opacity"
				></div>
				<div
					class="relative bg-white dark:bg-gray-800 rounded-2xl shadow-xl border border-gray-100 dark:border-gray-700"
				>
					<Search
						class="absolute left-6 top-1/2 transform -translate-y-1/2 text-gray-400 h-5 w-5"
					/>
					<Input
						type="text"
						placeholder="Search housing (e.g., Cary Quad, Fuse Apartments, McCutcheon Hall)"
						className="pl-14 pr-32 py-6 text-lg border-0 rounded-2xl focus:ring-2 focus:ring-yellow-500 bg-transparent"
						bind:query
					/>
					<Button
						className="absolute right-2 top-1/2 transform -translate-y-1/2 rounded-xl bg-gradient-to-r from-yellow-500 to-yellow-600 hover:from-yellow-600 hover:to-yellow-700 px-6 py-3 shadow-lg"
						action={() => goToSearch('', '')}
					>
						Search
					</Button>
				</div>
			</div>
		</div>

		<div class="flex flex-wrap justify-center gap-4 mb-20">
			<Button
				variant="outline"
				className="rounded-full border-2 border-gray-200 hover:border-yellow-400 hover:bg-yellow-50 bg-white px-6 py-3 text-gray-700 hover:text-yellow-700 transition-all shadow-sm hover:shadow-md dark:bg-gray-800 dark:border-gray-700 dark:text-gray-300 dark:hover:bg-gray-700 dark:hover:border-yellow-500"
				action={() => goToSearch('On-Campus', 'Dorm')}
			>
				<Home class="h-4 w-4 mr-2" />
				On-Campus Dorms
			</Button>
			<Button
				variant="outline"
				className="rounded-full border-2 border-gray-200 hover:border-yellow-400 hover:bg-yellow-50 bg-white px-6 py-3 text-gray-700 hover:text-yellow-700 transition-all shadow-sm hover:shadow-md dark:bg-gray-800 dark:border-gray-700 dark:text-gray-300 dark:hover:bg-gray-700 dark:hover:border-yellow-500"
				action={() => goToSearch('Off-Campus', 'Apartment')}
			>
				<MapPin class="h-4 w-4 mr-2" />
				Off-Campus Apartments
			</Button>
		</div>
	</div>
</section>
