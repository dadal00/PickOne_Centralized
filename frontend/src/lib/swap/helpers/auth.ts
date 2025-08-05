import { goto } from '$app/navigation'
import { PUBLIC_SWAP_BACKEND_PATH, PUBLIC_SVELTE_SWAP_ROOT } from '$env/static/public'
import { Status, type Account } from '$lib/swap/models'
import { appState } from '$lib/swap/app-state.svelte'
import {
	fetchBackend,
	isLimited,
	isSignedIn,
	isUpdating,
	isResetting,
	isCodeGood,
	isPasswordGood,
	isVerifying,
	isEmailGood,
	isSignUpGood
} from './utils'

export async function forgot(email: string): Promise<void> {
	if (isLimited() || !isEmailGood(email)) {
		return
	}

	try {
		appState.nowLimited()

		await fetchBackend('/forgot', { token: email })

		appState.setStatus(Status.isVerifyingForgot, true)
		goto(PUBLIC_SVELTE_SWAP_ROOT + '/auth/verify/forget')
	} catch (err) {}
}

export async function login(account: Account): Promise<void> {
	if (isLimited() || !isEmailGood(account.email) || !isPasswordGood(account.password)) {
		return
	}

	try {
		account.action = 'login'

		appState.nowLimited()

		await fetchBackend('/authenticate', account)

		appState.setStatus(Status.isVerifying, true)
		goto(PUBLIC_SVELTE_SWAP_ROOT + '/auth/verify')
	} catch (err) {}
}

export async function signup(account: Account, confirmPassword: string): Promise<void> {
	if (!isSignUpGood(account, confirmPassword)) {
		return
	}

	try {
		account.action = 'signup'

		appState.nowLimited()

		await fetchBackend('/authenticate', account)

		appState.setStatus(Status.isVerifying, true)
		goto(PUBLIC_SVELTE_SWAP_ROOT + '/auth/verify')
	} catch (err) {}
}

export async function verify(authCode: string): Promise<void> {
	if (isLimited() || !isVerifying || !isCodeGood(authCode)) {
		return
	}

	try {
		appState.nowLimited()

		await fetchBackend('/verify', { token: authCode })

		appState.setStatus(Status.isSignedIn, true)
		goto(PUBLIC_SVELTE_SWAP_ROOT + '/browse')
	} catch (err) {}
}

export async function verify_forget(authCode: string) {
	if (isLimited() || !isResetting() || !isCodeGood(authCode)) {
		return
	}

	try {
		appState.nowLimited()

		await fetchBackend('/verify', { token: authCode })

		appState.setStatus(Status.isVerifyingUpdate, true)
		goto(PUBLIC_SVELTE_SWAP_ROOT + '/auth/verify/update')
	} catch (err) {}
}

export async function update(newPassword: string) {
	if (isLimited() || !isUpdating() || !isPasswordGood(newPassword)) {
		return
	}

	try {
		appState.nowLimited()

		await fetchBackend('/verify', { token: newPassword })

		appState.setStatus(Status.isSignedIn, true)
		goto(PUBLIC_SVELTE_SWAP_ROOT + '/browse')
	} catch (err) {}
}

export async function signout() {
	if (isLimited() || !isSignedIn()) {
		return
	}

	appState.nowLimited()

	const response = await fetch(PUBLIC_SWAP_BACKEND_PATH + '/delete', {
		method: 'DELETE',
		credentials: 'include'
	})

	if (!response.ok) {
		throw new Error(`HTTP error! status: ${response.status}`)
	}

	appState.setStatus(Status.isSignedIn, false)
}

export async function resend(resendSeconds: number) {
	if (resendSeconds != 0 || isLimited()) {
		return
	}

	appState.nowLimited()

	const response = await fetch(PUBLIC_SWAP_BACKEND_PATH + '/resend', {
		method: 'POST',
		credentials: 'include'
	})

	if (!response.ok) {
		appState.setAuthError((await response.text()).slice(0, 50))

		throw new Error(`HTTP error! status: ${response.status}`)
	}
}
