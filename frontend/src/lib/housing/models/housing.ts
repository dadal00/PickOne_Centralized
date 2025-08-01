import { toFilterOptions, toSelectOptions } from '../helpers/utils'
import type { RatingsBrokenDown, SelectOptions, SortBy, SortDirection } from './general'
import type { HousingIDIterable } from './housingNames'

// Housing ID type derived from ID bank
export type HousingID = (typeof HousingIDIterable)[number]

export const HousingFields = {
	ID: 'id',
	OVERALL_RATING: 'overall_rating',
	RATINGS: 'ratings',
	REVIEW_COUNT: 'review_count',
	HOUSING_TYPE: 'housing_type',
	CAMPUS_TYPE: 'campus_type',
	WALK_TIME_MINS: 'walk_time_mins',
	COST_MIN: 'cost_min',
	COST_MAX: 'cost_max',
	ADDRESS: 'address',
	COST_SYMBOL: 'cost_symbol'
} as const

// Housing struct
export type Housing = {
	[HousingFields.ID]: HousingID
	// 1-500 => /100 => 0.00 - 5.00
	[HousingFields.OVERALL_RATING]: number
	[HousingFields.RATINGS]: HousingRatings
	[HousingFields.REVIEW_COUNT]: number
	[HousingFields.HOUSING_TYPE]: HousingType
	[HousingFields.CAMPUS_TYPE]: CampusType
	[HousingFields.WALK_TIME_MINS]: number
	// 1-255 => 1k - 255k per year
	[HousingFields.COST_MIN]: number
	// 1-255 => 1k - 255k per year
	[HousingFields.COST_MAX]: number
	[HousingFields.COST_SYMBOL]: CostSymbol
	[HousingFields.ADDRESS]: string
}

// Query used to search for housing hits
export type HousingQuery = {
	query: string
	[HousingFields.HOUSING_TYPE]: HousingType | ''
	[HousingFields.CAMPUS_TYPE]: CampusType | ''
	[HousingFields.COST_SYMBOL]: CostSymbol | ''
	sortBy: HousingSortBy
	offset: number
}

// Sortable fields when searching housing
export type HousingSortable =
	| (typeof HousingFields)['OVERALL_RATING']
	| (typeof HousingFields)['WALK_TIME_MINS']
	| (typeof HousingFields)['COST_MIN']
	| (typeof HousingFields)['COST_MAX']
	| (typeof HousingFields)['REVIEW_COUNT']
// Sort parameters struct for search
export type HousingSortBy = SortBy<HousingSortable>
// Default sorting is by most number of reviews first
export const defaultHousingSortBy: HousingSortBy = [HousingFields.REVIEW_COUNT, 'desc']

/*
	Searchable index for housing
	- User searches are matched by housing labels
*/
export const HousingTableName = 'housing'

// Sub-rating type for housing
export type HousingRatings = RatingsBrokenDown<number>

/*
	Converting types into UI usable values
	- first convert into lables
	- then convert into actual select options
*/
// Sorting labels used for UI
export const housingSortLabels: Record<HousingSortable, string> = {
	[HousingFields.OVERALL_RATING]: 'Rating',
	[HousingFields.WALK_TIME_MINS]: 'Walking Distance to Campus',
	[HousingFields.COST_MIN]: 'Lowest Cost',
	[HousingFields.COST_MAX]: 'Highest Cost',
	[HousingFields.REVIEW_COUNT]: 'Number of Reviews'
}
// Converted sort options into UI usable
export const housingSortSelect: SelectOptions[] = toSelectOptions(housingSortLabels)

// Same process as housing sort but for direction of sort
export const housingSortDirectionLabels: Record<SortDirection, string> = {
	asc: 'Low to High',
	desc: 'High to Low'
}
export const housingSortDirectionSelect: SelectOptions[] = toSelectOptions(
	housingSortDirectionLabels
)

/*
	Iterables created to derive types + UI usable options
	- dorm vs apartment
	- on vs off campus
	- how expensive it is
*/
// Define the constant values
export const HousingTypeIterable = ['Dorm', 'Apartment'] as const
// Derive a type to use
export type HousingType = (typeof HousingTypeIterable)[number]
// Derive a UI select compatible list
export const housingFilterTypeSelect: SelectOptions[] = toSelectOptions(
	toFilterOptions(HousingTypeIterable)
)

// Same process as for housing type
export const CampusTypeIterable = ['On-Campus', 'Off-Campus'] as const
export type CampusType = (typeof CampusTypeIterable)[number]
export const housingFilterCampusSelect: SelectOptions[] = toSelectOptions(
	toFilterOptions(CampusTypeIterable)
)

// Same process as for housing type
export const CostSymbolIterable = ['$', '$$', '$$$'] as const
export type CostSymbol = (typeof CostSymbolIterable)[number]
export const housingFilterCostSelect: SelectOptions[] = toSelectOptions(
	toFilterOptions(CostSymbolIterable)
)
