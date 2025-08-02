<script lang="ts">
	import StarRating from '../StarRating.svelte'
	import { RatingCategoryDetails } from '$lib/housing/models/general'
	import { convertBase5ToRating, convertRatingToBase5 } from '$lib/housing/helpers/housing'
	import type { WriteReviewRatings } from '$lib/housing/models/reviews'

	let { ratings = $bindable() } = $props<{
		ratings: WriteReviewRatings
	}>()
</script>

<div>
	<div
		class="mb-4 block leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-base font-semibold dark:text-gray-200"
	>
		Rate Each Category *
		<div class="space-y-4 mt-2">
			{#each RatingCategoryDetails as category}
				<div class="border rounded-lg p-4 dark:border-gray-700">
					<div class="flex justify-between items-start mb-2">
						<div>
							<h4 class="font-medium dark:text-gray-200">{category.label}</h4>
							<p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
								{category.description}
							</p>
						</div>
						<div class="flex items-center space-x-2">
							<StarRating
								rating={convertRatingToBase5(ratings[category.key])}
								action={(r) => (ratings[category.key] = convertBase5ToRating(r))}
							/>
							<span
								class="text-sm font-medium w-8 dark:text-gray-300 transition-opacity duration-200"
								style="opacity: {ratings[category.key] > 0 ? 1 : 0}"
							>
								{convertRatingToBase5(ratings[category.key])}/5
							</span>
						</div>
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
