<script lang="ts">
	import CardPiece from '$lib/housing/components/templates/CardPiece.svelte'
	import Housing from './subcomponents/Housing.svelte'
	import Title from './subcomponents/Title.svelte'
	import OverallRating from './subcomponents/OverallRating.svelte'
	import Ratings from './subcomponents/Ratings.svelte'
	import Description from './subcomponents/Description.svelte'
	import Submit from './subcomponents/Submit.svelte'
	import type { Review, ReviewRating, WriteReviewRatings } from '$lib/housing/models/reviews'
	import { PUBLIC_HOUSING_BACKEND_PATH, PUBLIC_SVELTE_HOUSING_ROOT } from '$env/static/public'
	import { appState } from '$lib/housing/app-state.svelte'
	import { goto } from '$app/navigation'
	import { onDestroy, onMount } from 'svelte'
	import { validatePayload } from '$lib/housing/helpers/housing'

	let overallRating: ReviewRating | 0 = $state(0)
	let ratings: WriteReviewRatings = $state({
		living_conditions: 0,
		location: 0,
		amenities: 0,
		value: 0,
		community: 0
	})
	let description = $state('')

	let error: string = $derived(appState.getPostError())

	const handleSubmit = async () => {
		if (appState.getLimited()) {
			return
		}

		appState.nowLimited()

		const review: Review | undefined = validatePayload(overallRating, ratings, description)

		if (!review) {
			return
		}

		const response = await fetch(PUBLIC_HOUSING_BACKEND_PATH + '/post-review', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			credentials: 'include',
			body: JSON.stringify(review)
		})

		if (!response.ok) {
			appState.setPostError((await response.text()).slice(0, 50))

			throw new Error(`HTTP error! status: ${response.status}`)
		}

		alert('Review submitted successfully!')
		goto(PUBLIC_SVELTE_HOUSING_ROOT + '/search')
	}

	onMount(() => {
		appState.setPostError('')
	})

	onDestroy(() => {
		appState.setPostError('')
	})
</script>

<CardPiece className="dark:bg-gray-800 dark:border-gray-700" cardPiece="cardCore">
	<Title />
	<CardPiece cardPiece="cardContent">
		<form onsubmit={handleSubmit} class="space-y-6">
			{#if error != ''}
				<p class="text-red-600 text-sm font-medium text-center">{error}</p>
			{/if}
			<Housing />
			<OverallRating bind:overallRating />
			<Ratings bind:ratings />
			<Description bind:description />
			<Submit />
		</form>
	</CardPiece>
</CardPiece>
