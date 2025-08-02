import { PaginatingClass } from '$lib/classes.svelte'
import type { SortDirection } from './models/general'
import {
	defaultHousingSortBy,
	type CampusType,
	type CostSymbol,
	type Housing,
	type HousingID,
	type HousingQuery,
	type HousingSortBy,
	type HousingType
} from './models/housing'
import { HousingFields } from './constants/housing'
import { type HousingSortable } from './constants/housing'
import {
	defaultReviewSortBy,
	type Review,
	type ReviewQuery,
	type ReviewRating,
	type ReviewSortBy
} from './models/reviews'

class AppState extends PaginatingClass {
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
	// Housing search params
	private housingQuery: string = $state('')
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
	// Review search params
	private reviewQuery: string = $state('')
	// Filters, default is no filter
	private reviewRatingFilter: ReviewRating | 0 = $state(0)
	// Sorting, default is most recent
	private reviewSortBy: ReviewSortBy = $state(defaultReviewSortBy)

	// Keeping state to prefill housing for writing new reviews
	private writeReviewHousing: HousingID | '' = $state('')
	// Display error when posting reviews
	private postError: string = $state('')

	private postLimited: boolean = $state(false)

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
			query: this.housingQuery,
			[HousingFields.HOUSING_TYPE]: this.housingTypeFilter,
			[HousingFields.CAMPUS_TYPE]: this.campusTypeFilter,
			[HousingFields.COST_SYMBOL]: this.costSymbolFilter,
			sortBy: this.housingSortBy,
			offset: this.offset
		}
	}

	// Only the string query
	getHousingQuery(): string {
		return this.housingQuery
	}

	setHousingQuery(query: string): void {
		this.housingQuery = query
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
			query: this.reviewQuery,
			[HousingFields.OVERALL_RATING]: this.reviewRatingFilter,
			sortBy: this.reviewSortBy,
			offset: this.offset
		}
	}

	setReviewQuery(query: string): void {
		this.reviewQuery = query
	}

	// Modify rating filters
	setRatingFilter(reviewRating: ReviewRating | 0): void {
		this.reviewRatingFilter = reviewRating
	}

	// Modify rousing sorting
	setReviewSortBy(reviewSortBy: ReviewSortBy): void {
		this.reviewSortBy = reviewSortBy
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
		this.housingQuery = ''
		this.housingTypeFilter = ''
		this.campusTypeFilter = ''
		this.costSymbolFilter = ''
		this.housingSortBy = defaultHousingSortBy
		this.offset = 0
	}
}

export const appState = new AppState()
