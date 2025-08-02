<script lang="ts">
	import { PUBLIC_SVELTE_HOUSING_ROOT } from '$env/static/public'
	import { appState } from '$lib/housing/app-state.svelte'
	import Button from '$lib/housing/components/templates/Button.svelte'
	import CardPiece from '$lib/housing/components/templates/CardPiece.svelte'
	import { ArrowRight, Star } from '@lucide/svelte'
	import { defaultHousingSortBy, type HousingID } from '$lib/housing/models/housing'
	import { HousingFields } from '$lib/housing/constants/housing'
	import { HousingNameLabels } from '$lib/housing/constants/housing'
	import { convertRatingToHousingLabel } from '$lib/housing/helpers/housing'
	import { onMount } from 'svelte'
	import { housingSearch } from '$lib/housing/meili-client'

	/*
		$derive will load in housing options once search is done
	*/
	const featuredHousing = $derived(appState.sampleHousing(3))

	onMount(() => {
		// Run a default search when user loads up website
		housingSearch('', '', '', '', defaultHousingSortBy, 0)
	})
</script>

<section class="py-20 px-4 sm:px-6 lg:px-8">
	<div class="max-w-7xl mx-auto">
		<div class="text-center mb-16">
			<h3 class="text-4xl font-bold text-gray-900 dark:text-gray-100 mb-4">
				Popular Housing Options
			</h3>
			<p class="text-lg text-gray-600 dark:text-gray-400">
				Discover what fellow Boilermakers are saying
			</p>
		</div>
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
			{#each featuredHousing as housing}
				<CardPiece
					className="group overflow-hidden hover:shadow-2xl transition-all duration-300 border-0 shadow-lg bg-white/80 backdrop-blur-sm hover:-translate-y-1 dark:bg-gray-800/80 dark:border dark:border-gray-700"
					cardPiece="cardCore"
				>
					<CardPiece className="pb-4" cardPiece="cardHeader">
						<div class="flex justify-between items-start mb-4">
							<div class="flex-1">
								<CardPiece
									className="text-2xl font-bold text-gray-900 group-hover:text-yellow-700 transition-colors mb-2 dark:text-gray-100 dark:group-hover:text-yellow-400"
									cardPiece="cardTitle"
								>
									{HousingNameLabels[housing[HousingFields.ID] as HousingID]}
								</CardPiece>
								<p class="text-sm text-gray-500 dark:text-gray-400 font-medium">
									{housing[HousingFields.CAMPUS_TYPE] + ' ' + housing[HousingFields.HOUSING_TYPE]}
								</p>
							</div>
							<div class="text-right ml-4">
								<div class="flex items-center space-x-2 mb-1">
									<Star class="h-6 w-6 fill-yellow-400 text-yellow-400" />
									<span class="text-2xl font-bold text-gray-900 dark:text-gray-100"
										>{convertRatingToHousingLabel(housing[HousingFields.OVERALL_RATING])}</span
									>
								</div>
								<p class="text-sm text-gray-500 dark:text-gray-400">
									{housing[HousingFields.REVIEW_COUNT]} reviews
								</p>
							</div>
						</div>
					</CardPiece>
					<CardPiece className="pt-0" cardPiece="cardContent">
						<a href={`${PUBLIC_SVELTE_HOUSING_ROOT}/housing/${housing[HousingFields.ID]}`}>
							<Button
								className="w-full bg-gradient-to-r from-yellow-500 to-yellow-600 hover:from-yellow-600 hover:to-yellow-700 text-white font-semibold py-3 rounded-xl shadow-lg hover:shadow-xl transition-all group"
							>
								View Reviews
								<ArrowRight class="h-4 w-4 ml-2 group-hover:translate-x-1 transition-transform" />
							</Button>
						</a>
					</CardPiece>
				</CardPiece>
			{/each}
		</div>
	</div>
</section>
