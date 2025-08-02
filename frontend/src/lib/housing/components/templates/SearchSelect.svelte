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

	const newSelect = new Select<SelectType['value']>({
		value: getAction(),
		onValueChange: (value) => {
			setAction(value)
		},
		sameWidth: false
	})
</script>

<button
	{...newSelect.trigger}
	class="flex justify-between items-center rounded-xl border border-gray-200 focus:border-yellow-400 focus:ring-yellow-400 dark:bg-gray-700 dark:border-gray-600 dark:text-gray-200 px-3 py-2 text-left"
>
	{(selectOptions as SelectOptions[]).find((o) => o.value === newSelect.value)?.label ??
		'Select one'}
	<ChevronDown class="ml-3 h-4 w-4 opacity-50 right-2" />
</button>

<div
	{...newSelect.content}
	class="dark:bg-gray-800 dark:border-gray-700 rounded-md border bg-white mt-1 max-h-60 overflow-auto shadow-lg z-50"
>
	{#each selectOptions as SelectOptions[] as option}
		<div
			{...newSelect.getOption(option.value)}
			class="cursor-pointer select-none px-3 py-2 text-sm rounded-sm text-gray-900 dark:text-gray-100 hover:bg-yellow-300 dark:hover:bg-yellow-600"
		>
			{#if newSelect.value === option.value}
				<span class="inline-block mr-2 text-yellow-600 dark:text-yellow-300">✓</span>
			{/if}
			{option.label}
		</div>
	{/each}
</div>
