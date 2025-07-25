<script lang="ts">
	import { ChevronDown, Funnel, Search } from '@lucide/svelte'
	import { Select } from 'melt/builders'

	let query = $state('')

	const housingOptions = [
		{ value: 'all', label: 'All Housing' },
		{ value: 'on-campus', label: 'On-Campus Dorms' },
		{ value: 'off-campus', label: 'Off-Campus Apartments' }
	]
	type Housing = (typeof housingOptions)[number]
	const housingSelect = new Select<Housing['value']>()
	const sortingOptions = [
		{ value: 'rating', label: 'Highly Rated' },
		{ value: 'reviews', label: 'Most Reviews' },
		{ value: 'name', label: 'Name A-Z' }
	]
	type Sorting = (typeof sortingOptions)[number]
	const sortingSelect = new Select<Sorting['value']>()
	// let typeFilter = $derived(select.value)
</script>

<div
	class="bg-white/80 backdrop-blur-sm rounded-2xl shadow-xl border border-gray-100 p-8 mb-8 dark:bg-gray-800/80 dark:border-gray-700"
>
	<div class="flex items-center space-x-3 mb-6">
		<Funnel class="h-5 w-5 text-yellow-600" />
		<h2 class="text-lg font-semibold text-gray-900 dark:text-gray-100">Search & Filter</h2>
	</div>
	<div class="grid grid-cols-1 md:grid-cols-4 gap-4 flex items-center">
		<div class="md:col-span-2">
			<div class="relative">
				<Search class="absolute left-4 top-1/2 transform -translate-y-1/2 text-gray-400 h-5 w-5" />
				<input
					type="text"
					bind:value={query}
					placeholder="Search housing..."
					class={'flex h-10 w-full border border-gray-200 bg-background px-3 pl-12 py-3 text-base md:text-sm file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground rounded-xl focus-visible:outline-none focus:border-yellow-400 focus:ring-2 focus:ring-yellow-400 disabled:cursor-not-allowed disabled:opacity-50 dark:bg-gray-700 dark:border-gray-600 dark:text-gray-200'}
				/>
			</div>
		</div>

		<button
			{...housingSelect.trigger}
			class="flex justify-between items-center rounded-xl border border-gray-200 focus:border-yellow-400 focus:ring-yellow-400 dark:bg-gray-700 dark:border-gray-600 dark:text-gray-200 px-3 py-2 w-full text-left"
		>
			{housingOptions.find((o) => o.value === housingSelect.value)?.label ??
				'Select a housing type'}
			<ChevronDown class="h-4 w-4 opacity-50 right-2" />
		</button>

		<div
			{...housingSelect.content}
			class="dark:bg-gray-800 dark:border-gray-700 rounded-md border bg-white mt-1 max-h-60 overflow-auto shadow-lg z-50"
		>
			{#each housingOptions as option}
				<div
					{...housingSelect.getOption(option.value)}
					class="cursor-pointer select-none px-3 py-2 text-sm rounded-sm text-gray-900 dark:text-gray-100 hover:bg-yellow-300 dark:hover:bg-yellow-600"
				>
					{#if housingSelect.value === option.value}
						<span class="inline-block mr-2 text-yellow-600 dark:text-yellow-300">✓</span>
					{/if}
					{option.label}
				</div>
			{/each}
		</div>

		<button
			{...sortingSelect.trigger}
			class="flex justify-between items-center rounded-xl border border-gray-200 focus:border-yellow-400 focus:ring-yellow-400 dark:bg-gray-700 dark:border-gray-600 dark:text-gray-200 px-3 py-2 w-full text-left"
		>
			{sortingOptions.find((o) => o.value === sortingSelect.value)?.label ??
				'Select a housing type'}
			<ChevronDown class="h-4 w-4 opacity-50 right-2" />
		</button>

		<div
			{...sortingSelect.content}
			class="dark:bg-gray-800 dark:border-gray-700 rounded-md border bg-white mt-1 max-h-60 overflow-auto shadow-lg z-50"
		>
			{#each sortingOptions as option}
				<div
					{...sortingSelect.getOption(option.value)}
					class="cursor-pointer select-none px-3 py-2 text-sm rounded-sm text-gray-900 dark:text-gray-100 hover:bg-yellow-300 dark:hover:bg-yellow-600"
				>
					{#if sortingSelect.value === option.value}
						<span class="inline-block mr-2 text-yellow-600 dark:text-yellow-300">✓</span>
					{/if}
					{option.label}
				</div>
			{/each}
		</div>
	</div>
</div>
