<script lang="ts">
	import { ChevronDown } from '@lucide/svelte'
	import { Select } from 'melt/builders'

	const housingOptions = {
		'On-Campus': [
			'Cary Quadrangle',
			'McCutcheon Hall',
			'Tarkington Hall',
			'Wiley Hall',
			'Owen Hall',
			'Shreve Hall',
			'Earhart Hall',
			'Harrison Hall',
			'Hillenbrand Hall',
			'Meredith Hall',
			'Meredith South',
			'Windsor Halls',
			'First Street Towers',
			'Winifred Parker Hall',
			'Frieda Parker Hall',
			'Hawkins Hall'
		],
		'Off-Campus': [
			'Fuse Apartments',
			'Hub on Campus',
			'Rise on Chauncey',
			'Chauncey Square Apartments',
			'Lark West Lafayette',
			'Alight West Lafayette',
			'Redpoint West Lafayette',
			'Aspire at Discovery Park',
			'The Quarters',
			'Verve West Lafayette',
			'River Market Apartments',
			'Morris Rentals'
		]
	}

	const flatOptions = Object.values(housingOptions).flat()
	type Housing = (typeof flatOptions)[number]
	const housingSelect = new Select<Housing>()
</script>

<div class="grid gap-4">
	<div class="space-y-1.5">
		<div
			class="text-base font-semibold leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 dark:text-gray-300"
		>
			Housing *
			<button
				{...housingSelect.trigger}
				class="mt-1 flex text-sm justify-between rounded-md border border-gray-200 dark:border-gray-600 dark:bg-gray-700 font-medium dark:text-gray-200 px-3 py-2 w-full text-left"
			>
				{housingSelect.valueAsString === '' ? 'Select housing' : housingSelect.valueAsString}
				<ChevronDown class="h-4 w-4 opacity-50 right-2" />
			</button>

			<div
				{...housingSelect.content}
				class="mt-1 max-h-96 overflow-auto rounded-md border bg-white dark:bg-gray-800 dark:border-gray-700 shadow-lg z-50"
			>
				{#each Object.entries(housingOptions) as [group, options]}
					<div
						class="px-2 py-1.5 text-sm font-semibold text-gray-900 bg-gray-100 dark:bg-gray-700 dark:text-gray-200"
					>
						{group}
					</div>

					{#each options as option}
						<div
							{...housingSelect.getOption(option)}
							class="cursor-pointer select-none px-3 py-2 text-sm rounded-sm text-gray-900 dark:text-gray-100 hover:bg-yellow-300 dark:hover:bg-yellow-600"
						>
							{#if housingSelect.value === option}
								<span class="inline-block mr-2 text-yellow-600 dark:text-yellow-300">✓</span>
							{/if}
							{option}
						</div>
					{/each}
				{/each}
			</div>
		</div>
	</div>
</div>
