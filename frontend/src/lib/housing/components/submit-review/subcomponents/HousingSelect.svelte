<script lang="ts">
	import { type SelectOptions } from '$lib/housing/models/general'
	import { ChevronDown } from '@lucide/svelte'
	import { Select } from 'melt/builders'

	const { selectOptions, getAction, setAction } = $props<{
		selectOptions: SelectOptions[]
		getAction: Function
		setAction: Function
	}>()

	type SelectType = (typeof selectOptions)[number]

	const housingSelect = new Select<SelectType['value']>({
		value: getAction(),
		onValueChange: (value) => {
			setAction(value)
		}
	})
</script>

<button
	{...housingSelect.trigger}
	class="mt-1 flex text-sm justify-between rounded-md border border-gray-200 dark:border-gray-600 dark:bg-gray-700 font-medium dark:text-gray-200 px-3 py-2 w-full text-left"
>
	{(selectOptions as SelectOptions[]).find((o) => o.value === housingSelect.value)?.label ??
		'Select your housing'}
	<ChevronDown class="h-4 w-4 opacity-50 right-2" />
</button>

<div
	{...housingSelect.content}
	class="mt-1 max-h-96 overflow-auto rounded-md border bg-white dark:bg-gray-800 dark:border-gray-700 shadow-lg z-50"
>
	{#each selectOptions as SelectOptions[] as option}
		{#if option.value === 'On-Campus' || option.value === 'Off-Campus'}
			<div
				class="px-2 py-1.5 text-sm font-semibold text-gray-900 bg-gray-100 dark:bg-gray-700 dark:text-gray-200"
			>
				{option.label}
			</div>
		{:else}
			<div
				{...housingSelect.getOption(option.value)}
				class="cursor-pointer select-none px-3 py-2 text-sm rounded-sm text-gray-900 dark:text-gray-100 hover:bg-yellow-300 dark:hover:bg-yellow-600"
			>
				{#if housingSelect.value === option.value}
					<span class="inline-block mr-2 text-yellow-600 dark:text-yellow-300">✓</span>
				{/if}
				{option.label}
			</div>
		{/if}
	{/each}
</div>
