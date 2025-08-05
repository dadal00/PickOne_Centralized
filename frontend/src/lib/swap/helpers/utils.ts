import {
	PUBLIC_CODE_LENGTH,
	PUBLIC_MAX_CHARS,
	PUBLIC_MIN_PASSWORD_LENGTH,
	PUBLIC_SWAP_BACKEND_PATH
} from '$env/static/public'
import { appState } from '$lib/swap/app-state.svelte'
import { Status, type Account, type ExpirationColor, type TokenPayload } from '../models'
import DOMPurify from 'dompurify'

export function isLimited(): boolean {
	return appState.getLimited()
}

export function isSignedIn(): boolean {
	return appState.getStatus(Status.isSignedIn)
}

export function isUpdating(): boolean {
	return appState.getStatus(Status.isVerifyingUpdate)
}

export function isResetting(): boolean {
	return appState.getStatus(Status.isVerifyingForgot)
}

export function isVerifying(): boolean {
	return appState.getStatus(Status.isVerifying)
}

export function isSignUpGood(account: Account, confirmPassword: string): boolean {
	if (isLimited() || !isEmailGood(account.email) || !isPasswordGood(account.password)) {
		return false
	}

	if (account.password !== confirmPassword) {
		appState.setAuthError('Passwords do not match')

		return false
	}

	return true
}

export function isEmailGood(email: string): boolean {
	if (/.+@purdue\.edu$/.test(email) && email.length <= Number(PUBLIC_MAX_CHARS)) {
		return true
	}

	appState.setAuthError('Email must be a Purdue address')

	return false
}

export function isPasswordGood(password: string): boolean {
	if (
		password !== '' &&
		password.length <= Number(PUBLIC_MAX_CHARS) &&
		password.length >= Number(PUBLIC_MIN_PASSWORD_LENGTH)
	) {
		return true
	}

	appState.setAuthError('Password must be 10+ characters')

	return false
}

export function isCodeGood(code: string): boolean {
	if (/^\d+$/.test(code) && code.length === 6) {
		return true
	}

	appState.setAuthError('Only ' + PUBLIC_CODE_LENGTH + ' numbers')

	return false
}

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
