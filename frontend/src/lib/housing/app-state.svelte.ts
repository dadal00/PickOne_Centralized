import { PaginatingClass } from '$lib/classes.svelte'
import type { SortDirection } from './models/general'
import {
	defaultHousingSortBy,
	type CostSymbol,
	type Housing,
	type HousingQuery,
	type HousingSortBy,
	type HousingType
} from './models/housing'
import { HousingFields } from './constants/housing'
import type { HousingSortable, CampusType, HousingID } from './constants/housing'
import {
	defaultReviewSortBy,
	type Review,
	type ReviewQuery,
	type ReviewSortBy,
	type ThumbsDelta,
	type ThumbsDeltaMap
} from './models/reviews'
import { type ReviewRating } from './constants/reviews'
import { PUBLIC_HOUSING_MAX_THUMBS } from '$env/static/public'
import { flushThumbs } from './helpers/housing'

class AppState extends PaginatingClass {
	/*
		Unified query
		- no need to separate as housing is cleared on destruction of both searches
	*/
	private query: string = $state('')

	// Housing search results
	private housingHits: Housing[] = $state([
		{
			id: 'cary-quad',
			overall_rating: 420,
			ratings: {
				living_conditions: 410,
				location: 480,
				amenities: 390,
				value: 400,
				community: 450
			},
			review_count: 156,
			housing_type: 'Dorm',
			campus_type: 'On-Campus',
			walk_time_mins: 5,
			cost_min: 8,
			cost_max: 10,
			cost_symbol: '$$$',
			address: '1131 3rd Street, West Lafayette, IN 47907'
		}
	])
	// Filters, default is no filters
	private housingTypeFilter: HousingType | '' = $state('')
	private campusTypeFilter: CampusType | '' = $state('')
	private costSymbolFilter: CostSymbol | '' = $state('')
	// Sorting, default is most number of reviews first
	private housingSortBy: HousingSortBy = $state(defaultHousingSortBy)

	// Review search results
	private reviewHits: Review[] = $state([
		{
			id: '5',
			housing_id: 'cary-quad',
			overall_rating: 200,
			ratings: {
				living_conditions: 200,
				location: 300,
				amenities: 200,
				value: 200,
				community: 300
			},
			date: '2024-09-18',
			description:
				'Heating barely worked. Maintenance slow to respond. It was quiet, which I appreciated, but overall it felt like a downgrade.',
			thumbs_up: 5,
			thumbs_down: 3
		}
	])
	// Filters, default is no filter
	private reviewRatingFilter: ReviewRating | 0 = $state(0)
	// Sorting, default is most recent
	private reviewSortBy: ReviewSortBy = $state(defaultReviewSortBy)

	// Keeping state to prefill housing for writing new reviews
	private writeReviewHousing: HousingID | '' = $state('')
	// Display error when posting reviews
	private postError: string = $state('')
	// Debouncing posting
	private postLimited: boolean = $state(false)

	// Array of thumb actions
	private thumbActions: ThumbsDeltaMap = $state({})

	getThumbActions(): ThumbsDeltaMap {
		return this.thumbActions
	}

	updateThumbAction(reviewId: string, action: ThumbsDelta): void {
		if (!(reviewId in this.thumbActions)) {
			this.thumbActions[reviewId] = action

			if (Object.keys(this.thumbActions).length >= Number(PUBLIC_HOUSING_MAX_THUMBS)) {
				flushThumbs()
			}
		} else if (this.thumbActions[reviewId] === action) {
			delete this.thumbActions[reviewId]
		} else {
			this.thumbActions[reviewId] = action
		}
	}

	resetThumbs(): void {
		this.thumbActions = {}
	}

	getPostError(): string {
		return this.postError
	}

	setPostError(err: string): void {
		this.postError = err
	}

	getLimited(): boolean {
		return this.postLimited
	}

	nowLimited(): void {
		this.postLimited = true

		setTimeout(() => {
			this.postLimited = false
		}, 1000)
	}

	getWriteReviewHousing(): HousingID | '' {
		return this.writeReviewHousing
	}

	setWriteReviewHousing(writeReviewHousing: HousingID): void {
		this.writeReviewHousing = writeReviewHousing
	}

	// Samples for home page
	sampleHousing(count: number): Housing[] {
		return this.housingHits.slice(0, count)
	}

	// Helpers to check if housing is loaded
	fetchHousing(id: string): Housing | undefined {
		return this.housingHits.find((housing) => housing.id === id)
	}

	// The entire query including options
	getFullHousingQuery(): HousingQuery {
		return {
			query: this.query,
			[HousingFields.HOUSING_TYPE]: this.housingTypeFilter,
			[HousingFields.CAMPUS_TYPE]: this.campusTypeFilter,
			[HousingFields.COST_SYMBOL]: this.costSymbolFilter,
			sortBy: this.housingSortBy,
			offset: this.offset
		}
	}

	// Only the string query
	getQuery(): string {
		return this.query
	}

	setQuery(query: string): void {
		this.query = query
	}

	// Get housing filters
	getHousingTypeFilter(): HousingType | '' {
		return this.housingTypeFilter
	}

	getCampusTypeFilter(): CampusType | '' {
		return this.campusTypeFilter
	}

	getCostSymbolFilter(): CostSymbol | '' {
		return this.costSymbolFilter
	}

	// Modify housing filters
	setHousingTypeFilter(housingType: HousingType | ''): void {
		this.housingTypeFilter = housingType
	}

	setCampusTypeFilter(campusType: CampusType | ''): void {
		this.campusTypeFilter = campusType
	}

	setCostSymbolFilter(costSymbol: CostSymbol | ''): void {
		this.costSymbolFilter = costSymbol
	}

	// Get housing sort by
	getHousingSortCategory(): HousingSortable {
		return this.housingSortBy[0]
	}

	getHousingSortDirection(): SortDirection {
		return this.housingSortBy[1]
	}

	// Modify housing sorting
	setHousingSortCategory(housingSortCategory: HousingSortable): void {
		this.housingSortBy[0] = housingSortCategory
	}

	setHousingSortDirection(housingSortDirection: SortDirection): void {
		this.housingSortBy[1] = housingSortDirection
	}

	// Query the reviews
	getFullReviewQuery(): ReviewQuery {
		return {
			query: this.query,
			[HousingFields.OVERALL_RATING]: this.reviewRatingFilter,
			sortBy: this.reviewSortBy,
			offset: this.offset
		}
	}

	// Modify rating filters
	getRatingFilter(): number {
		return this.reviewRatingFilter
	}

	setRatingFilter(reviewRating: ReviewRating | 0): void {
		this.reviewRatingFilter = reviewRating
	}

	// Modify review sorting
	getReviewSortDirection(): SortDirection {
		return this.reviewSortBy[1]
	}

	setReviewSortDirection(reviewSortDirection: SortDirection): void {
		this.reviewSortBy[1] = reviewSortDirection
	}

	// Search results
	getReviews(): Review[] {
		return this.reviewHits
	}

	getHousingHits(): Housing[] {
		return this.housingHits
	}

	setReviewHits(reviewHits: Review[]): void {
		this.reviewHits = reviewHits
	}

	setHousingHits(housingHits: Housing[]): void {
		this.housingHits = housingHits
	}

	setTotalHits(totalHits: number): void {
		this.totalHits = totalHits
	}

	// Clear housing search
	clearFullHousingQuery(): void {
		this.query = ''
		this.housingTypeFilter = ''
		this.campusTypeFilter = ''
		this.costSymbolFilter = ''
		this.housingSortBy = defaultHousingSortBy
		this.offset = 0
	}

	// Clear reviews search
	clearFullReviewQuery(): void {
		this.query = ''
		this.reviewRatingFilter = 0
		this.reviewSortBy = defaultReviewSortBy
		this.offset = 0
	}
}

export const appState = new AppState()
