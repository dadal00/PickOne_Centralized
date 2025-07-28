export type HousingID =
	// On Campus
	| 'cary-quad'
	| 'mc-cutcheon'
	| 'tarkington'
	| 'wiley'
	| 'owen'
	| 'shreve'
	| 'earhart'
	| 'harrison'
	| 'hillenbrand'
	| 'meredith'
	| 'meredith-south'
	| 'windsor'
	| 'first-street'
	| 'hilltop'
	| 'winifred'
	| 'frieda'
	| 'hawkins'
	| 'fuse'
	| 'aspire'
	| '3rd-and-west'
	| 'benchmark-ii'
	| 'grant-333'
	| 'provenance'
	| 'russell-414'
	| 'steely-410'
	| 'waldron-125'
	| 'waldron-19'
	| 'waldron-square'
	| 'honors-college-residences'

	// Off Campus
	| 'hub'
	| 'rise'
	| 'chauncey'
	| 'lark'
	| 'allight'
	| 'redpoint'
	| 'verve'
	| 'river'
	| 'morris'

export const HousingNameLabels: Record<HousingID, string> = {
	// On-Campus
	'cary-quad': 'Cary Quadrangle',
	'mc-cutcheon': 'McCutcheon Hall',
	tarkington: 'Tarkington Hall',
	wiley: 'Wiley Hall',
	owen: 'Owen Hall',
	shreve: 'Shreve Hall',
	earhart: 'Earhart Hall',
	harrison: 'Harrison Hall',
	hillenbrand: 'Hillenbrand Hall',
	meredith: 'Meredith Hall',
	'meredith-south': 'Meredith South',
	windsor: 'Windsor Halls',
	'first-street': 'First Street Towers',
	hilltop: 'Hilltop Apartments',
	winifred: 'Winifred Parker Hall',
	frieda: 'Frieda Parker Hall',
	hawkins: 'Hawkins Hall',
	fuse: 'Fuse Apartments',
	aspire: 'Aspire at Discovery Park',
	'3rd-and-west': '3rd and West',
	'benchmark-ii': 'Benchmark II',
	'grant-333': 'Grant Street Station 333',
	provenance: 'Provenance Apartments',
	'russell-414': '414 Russell Street',
	'steely-410': '410 Steely Street',
	'waldron-125': '125 Waldron Street',
	'waldron-19': '19 Waldron Street',
	'waldron-square': 'Waldron Square',
	'honors-college-residences': 'Honors College Residences',

	// Off-Campus
	hub: 'Hub on Campus',
	rise: 'Rise on Chauncey',
	chauncey: 'Chauncey Square Apartments',
	lark: 'Lark West Lafayette',
	allight: 'Alight West Lafayette',
	redpoint: 'Redpoint West Lafayette',
	verve: 'Verve West Lafayette',
	river: 'River Market Apartments',
	morris: 'Morris Rentals'
}

export type HousingType = 'Dorm' | 'Apartment'
export type CampusType = 'On-Campus' | 'Off-Campus'

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

export type RatingsBrokenDown = {
	[RatingCategory.LIVING_CONDITIONS]: number // 1-500 or 100, 200, 300, 400, 500 => /100 => 0.00 - 5.00 or 1, 2, 3, 4, 5
	[RatingCategory.LOCATION]: number // 1-500 or 100, 200, 300, 400, 500 => /100 => 0.00 - 5.00 or 1, 2, 3, 4, 5
	[RatingCategory.AMENITIES]: number // 1-500 or 100, 200, 300, 400, 500 => /100 => 0.00 - 5.00 or 1, 2, 3, 4, 5
	[RatingCategory.VALUE]: number // 1-500 or 100, 200, 300, 400, 500 => /100 => 0.00 - 5.00 or 1, 2, 3, 4, 5
	[RatingCategory.COMMUNITY]: number // 1-500 or 100, 200, 300, 400, 500 => /100 => 0.00 - 5.00 or 1, 2, 3, 4, 5
}

export type CostSymbol = '$' | '$$' | '$$$'

export type SemesterSeason = 'Fall' | 'Spring' | 'Summer'

export const ReviewFields = {
	SEMESTER_SEASON: 'semester_season',
	SEMESTER_YEAR: 'semester_year',
	DESCRIPTION: 'description',
	THUMBS_UP: 'thumbs_up',
	THUMBS_DOWN: 'thumbs_down'
} as const

export type Review = {
	[HousingFields.ID]?: string
	[HousingFields.OVERALL_RATING]: number // 100, 200, 300, 400, 500 => /100 => 1, 2, 3, 4, 5
	[HousingFields.RATINGS]: RatingsBrokenDown // 100, 200, 300, 400, 500 => /100 => 1, 2, 3, 4, 5
	[ReviewFields.SEMESTER_SEASON]: SemesterSeason
	[ReviewFields.SEMESTER_YEAR]: number // year <= 255 + 2000
	[ReviewFields.DESCRIPTION]: string
	[ReviewFields.THUMBS_UP]: number
	[ReviewFields.THUMBS_DOWN]: number
}

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

export type Housing = {
	[HousingFields.ID]: string
	[HousingFields.OVERALL_RATING]: number // 1-500 => /100 => 0.00 - 5.00
	[HousingFields.RATINGS]: RatingsBrokenDown
	[HousingFields.REVIEW_COUNT]: number
	[HousingFields.HOUSING_TYPE]: HousingType
	[HousingFields.CAMPUS_TYPE]: CampusType
	[HousingFields.WALK_TIME_MINS]: number
	[HousingFields.COST_MIN]: number // 1-255 => 1k - 255k per year
	[HousingFields.COST_MAX]: number // 1-255 => 1k - 255k per year
	[HousingFields.COST_SYMBOL]: CostSymbol
	[HousingFields.ADDRESS]: string
}
