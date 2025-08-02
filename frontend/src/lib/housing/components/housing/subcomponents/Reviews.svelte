<script lang="ts">
	import { appState } from '$lib/housing/app-state.svelte'
	import { Calendar, Star, ThumbsDown, ThumbsUp } from '@lucide/svelte'
	import CardPiece from '../../templates/CardPiece.svelte'
	import { RatingCategoryIterable } from '$lib/housing/models/general'
	import { HousingFields } from '$lib/housing/constants/housing'
	import { ReviewFields } from '$lib/housing/models/reviews'
	import {
		convertDate,
		convertRatingToBase5,
		convertRatingToReviewLabel
	} from '$lib/housing/helpers/housing'

	let reviews = $derived(appState.getReviews())
</script>

<div class="space-y-6">
	{#each reviews as review}
		<CardPiece
			className="bg-white/80 backdrop-blur-sm shadow-xl border border-gray-100 rounded-2xl dark:bg-gray-800/80 dark:border-gray-700"
			cardPiece="cardCore"
		>
			<CardPiece className="pt-6" cardPiece="cardContent">
				<div class="flex justify-between items-start mb-6">
					<div class="flex items-center space-x-3">
						<div class="flex items-center">
							{#each Array(5) as _, i}
								<Star
									class="h-5 w-5 {i < convertRatingToBase5(review[HousingFields.OVERALL_RATING])
										? 'fill-yellow-400 text-yellow-400'
										: 'text-gray-300 dark:text-gray-600'}"
								/>
							{/each}
						</div>
						<span class="font-bold text-lg text-gray-900 dark:text-gray-400"
							>{convertRatingToReviewLabel(review[HousingFields.OVERALL_RATING])}/5</span
						>
					</div>
				</div>

				<div class="grid grid-cols-2 md:grid-cols-5 gap-3 mb-6">
					{#each RatingCategoryIterable as [category, label]}
						<div class="text-center p-2 bg-gray-50 rounded-lg dark:bg-gray-900/50">
							<div class="text-xs text-gray-600 mb-1 dark:text-gray-300 capitalize">
								{label}
							</div>
							<div class="font-bold text-gray-900 dark:text-gray-100">
								{convertRatingToReviewLabel(review[HousingFields.RATINGS][category])}/5
							</div>
						</div>
					{/each}
				</div>

				<p class="text-gray-700 mb-6 dark:text-gray-300 leading-relaxed text-lg">
					{review[ReviewFields.DESCRIPTION]}
				</p>

				<div
					class="flex items-center justify-between text-sm text-gray-500 dark:text-gray-400 pt-4 border-t border-gray-100 dark:border-gray-700"
				>
					<div class="text-sm text-gray-500">
						<div class="flex items-center space-x-2">
							<Calendar class="h-4 w-4" />
							<span class="font-medium">{convertDate(review[ReviewFields.DATE]!)}</span>
						</div>
					</div>
					<div class="flex items-center space-x-6">
						<button
							class="flex items-center space-x-2 hover:text-green-600 transition-colors dark:hover:text-green-400"
						>
							<ThumbsUp class="h-4 w-4" />
							<span class="font-medium">{review[ReviewFields.THUMBS_UP]}</span>
						</button>
						<button
							class="flex items-center space-x-2 hover:text-red-600 transition-colors dark:hover:text-red-400"
						>
							<ThumbsDown class="h-4 w-4" />
							<span class="font-medium">{review[ReviewFields.THUMBS_DOWN]}</span>
						</button>
					</div>
				</div>
			</CardPiece>
		</CardPiece>
	{/each}
</div>
