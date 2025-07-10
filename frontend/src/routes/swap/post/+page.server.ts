import { redirect, type Cookies } from '@sveltejs/kit'
import { PUBLIC_SVELTE_SWAP_ROOT } from '$env/static/public'

export const load = async ({ cookies }: { cookies: Cookies }) => {
	if (!cookies.get('session_id')) {
		redirect(302, PUBLIC_SVELTE_SWAP_ROOT + '/auth')
	}
}
