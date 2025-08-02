import { PUBLIC_MEILI_URL, PUBLIC_PAGE_SIZE } from '$env/static/public'
import { env } from '$env/dynamic/public'
import { Meilisearch } from 'meilisearch'
import { appState } from './app-state.svelte'
import { ReviewFields, type Review, type ReviewRating, type ReviewSortBy } from './models/reviews'
import {
	type CampusType,
	type CostSymbol,
	type Housing,
	type HousingID,
	type HousingSortBy,
	type HousingType
} from './models/housing'
import { HousingFields, HousingTableName } from './constants/housing'

const client = new Meilisearch({
	host: PUBLIC_MEILI_URL,
	apiKey: env.PUBLIC_MEILI_KEY,
	requestInit: {
		credentials: 'include'
	}
})

export async function housingSearch(
	query: string,
	housingType: HousingType | '',
	campusType: CampusType | '',
	costSymbol: CostSymbol | '',
	housingSortBy: HousingSortBy,
	offset: number
) {
	const filters: string[] = []

	if (housingType !== '') {
		filters.push(`${HousingFields.HOUSING_TYPE} = ${housingType}`)
	}

	if (campusType !== '') {
		filters.push(`${HousingFields.CAMPUS_TYPE} = ${campusType}`)
	}

	if (costSymbol !== '') {
		filters.push(`${HousingFields.COST_SYMBOL} = ${costSymbol}`)
	}

	const sortBy = [`${housingSortBy[0]}:${housingSortBy[1]}`]

	const response = await client.index(HousingTableName).search(query, {
		filter: filters,
		sort: sortBy,
		limit: Number(PUBLIC_PAGE_SIZE),
		offset: offset,
		attributesToHighlight: [HousingFields.ID],
		highlightPreTag: '<mark>',
		highlightPostTag: '</mark>'
	})

	appState.setHousingHits(response.hits as Housing[])
	appState.setTotalHits(response.estimatedTotalHits)
}

export async function reviewSearch(
	housingId: HousingID,
	query: string,
	overallRating: ReviewRating | 0,
	reviewSortBy: ReviewSortBy,
	offset: number
) {
	const filters = [
		...(overallRating !== 0 ? [`${HousingFields.OVERALL_RATING} = ${overallRating}`] : [])
	]

	const sortBy = [`${reviewSortBy[0]}:${reviewSortBy[1]}`]

	const response = await client.index(housingId).search(query, {
		filter: filters,
		sort: sortBy,
		limit: Number(PUBLIC_PAGE_SIZE),
		offset: offset,
		attributesToHighlight: [ReviewFields.DESCRIPTION],
		highlightPreTag: '<mark>',
		highlightPostTag: '</mark>'
	})

	appState.setReviewHits(response.hits as Review[])
	appState.setTotalHits(response.estimatedTotalHits)
}
