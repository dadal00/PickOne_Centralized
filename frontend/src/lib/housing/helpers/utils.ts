import { toast } from '@zerodevx/svelte-toast'
import { clsx, type ClassValue } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(...inputs))
}

export function bindNumber(payload: number, min: number, max: number): number {
	return Math.min(max, Math.max(min, payload))
}

export function copy(text: string) {
	navigator.clipboard
		.writeText(text)
		.then(() => toast.push('Copied!'))
		.catch(() => toast.push('Failed to copy'))
}

export function toSelectOptions<T extends Record<string, string>>(labels: T) {
	return Object.entries(labels).map(([value, label]) => ({
		value,
		label
	})) as { value: keyof T & string; label: string }[]
}

export function toFilterOptions<T extends readonly string[]>(iterable: T) {
	return Object.fromEntries([['', 'All'], ...iterable.map((v) => [v, v])]) as Record<
		'' | T[number],
		string
	>
}
