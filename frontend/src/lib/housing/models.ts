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

	// Off Campus
	| 'hub'
	| 'rise'
	| 'chauncey'
	| 'lark'
	| 'allight'
	| 'redpoint'
	| 'quarters'
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
	winifred: 'Winifred Parker Hall',
	frieda: 'Frieda Parker Hall',
	hawkins: 'Hawkins Hall',
	hilltop: 'Hilltop Apartments',

	// Off-Campus
	fuse: 'Fuse Apartments',
	aspire: 'Aspire at Discovery Park',
	hub: 'Hub on Campus',
	rise: 'Rise on Chauncey',
	chauncey: 'Chauncey Square Apartments',
	lark: 'Lark West Lafayette',
	allight: 'Alight West Lafayette',
	redpoint: 'Redpoint West Lafayette',
	quarters: 'The Quarters',
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
	[RatingCategory.LIVING_CONDITIONS]: number
	[RatingCategory.LOCATION]: number
	[RatingCategory.AMENITIES]: number
	[RatingCategory.VALUE]: number
	[RatingCategory.COMMUNITY]: number
}

export type Amenities = [
	'Study Lounges',
	'24/7 Front Desk',
	'Community Kitchen',
	'Lounge Areas',
	'In-Hall Dining Court',
	'Fitness Room',
	'Study Rooms',
	'Vending Areas',
	'Private Bathrooms'
]

export type SemesterSeason = 'Fall' | 'Spring' | 'Summer'

export const ReviewFields = {
	SEMESTER_SEASON: 'semester_season',
	SEMESTER_YEAR: 'semester_year',
	DESCRIPTION: 'description',
	THUMBS_UP: 'thumbs_up',
	THUMBS_DOWN: 'thumbs_down'
} as const

export type Review = {
	[HousingFields.OVERALL_RATING]: number
	[HousingFields.RATINGS]: RatingsBrokenDown
	[ReviewFields.SEMESTER_SEASON]: SemesterSeason
	[ReviewFields.SEMESTER_YEAR]: number
	[ReviewFields.DESCRIPTION]: string
	[ReviewFields.THUMBS_UP]: number
	[ReviewFields.THUMBS_DOWN]: number
}

export const HousingFields = {
	ID: 'housing_id',
	OVERALL_RATING: 'overall_rating',
	RATINGS: 'ratings',
	REVIEW_COUNT: 'review_count',
	HOUSING_TYPE: 'housing_type',
	CAMPUS_TYPE: 'campus_type',
	WALK_TIME_MINS: 'walk_time',
	COST_MIN: 'cost_min',
	COST_MAX: 'cost_max',
	ADDRESS: 'address',
	AMENITIES: 'amenities'
} as const

export type Housing = {
	[HousingFields.ID]: string
	[HousingFields.OVERALL_RATING]: number
	[HousingFields.RATINGS]: RatingsBrokenDown
	[HousingFields.REVIEW_COUNT]: number
	[HousingFields.HOUSING_TYPE]: HousingType
	[HousingFields.CAMPUS_TYPE]: CampusType
	[HousingFields.WALK_TIME_MINS]: number
	[HousingFields.COST_MIN]: number
	[HousingFields.COST_MAX]: number
	[HousingFields.ADDRESS]: string
	[HousingFields.AMENITIES]: Amenities[]
}
