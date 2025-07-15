import { PUBLIC_SWAP_BACKEND_PATH } from '$env/static/public'
import { appState } from '$lib/swap/AppState.svelte'
import type { Account, ExpirationColor, TokenPayload } from '../models'
import DOMPurify from 'dompurify'

export async function fetchBackend(path: string, payload: Account | TokenPayload) {
	const response = await fetch(PUBLIC_SWAP_BACKEND_PATH + path, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		credentials: 'include',
		body: JSON.stringify(payload)
	})

	if (!response.ok) {
		appState.setAuthError((await response.text()).slice(0, 50))

		throw new Error(`HTTP error! status: ${response.status}`)
	}
}

export function getDaysUntil(dateString: string): [string, ExpirationColor] {
	const diff = Math.max(
		Math.floor(
			(new Date(dateString).getTime() - appState.getDate().getTime()) / (1000 * 60 * 60 * 24)
		),
		0
	)

	if (diff === 0) {
		return ['Expires today!', 'red']
	}
	if (diff === 1) {
		return ['Expires tommorow!', 'yellow']
	}
	return ['Expires in ' + diff + ' days.', 'green']
}

export function sanitizeHtml(input: string): string {
	return DOMPurify.sanitize(input, {
		ALLOWED_TAGS: ['mark'],
		ALLOWED_ATTR: []
	})
}
