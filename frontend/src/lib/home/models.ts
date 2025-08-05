import { PUBLIC_SVELTE_HOUSING_ROOT, PUBLIC_SVELTE_SWAP_ROOT } from '$env/static/public'

export type ChartData = {
	website: Website
	visitors: number
}

type WebsiteMeta = {
	label: string
	color?: string
	gradient?: {
		gradientStart: string
		gradientEnd: string
	}
	link: string
}

export const websiteMeta: Record<Website, WebsiteMeta> = {
	home: {
		label: 'Home',
		color: '#2C67F2',
		link: '/'
	},
	photos: {
		label: 'Cut',
		color: '#5b98d9',
		link: 'https://t.me/boilercuts_bot'
	},
	swap: {
		label: 'Swap',
		gradient: {
			gradientStart: '#facc15',
			gradientEnd: '#f59e0b'
		},
		link: PUBLIC_SVELTE_SWAP_ROOT
	},
	housing: {
		label: 'Rate',
		color: '#5b98d9',
		link: PUBLIC_SVELTE_HOUSING_ROOT
	}
}

type Website = 'swap' | 'photos' | 'home' | 'housing'

export type chartConfig = {
	width: number
	height: number
	mobile: boolean
	mobileChanged: boolean
	outerPadding: number
	minBarWidth: number
}

export const baseChartConfig = {
	// Config that changes
	baseWidth: 1000,
	baseHeight: 625,
	baseOuterPadding: 0.01,
	baseMinBarWidth: 200,

	// Config that does not change
	delay: 200,
	xScale: 1.1,
	innerPadding: 0.35,
	heightScale: 0.95,

	// Text config
	mobileFontSize: 'max(2.6vh, 0.7vw, 0.5rem)',
	notMobileFontSize: 'max(3.4vh, 1.4vw, 1rem)',
	fontFamily: 'Verdana, Geneva, sans-serif',
	textColor: 'white',
	xDistance: 20,
	yChange: '0.35em',

	// Border config
	borderColor: '#5e5757',
	borderWidth: '2',
	borderRadius: '11'
}
