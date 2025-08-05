import {
	ItemFields,
	Status,
	type Condition,
	type FullQuery,
	type Item,
	type ItemType,
	type Location
} from './models'
import { page } from '$app/state'
import { PUBLIC_SVELTE_SWAP_ROOT } from '$env/static/public'
import { PaginatingClass } from '$lib/classes.svelte'

class AppState extends PaginatingClass {
	private signedIn: boolean = $state(false)
	private toVerify: boolean = $state(false)
	private toVerifyForgot: boolean = $state(false)
	private toVerifyUpdate: boolean = $state(false)

	private authError: string = $state('')
	private postError: string = $state('')

	private limited: boolean = $state(false)
	private productLimited: boolean = $state(false)

	// Query params for search
	private query: string = $state('')
	// Search filters
	private itemTypeFilter: ItemType | '' = $state('')
	private locationFilter: Location | '' = $state('')
	private conditionFilter: Condition | '' = $state('')

	// Search resuts
	private hits: Item[] = $state([])
	// Date used for marking expiration
	private todaysDate: Date = new Date()

	getPostError(): string {
		return this.postError
	}

	setPostError(err: string): void {
		this.postError = err
	}

	getAuthError(): string {
		return this.authError
	}

	setAuthError(err: string): void {
		this.authError = err
	}

	getDate(): Date {
		return this.todaysDate
	}

	getLimited(): boolean {
		return this.limited
	}

	getProductLimited(): boolean {
		return this.productLimited
	}

	nowLimited(): void {
		this.limited = true
		setTimeout(() => {
			this.limited = false
		}, 500)
	}

	nowProductLimited(): void {
		this.productLimited = true
		setTimeout(() => {
			this.productLimited = false
		}, 1000)
	}

	setQuery(query: string): void {
		this.offset = 0
		this.query = query
	}

	setItemTypeFilter(filter: ItemType | ''): void {
		this.offset = 0
		this.itemTypeFilter = filter
	}

	setLocationFilter(filter: Location | ''): void {
		this.offset = 0
		this.locationFilter = filter
	}

	setConditionFilter(filter: Condition | ''): void {
		this.offset = 0
		this.conditionFilter = filter
	}

	getFullQuery(): FullQuery {
		return {
			query: this.query,
			[ItemFields.ITEM_TYPE]: this.itemTypeFilter,
			[ItemFields.LOCATION]: this.locationFilter,
			[ItemFields.CONDITION]: this.conditionFilter,
			offset: this.offset
		}
	}

	setQueryResults(items: Item[], totalItems: number): void {
		this.hits = items
		this.totalHits = totalItems
	}

	getTotalHits(): number {
		return this.totalHits
	}

	getHits(): Item[] {
		return page.url.pathname.includes(PUBLIC_SVELTE_SWAP_ROOT + '/browse')
			? this.hits
			: this.hits.slice(0, 3)
	}

	getStatus(status: Status): boolean {
		switch (status) {
			case Status.isSignedIn:
				return this.signedIn
			case Status.isVerifying:
				return this.toVerify
			case Status.isVerifyingForgot:
				return this.toVerifyForgot
			case Status.isVerifyingUpdate:
				return this.toVerifyUpdate
			default:
				throw new Error('Invalid flag')
		}
	}

	setStatus(status: Status, value: boolean): void {
		switch (status) {
			case Status.isSignedIn:
				this.signedIn = value
				break
			case Status.isVerifying:
				this.toVerify = value
				break
			case Status.isVerifyingForgot:
				this.toVerifyForgot = value
				break
			case Status.isVerifyingUpdate:
				this.toVerifyUpdate = value
				break
			default:
				throw new Error('Invalid flag')
		}
	}
}

export const appState = new AppState()
