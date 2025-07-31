<script lang="ts">
	import CardPiece from '$lib/housing/components/templates/CardPiece.svelte'
	import Housing from './subcomponents/Housing.svelte'
	import Title from './subcomponents/Title.svelte'
	import OverallRating from './subcomponents/OverallRating.svelte'
	import Ratings from './subcomponents/Ratings.svelte'
	import Description from './subcomponents/Description.svelte'
	import Submit from './subcomponents/Submit.svelte'

	const handleSubmit = async () => {
		alert('Review submitted successfully! Thank you for helping fellow students.')
	}
</script>

<CardPiece className="dark:bg-gray-800 dark:border-gray-700" cardPiece="cardCore">
	<Title />
	<CardPiece cardPiece="cardContent">
		<form on:submit|preventDefault={handleSubmit} class="space-y-6">
			<Housing />
			<OverallRating />
			<Ratings />
			<Description />
			<Submit />
		</form>
	</CardPiece>
</CardPiece>
