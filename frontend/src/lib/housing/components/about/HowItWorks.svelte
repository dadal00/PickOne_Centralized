<script lang="ts">
	import CardPiece from '../templates/CardPiece.svelte'
</script>

<div class="mb-20">
	<CardPiece
		className="bg-white/80 backdrop-blur-sm shadow-xl border border-gray-100 rounded-2xl dark:bg-gray-800/80 dark:border-gray-700"
		cardPiece="cardCore"
	>
		<CardPiece cardPiece="cardHeader">
			<CardPiece
				className="text-3xl font-bold text-gray-900 text-center dark:text-gray-100"
				cardPiece="cardTitle">How It Works</CardPiece
			>
		</CardPiece>
		<CardPiece className="space-y-8" cardPiece="cardContent">
			<div class="grid grid-cols-1 md:grid-cols-3 gap-8">
				{#each ['Search Housing', 'Read Reviews', 'Share Your Experience'] as title, i}
					<div class="text-center">
						<div class="flex justify-center mb-4">
							<div
								class="w-12 h-12 bg-gradient-to-br from-yellow-400 to-yellow-600 rounded-full flex items-center justify-center text-white font-bold text-xl"
							>
								{i + 1}
							</div>
						</div>
						<h3 class="text-xl font-bold text-gray-900 mb-2 dark:text-gray-100">{title}</h3>
						<p class="text-gray-600 dark:text-gray-400">[...]</p>
					</div>
				{/each}
			</div>
		</CardPiece>
	</CardPiece>
</div>
