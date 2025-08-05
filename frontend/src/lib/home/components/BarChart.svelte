<script lang="ts">
	import * as d3 from 'd3'
	import { onDestroy, onMount } from 'svelte'
	import { calculateDimensions, chart_init, update_chart } from '../helpers/chart'
	import { type chartConfig, type ChartData } from '../models'
	import { chartState } from '../chart-state.svelte'

	const data: ChartData[] = $derived(chartState.getData())

	const dynamicChartConfig: chartConfig = $derived(chartState.getChartConfig())

	let container: HTMLDivElement | null = null
	let svg: d3.Selection<SVGSVGElement, unknown, HTMLElement, any> | null = null
	let resizeObserver: ResizeObserver | null = null
	let resizeHandler: () => void

	onMount(async () => {
		svg = update_chart(
			dynamicChartConfig,
			calculateDimensions(
				container,
				chart_init(dynamicChartConfig.width, dynamicChartConfig.height)
			),
			data
		)

		resizeHandler = () => {
			svg = calculateDimensions(container, svg)
		}

		resizeObserver = new ResizeObserver(resizeHandler)

		if (container) {
			resizeObserver.observe(container)
		}

		window.addEventListener('resize', resizeHandler)
	})

	$effect(() => {
		if (dynamicChartConfig.mobileChanged) {
			svg?.remove()

			svg = update_chart(
				dynamicChartConfig,
				chart_init(dynamicChartConfig.width, dynamicChartConfig.height),
				data
			)

			return
		}

		if (svg) {
			svg = update_chart(dynamicChartConfig, svg, data)
		}
	})

	onDestroy(() => {
		if (resizeObserver) {
			resizeObserver.disconnect()

			window.removeEventListener('resize', resizeHandler)
		}

		svg?.remove()
	})
</script>

<div
	class="w-full pl-[3vw] flex-1 h-full overflow-visible chart-container mt-6"
	bind:this={container}
>
	<div id="chart" aria-label="Live Website Visitors Chart"></div>
</div>
