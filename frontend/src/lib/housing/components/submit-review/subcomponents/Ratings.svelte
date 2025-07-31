<script lang="ts">
	import StarRating from '../StarRating.svelte'

	const ratingCategories = [
		{
			key: 'livingConditions',
			label: 'Living Conditions',
			description: 'Cleanliness, maintenance, noise levels, room condition, AC/heating'
		},
		{
			key: 'location',
			label: 'Location & Accessibility',
			description: 'Proximity to campus, bus stops, groceries, accessibility features'
		},
		{
			key: 'amenities',
			label: 'Amenities',
			description: 'Laundry, internet, study areas, gym, kitchen, parking, security'
		},
		{
			key: 'value',
			label: 'Value & Cost',
			description: 'Rent vs quality, utility inclusions, lease flexibility'
		},
		{
			key: 'community',
			label: 'Community & Social',
			description: 'Resident community, events, staff friendliness'
		}
	]

	let categoryRatings: Record<string, number> = {
		livingConditions: 0,
		location: 0,
		amenities: 0,
		value: 0,
		community: 0
	}
</script>

<div>
	<div
		class="mb-4 block leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-base font-semibold dark:text-gray-200"
	>
		Rate Each Category *
		<div class="space-y-4 mt-2">
			{#each ratingCategories as category}
				<div class="border rounded-lg p-4 dark:border-gray-700">
					<div class="flex justify-between items-start mb-2">
						<div>
							<h4 class="font-medium dark:text-gray-200">{category.label}</h4>
							<p class="text-sm text-gray-600 dark:text-gray-400">
								{category.description}
							</p>
						</div>
						<div class="flex items-center space-x-2">
							<StarRating
								rating={categoryRatings[category.key]}
								action={(r) => (categoryRatings[category.key] = r)}
							/>
							<span
								class="text-sm font-medium w-8 dark:text-gray-300 transition-opacity duration-200"
								style="opacity: {categoryRatings[category.key] > 0 ? 1 : 0}"
							>
								{categoryRatings[category.key]}/5
							</span>
						</div>
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
