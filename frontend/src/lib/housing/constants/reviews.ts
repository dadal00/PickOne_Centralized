import type { SortDirection } from '../models/general'

export const ReviewFields = {
	HOUSING_ID: 'housing_id',
	DATE: 'date',
	DESCRIPTION: 'description',
	THUMBS_UP: 'thumbs_up',
	THUMBS_DOWN: 'thumbs_down'
} as const

export const reviewSortLabels: Record<SortDirection, string> = {
	asc: 'Oldest First',
	desc: 'Most Recent'
}

export const ReviewRatingIterable = [100, 200, 300, 400, 500] as const
