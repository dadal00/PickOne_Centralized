import { toSelectOptions } from '../helpers/utils'
import type { SelectOptions, SortDirection } from '../models/general'
import { type CampusType, type HousingID } from '../models/housing'

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

export type HousingSortable =
	| (typeof HousingFields)['OVERALL_RATING']
	| (typeof HousingFields)['WALK_TIME_MINS']
	| (typeof HousingFields)['COST_MIN']
	| (typeof HousingFields)['COST_MAX']
	| (typeof HousingFields)['REVIEW_COUNT']

/*
	Searchable index for housing
	- User searches are matched by housing labels
*/
export const HousingTableName = 'housing'

export const housingSortLabels: Record<HousingSortable, string> = {
	[HousingFields.OVERALL_RATING]: 'Rating',
	[HousingFields.WALK_TIME_MINS]: 'Walking Distance to Campus',
	[HousingFields.COST_MIN]: 'Lowest Cost',
	[HousingFields.COST_MAX]: 'Highest Cost',
	[HousingFields.REVIEW_COUNT]: 'Number of Reviews'
}

export const housingSortDirectionLabels: Record<SortDirection, string> = {
	asc: 'Low to High',
	desc: 'High to Low'
}

export const HousingTypeIterable = ['Dorm', 'Apartment'] as const

export const CostSymbolIterable = ['$', '$$', '$$$'] as const

export const CampusTypeIterable = ['On-Campus', 'Off-Campus'] as const

// Display Labels for Housing
export const HousingNameLabels: Record<HousingID | CampusType, string> = {
	// On-Campus
	[CampusTypeIterable[0]]: CampusTypeIterable[0],
	'3rd-and-west': '3rd and West',
	aspire: 'Aspire at Discovery Park',
	'benchmark-ii': 'Benchmark II',
	'cary-quad': 'Cary Quadrangle',
	earhart: 'Earhart Hall',
	'first-street': 'First Street Towers',
	frieda: 'Frieda Parker Hall',
	fuse: 'Fuse Apartments',
	'grant-333': 'Grant Street Station 333',
	harrison: 'Harrison Hall',
	hawkins: 'Hawkins Hall',
	hilltop: 'Hilltop Apartments',
	hillenbrand: 'Hillenbrand Hall',
	'honors-college-residences': 'Honors College Residences',
	'mc-cutcheon': 'McCutcheon Hall',
	meredith: 'Meredith Hall',
	'meredith-south': 'Meredith South',
	owen: 'Owen Hall',
	provenance: 'Provenance Apartments',
	'russell-414': '414 Russell Street',
	shreve: 'Shreve Hall',
	'steely-410': '410 Steely Street',
	tarkington: 'Tarkington Hall',
	'waldron-125': '125 Waldron Street',
	'waldron-19': '19 Waldron Street',
	'waldron-square': 'Waldron Square',
	wiley: 'Wiley Hall',
	winifred: 'Winifred Parker Hall',
	windsor: 'Windsor Halls',

	// Off-Campus
	[CampusTypeIterable[1]]: CampusTypeIterable[1],
	allight: 'Alight West Lafayette',
	chauncey: 'Chauncey Square Apartments',
	hub: 'Hub on Campus',
	lark: 'Lark West Lafayette',
	morris: 'Morris Rentals',
	redpoint: 'Redpoint West Lafayette',
	rise: 'Rise on Chauncey',
	river: 'River Market Apartments',
	verve: 'Verve West Lafayette'
}

// Same process as housing but for the housing select in writing reviews
export const writeReviewHousingSelect: SelectOptions[] = toSelectOptions(HousingNameLabels)

// URL Compatible IDs for Housing
export const HousingIDIterable = [
	// On Campus
	'cary-quad',
	'mc-cutcheon',
	'tarkington',
	'wiley',
	'owen',
	'shreve',
	'earhart',
	'harrison',
	'hillenbrand',
	'meredith',
	'meredith-south',
	'windsor',
	'first-street',
	'hilltop',
	'winifred',
	'frieda',
	'hawkins',
	'fuse',
	'aspire',
	'3rd-and-west',
	'benchmark-ii',
	'grant-333',
	'provenance',
	'russell-414',
	'steely-410',
	'waldron-125',
	'waldron-19',
	'waldron-square',
	'honors-college-residences',

	// Off Campus
	'hub',
	'rise',
	'chauncey',
	'lark',
	'allight',
	'redpoint',
	'verve',
	'river',
	'morris'
] as const
