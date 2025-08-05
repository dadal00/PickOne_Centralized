import { PUBLIC_PAGE_SIZE } from '$env/static/public'

// Centralized Class for Paginating
export abstract class PaginatingClass {
	// Total search results found
	protected totalHits: number = $state(0)

	// How many results to skip for pages
	protected offset: number = $state(0)

	getOffset(): number {
		return this.offset
	}

	setOffset(offset: number): void {
		this.offset = Math.max(
			Math.min(
				offset,
				// Calculating page number
				Math.floor(this.totalHits / Number(PUBLIC_PAGE_SIZE)) * Number(PUBLIC_PAGE_SIZE)
			),
			0
		)
	}

	incrementOffset(): void {
		this.offset = Math.min(
			this.offset + Number(PUBLIC_PAGE_SIZE),
			// Using page number to move forward 1 page
			Math.floor(this.totalHits / Number(PUBLIC_PAGE_SIZE)) * Number(PUBLIC_PAGE_SIZE)
		)
	}

	decrementOffset(): void {
		// Using page number to move backward 1 page
		this.offset = Math.max(this.offset - Number(PUBLIC_PAGE_SIZE), 0)
	}
}
