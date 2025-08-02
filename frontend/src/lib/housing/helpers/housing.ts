import { PUBLIC_HOUSING_MAX_CHARS, PUBLIC_HOUSING_MIN_CHARS } from '$env/static/public'
import { appState } from '../app-state.svelte'
import type { HousingID } from '../models/housing'
import type { Review, ReviewRating, ReviewRatings, WriteReviewRatings } from '../models/reviews'
import { bindNumber } from './utils'

export function convertRatingToBase5(rating: number): number {
	return bindNumber(rating, 0, 500) / 100.0
}

export function convertBase5ToRating(base5: number): ReviewRating | 0 {
	return (bindNumber(base5, 0, 5) * 100.0) as ReviewRating
}

export function convertRatingToHousingLabel(rating: number): string {
	return convertRatingToBase5(rating).toFixed(2)
}

export function convertRatingToReviewLabel(rating: number): string {
	return convertRatingToBase5(rating).toFixed(0)
}

export function convertCost(costMin: number, costMax: number): string {
	const safeMin = Math.min(costMin, costMax)
	const safeMax = Math.max(costMin, costMax)

	if (costMax == costMin) {
		return 'Around $' + bindNumber(safeMin, 1, 255) + ',000 per year'
	}

	return (
		'Around $' +
		bindNumber(safeMin, 1, 255) +
		',000 - $' +
		bindNumber(safeMax, 1, 255) +
		',000 per year'
	)
}

export function walkToWALC(address: string): string {
	return `https://www.google.com/maps/dir/?api=1&origin=${encodeURIComponent(address)}&destination=WALC,+West+Lafayette,+IN&travelmode=walking`
}

export function convertDate(datePreFormat: string): string {
	const date = new Date(datePreFormat)

	return date.toLocaleDateString('en-US', { month: 'long', year: 'numeric' })
}

export function validatePayload(
	overall_rating: ReviewRating | 0,
	ratings: WriteReviewRatings,
	description: string
): Review | undefined {
	const housing_id = appState.getWriteReviewHousing()

	if (!housing_id) {
		appState.setPostError('Invalid housing')
		return
	}

	if (overall_rating === 0) {
		appState.setPostError('Invalid overall rating')
		return
	}

	for (const [, value] of Object.entries(ratings)) {
		if (value === 0) {
			appState.setPostError('Invalid sub-rating')
			return
		}
	}

	if (
		description.length < Number(PUBLIC_HOUSING_MIN_CHARS) ||
		description.length > Number(PUBLIC_HOUSING_MAX_CHARS)
	) {
		appState.setPostError('Invalid description')
		return
	}

	return {
		housing_id: appState.getWriteReviewHousing() as HousingID,
		overall_rating: overall_rating as ReviewRating,
		ratings: ratings as ReviewRatings,
		description: description
	}
}
