<script lang="ts">
	import { PUBLIC_SVELTE_HOUSING_ROOT } from '$env/static/public'
	import CardPiece from '../templates/CardPiece.svelte'
</script>

<div class="text-center">
	<CardPiece
		className="bg-gradient-to-br from-yellow-50 to-yellow-100 border border-yellow-200 rounded-2xl dark:from-yellow-900/20 dark:to-yellow-900/10 dark:border-yellow-800/50"
		cardPiece="cardCore"
	>
		<CardPiece className="py-12" cardPiece="cardContent">
			<h2 class="text-3xl font-bold text-gray-900 mb-6 dark:text-gray-100">Join Our Community</h2>
			<p class="text-lg text-gray-700 mb-8 max-w-2xl mx-auto dark:text-gray-300">
				Ready to help fellow Boilermakers? Start by sharing your experience or reading others.
			</p>
			<div class="flex flex-col sm:flex-row gap-4 justify-center">
				<a
					href={PUBLIC_SVELTE_HOUSING_ROOT + '/submit-review'}
					class="bg-gradient-to-r from-yellow-500 to-yellow-600 hover:from-yellow-600 hover:to-yellow-700 text-white font-semibold px-8 py-3 rounded-xl shadow-lg hover:shadow-xl transition-all"
					>Write a Review</a
				>
				<a
					href={PUBLIC_SVELTE_HOUSING_ROOT + '/search'}
					class="bg-gradient-to-r from-yellow-500 to-yellow-600 hover:from-yellow-600 hover:to-yellow-700 text-white font-semibold px-8 py-3 rounded-xl shadow-lg hover:shadow-xl transition-all"
					>Explore Housing</a
				>
			</div>
		</CardPiece>
	</CardPiece>
</div>
