import type { Housing, Review } from './models'

class AppState {
	private housingHits: Housing[] = []

	private reviewHits: Review[] = []

	getHousing(id: string) {
		return this.housingHits.find((housing) => housing.id === id)
	}

	getReviews() {
		return this.reviewHits
	}

	getHousingHits() {
		return this.housingHits
	}

	sampleHousing(count: number) {
		return this.housingHits.slice(0, count)
	}
}

export const appState = new AppState()
