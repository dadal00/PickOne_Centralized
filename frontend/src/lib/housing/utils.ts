import { toast } from '@zerodevx/svelte-toast'
import { clsx, type ClassValue } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(...inputs))
}

export function bindNumber(payload: number, min: number, max: number): number {
	return Math.min(max, Math.max(min, payload))
}

export function convertRatingToBase5(rating: number): number {
	return bindNumber(rating, 0, 500) / 100.0
}

export function convertRatingToHousingLabel(rating: number): string {
	return convertRatingToBase5(rating).toFixed(2)
}

export function convertRatingToReviewLabel(rating: number): string {
	return convertRatingToBase5(rating).toFixed(0)
}

export function convertCost(costMin: number, costMax: number): string {
	const safeMin = Math.min(costMin, costMax)
	const safeMax = Math.max(costMin, costMax)

	if (costMax == costMin) {
		return 'Around $' + bindNumber(safeMin, 1, 255) + ',000 per year'
	}

	return (
		'Around $' +
		bindNumber(safeMin, 1, 255) +
		',000 - $' +
		bindNumber(safeMax, 1, 255) +
		',000 per year'
	)
}

export function walkToWALC(address: string): string {
	return `https://www.google.com/maps/dir/?api=1&origin=${encodeURIComponent(address)}&destination=WALC,+West+Lafayette,+IN&travelmode=walking`
}

export function copy(text: string) {
	navigator.clipboard
		.writeText(text)
		.then(() => toast.push('Copied!'))
		.catch(() => toast.push('Failed to copy'))
}

export function convertDate(datePreFormat: string): string {
	const date = new Date(datePreFormat)

	return date.toLocaleDateString('en-US', { month: 'long', year: 'numeric' })
}
