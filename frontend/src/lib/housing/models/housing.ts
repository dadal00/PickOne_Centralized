import { toFilterOptions, toSelectOptions } from '../helpers/utils'
import type { RatingsBrokenDown, SelectOptions, SortBy } from './general'
import {
	CampusTypeIterable,
	CostSymbolIterable,
	HousingFields,
	housingSortDirectionLabels,
	housingSortLabels,
	HousingTypeIterable,
	type HousingIDIterable,
	type HousingSortable
} from '../constants/housing'

// Housing ID type derived from ID bank
export type HousingID = (typeof HousingIDIterable)[number]

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

export type HousingSortBy = SortBy<HousingSortable>
export const defaultHousingSortBy: HousingSortBy = [HousingFields.REVIEW_COUNT, 'desc']

export type HousingRatings = RatingsBrokenDown<number>

export const housingSortSelect: SelectOptions[] = toSelectOptions(housingSortLabels)

export const housingSortDirectionSelect: SelectOptions[] = toSelectOptions(
	housingSortDirectionLabels
)

export type HousingType = (typeof HousingTypeIterable)[number]
export const housingFilterTypeSelect: SelectOptions[] = toSelectOptions(
	toFilterOptions(HousingTypeIterable)
)

export type CampusType = (typeof CampusTypeIterable)[number]
export const housingFilterCampusSelect: SelectOptions[] = toSelectOptions(
	toFilterOptions(CampusTypeIterable)
)

export type CostSymbol = (typeof CostSymbolIterable)[number]
export const housingFilterCostSelect: SelectOptions[] = toSelectOptions(
	toFilterOptions(CostSymbolIterable)
)
