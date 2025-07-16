import { PUBLIC_HOME_BACKEND_PATH } from '$env/static/public'
import { chartState } from '../ChartState.svelte'

export function format_number(num: number): string {
	if (num >= 1_000_000_000) {
		return (num / 1_000_000_000).toFixed(1) + 'B'
	} else if (num >= 1_000_000) {
		return (num / 1_000_000).toFixed(1) + 'M'
	} else if (num >= 1_000) {
		return (num / 1_000).toFixed(1) + 'K'
	}
	return num.toString()
}

export async function fetch_visitors(): Promise<void> {
	const response = await fetch(PUBLIC_HOME_BACKEND_PATH + '/visitors', {
		method: 'POST',
		credentials: 'include'
	})

	if (!response.ok) {
		setTimeout(fetch_visitors, 5000)
		throw new Error(`HTTP error! status: ${response.status}`)
	}

	chartState.setData(await response.json())
}
