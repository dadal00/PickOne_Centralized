<script>
	import BarChart from '$lib/home/components/BarChart.svelte'
	import Header from '$lib/home/components/Header.svelte'
</script>

<main class="flex flex-col w-screen h-screen bg-[#fff6ed] fixed py-8">
	<Header />
	<BarChart />
</main>

<style lang="postcss">
	@reference "tailwindcss";
</style>
