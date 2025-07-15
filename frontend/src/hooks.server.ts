import type { Handle } from '@sveltejs/kit'
import { env } from '$env/dynamic/private'
import { PUBLIC_SWAP_BACKEND_PATH } from '$env/static/public'
import { SignJWT } from 'jose'

export const handle: Handle = async ({ event, resolve }) => {
	if (
		(event.request.method === 'HEAD' && event.request.headers.get('x-refresh')) ||
		!event.cookies.get('api_token')
	) {
		const jwt = await new SignJWT({})
			.setProtectedHeader({ alg: 'HS256' })
			.setIssuedAt()
			.setExpirationTime('5m')
			.sign(new TextEncoder().encode(env.SWAP_API_TOKEN))

		event.cookies.set('api_token', jwt, {
			path: PUBLIC_SWAP_BACKEND_PATH,
			httpOnly: true,
			sameSite: 'strict',
			secure: true,
			maxAge: 60 * 5
		})
	}

	if (
		(event.request.method === 'HEAD' && event.request.headers.get('x-refresh')) ||
		!event.cookies.get('search_token')
	) {
		const jwt = await new SignJWT({})
			.setProtectedHeader({ alg: 'HS256' })
			.setIssuedAt()
			.setExpirationTime('5m')
			.sign(new TextEncoder().encode(env.SEARCH_TOKEN))

		event.cookies.set('search_token', jwt, {
			path: '/search',
			httpOnly: true,
			sameSite: 'strict',
			secure: true,
			maxAge: 60 * 5
		})
	}

	return resolve(event)
}
