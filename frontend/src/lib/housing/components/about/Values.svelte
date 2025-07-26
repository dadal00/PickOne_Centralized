<script lang="ts">
	import CardPiece from '../templates/CardPiece.svelte'

	const team = [
		{
			role: 'Mission',
			description:
				'To help Purdue students make informed housing decisions through honest, peer-to-peer reviews.'
		},
		{
			role: 'Vision',
			description:
				'Creating a transparent housing community where every Boilermaker finds their perfect home.'
		},
		{
			role: 'Values',
			description:
				'Honesty, community, accessibility, and student empowerment guide everything we do.'
		}
	]
</script>

<div class="mb-20">
	<h2 class="text-3xl font-bold text-gray-900 text-center mb-12 dark:text-gray-100">
		Our Foundation
	</h2>
	<div class="grid grid-cols-1 md:grid-cols-3 gap-8">
		{#each team as item (item.role)}
			<CardPiece
				className="bg-white/80 backdrop-blur-sm shadow-xl border border-gray-100 rounded-2xl text-center hover:shadow-2xl transition-shadow dark:bg-gray-800/80 dark:border-gray-700"
				cardPiece="cardCore"
			>
				<CardPiece cardPiece="cardHeader">
					<CardPiece
						className="text-2xl font-bold text-yellow-600 dark:text-yellow-400"
						cardPiece="cardTitle">{item.role}</CardPiece
					>
				</CardPiece>
				<CardPiece cardPiece="cardContent">
					<p class="text-gray-700 dark:text-gray-300 leading-relaxed">{item.description}</p>
				</CardPiece>
			</CardPiece>
		{/each}
	</div>
</div>
