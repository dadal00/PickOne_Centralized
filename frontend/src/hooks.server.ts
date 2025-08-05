import type { Handle } from '@sveltejs/kit'
import { env } from '$env/dynamic/private'
import {
	PUBLIC_SWAP_BACKEND_PATH,
	PUBLIC_HOME_BACKEND_PATH,
	PUBLIC_HOUSING_BACKEND_PATH,
	PUBLIC_MEILI_PATH,
	PUBLIC_SVELTE_SWAP_ROOT,
	PUBLIC_SVELTE_HOUSING_ROOT
} from '$env/static/public'
import { SignJWT } from 'jose'

export const handle: Handle = async ({ event, resolve }) => {
	let token: string
	let tokenPath: string
	let search: boolean = false

	if (
		(event.request.method === 'HEAD' && event.request.headers.get('x-refresh')) ||
		!event.cookies.get('api_token')
	) {
		switch (true) {
			case event.url.pathname.startsWith(PUBLIC_SVELTE_HOUSING_ROOT):
				token = env.HOUSING_API_TOKEN!
				tokenPath = PUBLIC_HOUSING_BACKEND_PATH
				search = true
				break
			case event.url.pathname.startsWith(PUBLIC_SVELTE_SWAP_ROOT):
				token = env.SWAP_API_TOKEN!
				tokenPath = PUBLIC_SWAP_BACKEND_PATH
				search = true
				break
			case event.url.pathname === '/':
				token = env.HOME_API_TOKEN!
				tokenPath = PUBLIC_HOME_BACKEND_PATH
				search = false
				break
			default:
				token = 'Well'
				tokenPath = 'Oh'
				search = false
		}

		const jwt = await new SignJWT({})
			.setProtectedHeader({ alg: 'HS256' })
			.setIssuedAt()
			.setExpirationTime('5m')
			.sign(new TextEncoder().encode(token))

		event.cookies.set('api_token', jwt, {
			path: tokenPath,
			httpOnly: true,
			sameSite: 'strict',
			secure: true,
			maxAge: 60 * 5
		})
	}

	if (
		((event.request.method === 'HEAD' && event.request.headers.get('x-refresh')) ||
			!event.cookies.get('search_token')) &&
		search
	) {
		const jwt = await new SignJWT({})
			.setProtectedHeader({ alg: 'HS256' })
			.setIssuedAt()
			.setExpirationTime('5m')
			.sign(new TextEncoder().encode(env.SEARCH_TOKEN))

		event.cookies.set('search_token', jwt, {
			path: PUBLIC_MEILI_PATH,
			httpOnly: true,
			sameSite: 'strict',
			secure: true,
			maxAge: 60 * 5
		})
	}

	return resolve(event)
}
