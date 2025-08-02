export type SortBy<T> = [T, SortDirection]

export type SortDirection = 'asc' | 'desc'

export type RatingsBrokenDown<T> = {
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

export const RatingCategoryDetails = [
	{
		key: RatingCategory.LIVING_CONDITIONS,
		label: 'Living Conditions',
		description: 'Cleanliness, maintenance, noise levels, room condition, AC/heating'
	},
	{
		key: RatingCategory.LOCATION,
		label: 'Location & Accessibility',
		description: 'Proximity to campus, bus stops, groceries, accessibility features'
	},
	{
		key: RatingCategory.AMENITIES,
		label: 'Amenities',
		description: 'Laundry, internet, study areas, gym, kitchen, parking, security'
	},
	{
		key: RatingCategory.VALUE,
		label: 'Value',
		description: 'Rent vs quality, utility inclusions, lease flexibility'
	},
	{
		key: RatingCategory.COMMUNITY,
		label: 'Community',
		description: 'Resident community, events, staff friendliness'
	}
]

export const RatingCategoryIterable = Object.entries(RatingCategoryDetails).map(([key, value]) => [
	key as RatingCategoryValue,
	value.label
]) as [RatingCategoryValue, string][]

export type SelectOptions = {
	value: string
	label: string
}
