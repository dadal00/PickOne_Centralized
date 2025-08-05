import * as d3 from 'd3'
import { baseChartConfig, websiteMeta, type chartConfig, type ChartData } from '../models'
import { chartState } from '../chart-state.svelte'
import { format_number } from './utils'

export function chart_init(
	width: number,
	height: number
): d3.Selection<SVGSVGElement, unknown, HTMLElement, any> {
	const svg = d3
		.select('#chart')
		.append('svg')
		.attr('viewBox', [0, 0, width, height])
		.attr('width', '100%')
		.attr('height', '100%')

	svg
		.append('defs')
		.append('linearGradient')
		.attr('id', 'swap-gradient')
		.attr('x1', '0%')
		.attr('y1', '0%')
		.attr('x2', '100%')
		.attr('y2', '100%')
		.selectAll('stop')
		.data([
			{ offset: '0%', color: websiteMeta.swap.gradient!.gradientStart },
			{ offset: '100%', color: websiteMeta.swap.gradient!.gradientEnd }
		])
		.enter()
		.append('stop')
		.attr('offset', (d) => d.offset)
		.attr('stop-color', (d) => d.color)

	return svg
}

export function update_chart(
	dynamicChartConfig: chartConfig,
	svg: d3.Selection<SVGSVGElement, unknown, HTMLElement, any> | null,
	data: ChartData[]
): d3.Selection<SVGSVGElement, unknown, HTMLElement, any> | null {
	if (!svg) return svg

	const xMax = d3.max(data, (dataPoint) => dataPoint.visitors) ?? 0

	const xScale = d3
		.scaleLinear()
		.domain([0, xMax * baseChartConfig.xScale])
		.range([0, dynamicChartConfig.width])

	const yScale = d3
		.scaleBand()
		.domain(data.map((dataPoint) => dataPoint.website))
		.range([0, dynamicChartConfig.height])
		.paddingInner(baseChartConfig.innerPadding)
		.paddingOuter(dynamicChartConfig.outerPadding)

	const bars = svg
		.selectAll<SVGGElement, ChartData>('.bar')
		.data(data, (dataPoint) => dataPoint.website)

	bars.exit().transition().duration(baseChartConfig.delay).attr('width', 0).remove()

	const newBars = bars
		.enter()
		.append('a')
		.attr('xlink:href', (dataPoint: ChartData) => websiteMeta[dataPoint.website].link)
		.attr('target', '_blank')
		.append('g')
		.attr('class', 'bar')
		.attr('transform', (dataPoint: ChartData) => `translate(0, ${yScale(dataPoint.website)})`)
		.attr('opacity', 0)

	newBars
		.append('rect')
		.attr('height', yScale.bandwidth())
		.attr('fill', (dataPoint: ChartData) => {
			return dataPoint.website === 'swap'
				? 'url(#swap-gradient)'
				: websiteMeta[dataPoint.website].color!
		})
		.attr('stroke', baseChartConfig.borderColor)
		.attr('stroke-width', baseChartConfig.borderWidth)
		.attr('rx', baseChartConfig.borderRadius)
		.attr('ry', baseChartConfig.borderRadius)

	newBars
		.append('text')
		.attr('class', 'value-label')
		.style(
			'font-size',
			dynamicChartConfig.mobile ? baseChartConfig.mobileFontSize : baseChartConfig.notMobileFontSize
		)
		.style('font-family', baseChartConfig.fontFamily)
		.style('fill', 'white')
		.style('text-anchor', dynamicChartConfig.mobile ? 'beginning' : 'end')

	newBars
		.append('text')
		.attr('class', 'name-label')
		.style(
			'font-size',
			dynamicChartConfig.mobile ? baseChartConfig.mobileFontSize : baseChartConfig.notMobileFontSize
		)
		.style('font-family', baseChartConfig.fontFamily)
		.style('fill', baseChartConfig.textColor)
		.style('text-anchor', 'beginning')

	const merged = newBars.merge(bars)

	merged
		.transition()
		.duration(baseChartConfig.delay)
		.attr('transform', (dataPoint: ChartData) => `translate(1, ${yScale(dataPoint.website)})`)
		.attr('opacity', 1)

	merged
		.select('rect')
		.transition()
		.duration(baseChartConfig.delay)
		.attr('width', (dataPoint: ChartData) =>
			Math.max(dynamicChartConfig.minBarWidth, xScale(dataPoint.visitors))
		)
		.attr('height', yScale.bandwidth())

	merged
		.select('.value-label')
		.transition()
		.duration(baseChartConfig.delay)
		.attr('x', (dataPoint: ChartData) =>
			dynamicChartConfig.mobile
				? baseChartConfig.xDistance
				: Math.max(dynamicChartConfig.minBarWidth, xScale(dataPoint.visitors)) - 20
		)
		.attr('y', dynamicChartConfig.mobile ? yScale.bandwidth() / 2 + 20 : yScale.bandwidth() / 2)
		.attr('dy', baseChartConfig.yChange)
		.text((dataPoint: ChartData) => format_number(dataPoint.visitors))

	merged
		.select('.name-label')
		.transition()
		.duration(baseChartConfig.delay)
		.attr('x', baseChartConfig.xDistance)
		.attr('y', dynamicChartConfig.mobile ? yScale.bandwidth() / 2 - 20 : yScale.bandwidth() / 2)
		.attr('dy', baseChartConfig.yChange)
		.text((dataPoint: ChartData) => websiteMeta[dataPoint.website].label)

	return svg
}

function getMinBarWidth(width: number, height: number): number {
	if (width < 445) {
		chartState.setMobileChanged(chartState.getChartConfig().mobile ? false : true)

		chartState.setMobile(true)

		return height < 620 ? 125 : 150
	}

	if (height < 450) {
		chartState.setMobileChanged(chartState.getChartConfig().mobile ? true : false)

		chartState.setMobile(false)

		return width < 1200 ? 205 : 240
	}

	if (width < 1200) {
		chartState.setMobileChanged(chartState.getChartConfig().mobile ? false : true)

		chartState.setMobile(true)

		return height < 650 ? 125 : 180
	}
	chartState.setMobileChanged(chartState.getChartConfig().mobile ? true : false)

	chartState.setMobile(false)

	return 320
}

export function calculateDimensions(
	container: HTMLDivElement | null,
	svg: d3.Selection<SVGSVGElement, unknown, HTMLElement, any> | null
): d3.Selection<SVGSVGElement, unknown, HTMLElement, any> | null {
	if (!container) return svg

	const containerRect = container.getBoundingClientRect()

	chartState.setOuterPadding(containerRect.height < 430 ? 0.02 : 0.01)

	chartState.setMinBarWidth(getMinBarWidth(containerRect.width, containerRect.height))

	chartState.setWidth(containerRect.width)

	chartState.setHeight(containerRect.height * baseChartConfig.heightScale)

	if (!svg) return svg

	let dynamicChartConfig = chartState.getChartConfig()

	svg
		.attr('viewBox', [0, 0, dynamicChartConfig.width, dynamicChartConfig.height])
		.attr('width', dynamicChartConfig.width)
		.attr('height', dynamicChartConfig.height)

	return update_chart(dynamicChartConfig, svg, chartState.getData())
}
