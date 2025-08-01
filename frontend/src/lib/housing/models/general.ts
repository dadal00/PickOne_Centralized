import type { ReviewRating } from './reviews'

export type SortBy<T> = [T, SortDirection]

export type SortDirection = 'asc' | 'desc'

export type RatingsBrokenDown<T = number | ReviewRating> = {
	[RatingCategory.LIVING_CONDITIONS]: T
	[RatingCategory.LOCATION]: T
	[RatingCategory.AMENITIES]: T
	[RatingCategory.VALUE]: T
	[RatingCategory.COMMUNITY]: T
}

export const RatingCategory = {
	LIVING_CONDITIONS: 'living_conditions',
	LOCATION: 'location',
	AMENITIES: 'amenities',
	VALUE: 'value',
	COMMUNITY: 'community'
} as const

export type RatingCategoryKey = keyof typeof RatingCategory
export type RatingCategoryValue = (typeof RatingCategory)[RatingCategoryKey]

export const RatingCategoryLabels: Record<RatingCategoryValue, string> = {
	[RatingCategory.LIVING_CONDITIONS]: 'Living Conditions',
	[RatingCategory.LOCATION]: 'Location',
	[RatingCategory.AMENITIES]: 'Amenities',
	[RatingCategory.VALUE]: 'Value',
	[RatingCategory.COMMUNITY]: 'Community'
}

export const RatingCategoryIterable = Object.entries(RatingCategoryLabels) as [
	RatingCategoryValue,
	string
][]

export type SelectOptions = {
	value: string
	label: string
}
