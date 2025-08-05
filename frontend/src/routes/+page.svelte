<script lang="ts">
	import BarChart from '$lib/home/components/BarChart.svelte'
	import Header from '$lib/home/components/Header.svelte'
</script>

<svelte:head>
	<title>PickOne</title>
	<meta name="description" content="Find a service. Choose a service" />
</svelte:head>

<main
	class="flex flex-col w-screen bg-[#fff6ed] fixed py-8"
	style="height: calc(var(--vh, 1vh) * 100)"
>
	<Header />
	<BarChart />
</main>

<style lang="postcss">
	@reference "tailwindcss";
</style>
