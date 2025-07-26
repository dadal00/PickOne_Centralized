export async function refreshToken() {
	try {
		await fetch('/', {
			method: 'HEAD',
			headers: {
				'x-refresh': 'true'
			}
		})
	} catch (e) {}
}
