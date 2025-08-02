import { toSelectOptions } from '../helpers/utils'
import type { RatingsBrokenDown, SelectOptions, SortBy } from './general'
import type { HousingID } from './housing'
import { HousingFields } from '../constants/housing'
import { ReviewFields, ReviewRatingIterable, reviewSortLabels } from '../constants/reviews'

// Review struct
export type Review = {
	[HousingFields.ID]?: string
	[ReviewFields.HOUSING_ID]: HousingID
	// 100, 200, 300, 400, 500 => /100 => 1, 2, 3, 4, 5
	[HousingFields.OVERALL_RATING]: ReviewRating
	// 100, 200, 300, 400, 500 => /100 => 1, 2, 3, 4, 5
	[HousingFields.RATINGS]: ReviewRatings
	[ReviewFields.DATE]?: string
	[ReviewFields.DESCRIPTION]: string
	[ReviewFields.THUMBS_UP]?: number
	[ReviewFields.THUMBS_DOWN]?: number
}

/*
	Query used to search for reviews
	- Search indexes are named by housing id
	- As a result, when searching, we pass in
	  an additional housing id
	- This is obtained by the url id only if 
	  it is a valid id from our housing name bank
*/
export type ReviewQuery = {
	query: string
	[HousingFields.OVERALL_RATING]: ReviewRating | 0
	sortBy: ReviewSortBy
	offset: number
}

/*
	Payload for updating thumbs up or down for reviews
	- uses a map by
	  {
		"uuid of review1": 'up',
		"uuid of review2": 'down',
		...
	  }
*/
// Using a union to restrict values
export type ThumbsDelta = 'up' | 'down'
// Hash map using uuid of the review as key
export type ThumbsDeltaMap = Record<string, ThumbsDelta>

export type ReviewRating = (typeof ReviewRatingIterable)[number]
// Typing the generic shared struct for sub ratings
export type ReviewRatings = RatingsBrokenDown<ReviewRating>

export type WriteReviewRatings = RatingsBrokenDown<ReviewRating | 0>

/*
	We use a type like this in case we want to expand the
	sortable fields in the future
	- define the sortable field(s)
	- create the generic sort by struct now typed
*/
// Redundant currently but matches general structure +
// allows for future addition
export type ReviewSortable = (typeof ReviewFields)['DATE']
// Typing generic struct for sorting
export type ReviewSortBy = SortBy<ReviewSortable>
// Default sorting is by most number of reviews first
export const defaultReviewSortBy: ReviewSortBy = [ReviewFields.DATE, 'desc']

export const reviewSortOptions: SelectOptions[] = toSelectOptions(reviewSortLabels)

export { ReviewFields }
