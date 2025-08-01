import { PaginatingClass } from '$lib/classes.svelte'
import {
	defaultHousingSortBy,
	HousingFields,
	type CampusType,
	type CostSymbol,
	type Housing,
	type HousingQuery,
	type HousingSortBy,
	type HousingType
} from './models/housing'
import {
	defaultReviewSortBy,
	type Review,
	type ReviewQuery,
	type ReviewRating,
	type ReviewSortBy
} from './models/reviews'

class AppState extends PaginatingClass {
	// Housing search results
	private housingHits: Housing[] = $state([])
	// Housing search params
	private housingQuery: string = $state('')
	// Filters, default is no filters
	private housingTypeFilter: HousingType | '' = $state('')
	private campusTypeFilter: CampusType | '' = $state('')
	private costSymbolFilter: CostSymbol | '' = $state('')
	// Sorting, default is most number of reviews first
	private housingSortBy: HousingSortBy = $state(defaultHousingSortBy)

	// Review search results
	private reviewHits: Review[] = $state([])
	// Review search params
	private reviewQuery: string = $state('')
	// Filters, default is no filter
	private reviewRatingFilter: ReviewRating | 0 = $state(0)
	// Sorting, default is most recent
	private reviewSortBy: ReviewSortBy = $state(defaultReviewSortBy)

	// Samples for home page
	sampleHousing(count: number): Housing[] {
		return this.housingHits.slice(0, count)
	}

	// Helpers to check if housing is loaded
	fetchHousing(id: string): Housing | undefined {
		return this.housingHits.find((housing) => housing.id === id)
	}

	// Query the housing options
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

	// Modify housing sorting
	setHousingSortBy(housingSortBy: HousingSortBy): void {
		this.housingSortBy = housingSortBy
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
}

export const appState = new AppState()
