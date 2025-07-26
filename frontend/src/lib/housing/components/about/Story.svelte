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
				cardPiece="cardTitle">Our Story</CardPiece
			>
		</CardPiece>
		<CardPiece
			className="text-lg text-gray-700 dark:text-gray-300 leading-relaxed space-y-6"
			cardPiece="cardContent"
		>
			<p>
				As Purdue students, we know how challenging it can be to choose the right place to live...
			</p>
			<p>That's why we created RateMyPWLHousing - to give students a platform...</p>
			<p>Our platform focuses on what matters most to students...</p>
		</CardPiece>
	</CardPiece>
</div>
