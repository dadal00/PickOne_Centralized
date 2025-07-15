export type ChartData = {
	website: Website
	visitors: number
	color: string
}

type Website = 'BoilerSwap' | 'BoilerCuts' | 'Voting' | 'Home'

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
