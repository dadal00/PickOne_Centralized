<script lang="ts">
	import { ArrowRight, MapPin, Star } from '@lucide/svelte'
	import CardPiece from '../templates/CardPiece.svelte'
	import Button from '../templates/Button.svelte'
	import { PUBLIC_SVELTE_HOUSING_ROOT } from '$env/static/public'

	const { housing } = $props()
</script>

<CardPiece
	className="bg-white/80 backdrop-blur-sm rounded-2xl shadow-xl border border-gray-100 overflow-hidden mb-8 dark:bg-gray-800/80 dark:border-gray-700"
	cardPiece="cardCore"
>
	<CardPiece className="p-8" cardPiece="cardContent">
		<div class="flex flex-col md:flex-row md:justify-between md:items-start gap-6">
			<div class="flex-1">
				<h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100 mb-3">{housing.name}</h1>
				<p class="text-xl text-gray-600 dark:text-gray-300 mb-4">{housing.type}</p>
				<div class="flex items-center space-x-6 text-gray-600 dark:text-gray-300 mb-6">
					<div class="flex items-center space-x-2">
						<MapPin class="h-5 w-5" />
						<span class="font-medium">{housing.walkTime}</span>
					</div>
					<span class="font-semibold text-lg">{housing.priceRange}</span>
				</div>
				<p class="text-gray-600 dark:text-gray-300">{housing.address}</p>
			</div>
			<div class="text-center md:text-right rounded-2xl p-6 min-w-[200px]">
				<div class="flex items-center justify-center md:justify-end space-x-3 mb-3">
					<Star class="h-10 w-10 fill-yellow-400 text-yellow-400" />
					<span class="text-4xl font-bold text-gray-900 dark:text-gray-100">{housing.rating}</span>
				</div>
				<p class="text-gray-600 dark:text-gray-300 mb-6 font-medium">
					{housing.reviewCount} reviews
				</p>
				<a href={PUBLIC_SVELTE_HOUSING_ROOT + '/submit-review'}>
					<Button
						className="bg-gradient-to-r from-yellow-500 to-yellow-600 hover:from-yellow-600 hover:to-yellow-700 text-white font-semibold px-6 py-3 rounded-xl shadow-lg hover:shadow-xl transition-all"
					>
						Write a Review
						<ArrowRight class="h-4 w-4 ml-2" />
					</Button>
				</a>
			</div>
		</div>
	</CardPiece>
</CardPiece>
