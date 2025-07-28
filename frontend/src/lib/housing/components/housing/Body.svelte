<script lang="ts">
	import { page } from '$app/state'
	import { appState } from '$lib/housing/AppState.svelte'
	import { Calendar, Star, ThumbsDown, ThumbsUp } from '@lucide/svelte'
	import CardPiece from '../templates/CardPiece.svelte'
	import TabPiece from '../templates/TabPiece.svelte'
	import Progress from '../templates/Progress.svelte'
	import SearchComponent from '$lib/housing/components/search/Search.svelte'

	let reviews = $derived(appState.getReviews(page.params.id))
	let activeTabValue = $state('overview')

	const ratingCategories = [
		{
			key: 'livingConditions',
			label: 'Living Conditions',
			value: appState.getHousingDetails(page.params.id).ratings.livingConditions
		},
		{
			key: 'location',
			label: 'Location & Access',
			value: appState.getHousingDetails(page.params.id).ratings.location
		},
		{
			key: 'amenities',
			label: 'Amenities',
			value: appState.getHousingDetails(page.params.id).ratings.amenities
		},
		{
			key: 'value',
			label: 'Value & Cost',
			value: appState.getHousingDetails(page.params.id).ratings.value
		},
		{
			key: 'community',
			label: 'Community',
			value: appState.getHousingDetails(page.params.id).ratings.community
		}
	]
</script>

<div class="space-y-8">
	<TabPiece
		className="grid w-full grid-cols-2 bg-white/80 backdrop-blur-sm rounded-xl shadow-lg border border-gray-100 p-1 dark:bg-gray-800/80 dark:border-gray-700"
		tabPiece="list"
	>
		<TabPiece
			className="rounded-lg font-semibold"
			tabValue="overview"
			tabPiece="trigger"
			activeClasses="bg-gradient-to-r from-yellow-500 to-yellow-600 text-white"
			bind:activeTabValue>Overview</TabPiece
		>
		<TabPiece
			className="rounded-lg font-semibold"
			tabValue="reviews"
			tabPiece="trigger"
			activeClasses="bg-gradient-to-r from-yellow-500 to-yellow-600 text-white"
			bind:activeTabValue>Reviews</TabPiece
		>
	</TabPiece>

	<TabPiece tabPiece="content" className="space-y-8">
		{#if activeTabValue === 'overview'}
			<CardPiece
				className="bg-white/80 backdrop-blur-sm shadow-xl border border-gray-100 rounded-2xl dark:bg-gray-800/80 dark:border-gray-700"
				cardPiece="cardCore"
			>
				<CardPiece cardPiece="cardHeader">
					<CardPiece
						cardPiece="cardTitle"
						className="text-2xl font-bold text-gray-900 dark:text-gray-100"
						>Rating Breakdown</CardPiece
					>
				</CardPiece>
				<CardPiece cardPiece="cardContent">
					<div class="space-y-6">
						{#each ratingCategories as category}
							<div class="flex items-center space-x-6">
								<div class="w-40 text-sm font-semibold text-gray-900 dark:text-gray-100">
									{category.label}
								</div>
								<div class="flex-1">
									<Progress
										value={category.value * 20}
										className="h-3 bg-gray-200 dark:bg-gray-700"
									/>
								</div>
								<div class="w-16 text-right">
									<span class="text-lg font-bold text-gray-900 dark:text-gray-100"
										>{category.value.toFixed(1)}</span
									>
									<span class="text-sm text-gray-500 dark:text-gray-400">/5</span>
								</div>
							</div>
						{/each}
					</div>
				</CardPiece>
			</CardPiece>
		{:else if activeTabValue === 'reviews'}
			<SearchComponent />
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
												class="h-5 w-5 {i < review.rating
													? 'fill-yellow-400 text-yellow-400'
													: 'text-gray-300 dark:text-gray-600'}"
											/>
										{/each}
									</div>
									<span class="font-bold text-lg text-gray-900 dark:text-gray-400"
										>{review.rating}/5</span
									>
								</div>
							</div>

							<div class="grid grid-cols-2 md:grid-cols-5 gap-3 mb-6">
								{#each Object.entries(review.ratings) as [key, value]}
									<div class="text-center p-2 bg-gray-50 rounded-lg dark:bg-gray-900/50">
										<div class="text-xs text-gray-600 mb-1 dark:text-gray-300 capitalize">
											{key.replace(/([A-Z])/g, ' $1').trim()}
										</div>
										<div class="font-bold text-gray-900 dark:text-gray-100">{value}/5</div>
									</div>
								{/each}
							</div>

							<p class="text-gray-700 mb-6 dark:text-gray-300 leading-relaxed text-lg">
								{review.comment}
							</p>

							<div
								class="flex items-center justify-between text-sm text-gray-500 dark:text-gray-400 pt-4 border-t border-gray-100 dark:border-gray-700"
							>
								<div class="text-sm text-gray-500">
									<div class="flex items-center space-x-2">
										<Calendar class="h-4 w-4" />
										<span class="font-medium">{review.semester}</span>
									</div>
								</div>
								<div class="flex items-center space-x-6">
									<button
										class="flex items-center space-x-2 hover:text-green-600 transition-colors dark:hover:text-green-400"
									>
										<ThumbsUp class="h-4 w-4" />
										<span class="font-medium">{review.helpful}</span>
									</button>
									<button
										class="flex items-center space-x-2 hover:text-red-600 transition-colors dark:hover:text-red-400"
									>
										<ThumbsDown class="h-4 w-4" />
										<span class="font-medium">{review.notHelpful}</span>
									</button>
								</div>
							</div>
						</CardPiece>
					</CardPiece>
				{/each}
			</div>
		{/if}
	</TabPiece>
</div>
