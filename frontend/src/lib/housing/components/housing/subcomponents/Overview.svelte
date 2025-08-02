<script lang="ts">
	import { RatingCategoryIterable } from '$lib/housing/models/general'
	import { type Housing } from '$lib/housing/models/housing'
	import { HousingFields } from '$lib/housing/constants/housing'
	import { convertRatingToBase5 } from '$lib/housing/helpers/housing'
	import CardPiece from '../../templates/CardPiece.svelte'
	import Progress from '../../templates/Progress.svelte'

	let { housing } = $props<{
		housing: Housing
	}>()
</script>

<CardPiece
	className="bg-white/80 backdrop-blur-sm shadow-xl border border-gray-100 rounded-2xl dark:bg-gray-800/80 dark:border-gray-700"
	cardPiece="cardCore"
>
	<CardPiece cardPiece="cardHeader">
		<CardPiece cardPiece="cardTitle" className="text-2xl font-bold text-gray-900 dark:text-gray-100"
			>Rating Breakdown</CardPiece
		>
	</CardPiece>
	<CardPiece cardPiece="cardContent">
		<div class="space-y-6">
			{#each RatingCategoryIterable as [category, label]}
				<div class="flex items-center space-x-6">
					<div class="w-40 text-sm font-semibold text-gray-900 dark:text-gray-100">
						{label}
					</div>
					<div class="flex-1">
						<Progress
							value={convertRatingToBase5(housing[HousingFields.RATINGS][category]) * 20}
							className="h-3 bg-gray-200 dark:bg-gray-700"
						/>
					</div>
					<div class="w-16 text-right">
						<span class="text-lg font-bold text-gray-900 dark:text-gray-100"
							>{convertRatingToBase5(housing[HousingFields.RATINGS][category]).toFixed(1)}</span
						>
						<span class="text-sm text-gray-500 dark:text-gray-400">/5</span>
					</div>
				</div>
			{/each}
		</div>
	</CardPiece>
</CardPiece>
