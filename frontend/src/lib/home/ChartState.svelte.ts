import { baseChartConfig, type chartConfig, type ChartData } from './models'

class ChartState {
	private chartConfig: chartConfig = $state({
		width: baseChartConfig.baseWidth,
		height: baseChartConfig.baseHeight,
		mobile: false,
		mobileChanged: false,
		outerPadding: baseChartConfig.baseOuterPadding,
		minBarWidth: baseChartConfig.baseMinBarWidth
	})

	private data: ChartData[] = [
		{ website: 'BoilerSwap', visitors: 1250, color: 'red' },
		{ website: 'BoilerCuts', visitors: 980, color: 'blue' },
		{ website: 'Voting', visitors: 700, color: 'green' },
		{ website: 'Home', visitors: 600, color: 'purple' }
	]

	getChartConfig(): chartConfig {
		return this.chartConfig
	}

	setMinBarWidth(minBarWidth: number): void {
		this.chartConfig.minBarWidth = minBarWidth
	}

	setOuterPadding(outerPadding: number): void {
		this.chartConfig.outerPadding = outerPadding
	}

	setWidth(width: number): void {
		this.chartConfig.width = width
	}

	setHeight(height: number): void {
		this.chartConfig.height = height
	}

	setMobileChanged(mobileChanged: boolean): void {
		this.chartConfig.mobileChanged = mobileChanged
	}

	setMobile(mobile: boolean): void {
		this.chartConfig.mobile = mobile
	}

	getData(): ChartData[] {
		return this.data
	}

	setData(data: ChartData[]): void {
		this.data = data
	}
}

export const chartState = new ChartState()
