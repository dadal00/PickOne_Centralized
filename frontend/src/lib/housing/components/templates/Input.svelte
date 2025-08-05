<script lang="ts">
	import { PUBLIC_HOUSING_MIN_CHARS } from '$env/static/public'
	import { cn } from '$lib/housing/helpers/utils'

	let {
		type = 'text',
		className = '',
		disabled = false,
		query = $bindable(),
		placeholder
	} = $props<{
		type?: string
		className?: string
		disabled?: boolean
		query?: string
		placeholder: string
	}>()
</script>

<input
	{placeholder}
	{type}
	{disabled}
	class={cn(
		'flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-base ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 md:text-sm',
		className
	)}
	bind:value={query}
	maxlength={Number(PUBLIC_HOUSING_MIN_CHARS)}
/>
