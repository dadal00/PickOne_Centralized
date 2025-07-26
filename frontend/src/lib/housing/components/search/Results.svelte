<script lang="ts">
	import Button from '$lib/housing/components/templates/Button.svelte'
	import CardPiece from '$lib/housing/components/templates/CardPiece.svelte'
	import { Search, Star, Users } from '@lucide/svelte'
	import Badge from '$lib/housing/components/templates/Badge.svelte'
	import { appState } from '$lib/housing/AppState.svelte'
	import { PUBLIC_SVELTE_HOUSING_ROOT } from '$env/static/public'

	const housingData = $derived(appState.getAllHousing())

	const filteredHousing = $derived(housingData)
</script>

<h2 class="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-8">
	{filteredHousing.length} Housing Options Found
</h2>

<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
	{#each filteredHousing as housing}
		<CardPiece
			className="group overflow-hidden hover:shadow-2xl transition-all duration-300 border-0 shadow-lg bg-white/80 backdrop-blur-sm hover:-translate-y-1 dark:bg-gray-800/80 dark:border dark:border-gray-700"
			cardPiece="cardCore"
		>
			<CardPiece className="pb-4" cardPiece="cardHeader">
				<div class="flex justify-between items-start mb-4">
					<div class="flex-1">
						<div class="flex items-center justify-between mb-2">
							<CardPiece
								className="text-xl font-bold text-gray-900 group-hover:text-yellow-700 transition-colors dark:text-gray-100 dark:group-hover:text-yellow-400"
								cardPiece="cardTitle"
							>
								{housing.name}
							</CardPiece>
							<Badge
								className="bg-white/90 text-gray-900 font-semibold shadow-sm ml-2 dark:bg-gray-700 dark:text-gray-200"
							>
								{housing.priceRangeSymbol}
							</Badge>
						</div>
						<p class="text-sm text-gray-500 dark:text-gray-400 font-medium">{housing.type}</p>
					</div>
				</div>
				<div class="flex justify-between items-center">
					<div class="flex items-center space-x-2">
						<Star class="h-5 w-5 fill-yellow-400 text-yellow-400" />
						<span class="font-bold text-lg text-gray-900 dark:text-gray-100">{housing.rating}</span>
					</div>
					<div class="flex items-center space-x-2 text-sm text-gray-600 dark:text-gray-400">
						<Users class="h-4 w-4" />
						<span>{housing.reviewCount} reviews</span>
					</div>
				</div>
			</CardPiece>
			<CardPiece className="pt-0" cardPiece="cardContent">
				<a href={`${PUBLIC_SVELTE_HOUSING_ROOT}/housing/${housing.id}`}>
					<Button
						className="w-full bg-gradient-to-r from-yellow-500 to-yellow-600 hover:from-yellow-600 hover:to-yellow-700 text-white font-semibold py-3 rounded-xl shadow-lg hover:shadow-xl transition-all"
					>
						View Reviews
					</Button>
				</a>
			</CardPiece>
		</CardPiece>
	{/each}
</div>

{#if filteredHousing.length === 0}
	<div class="text-center py-16">
		<Search class="h-20 w-20 mx-auto text-gray-400 dark:text-gray-500 mb-6" />
		<h3 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-4">No housing found</h3>
		<p class="text-gray-600 dark:text-gray-400 text-lg">
			Try adjusting your search criteria or filters.
		</p>
	</div>
{/if}
