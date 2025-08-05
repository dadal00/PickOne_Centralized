<script lang="ts">
	import { cn } from '$lib/housing/helpers/utils'
	import type { Snippet } from 'svelte'

	let {
		className = '',
		tabPiece,
		tabValue,
		activeTab = $bindable(),
		activeClasses,
		children
	} = $props<{
		className?: string
		tabPiece: 'list' | 'trigger' | 'content'
		tabValue?: string
		activeTab?: string
		activeClasses?: string
		children: Snippet
	}>()

	const styles: Record<string, string> = {
		list: 'inline-flex h-10 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground',
		trigger:
			'inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50',
		content:
			'mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2'
	}

	function handleClick() {
		activeTab = tabValue
	}
</script>

{#if tabPiece === 'content' || tabPiece === 'list'}
	<div class={cn(styles[tabPiece], className)}>
		{@render children()}
	</div>
{:else}
	<button
		class={cn(
			styles.trigger,
			className,
			tabValue === activeTab && `bg-background text-foreground shadow-sm ${activeClasses}`
		)}
		onclick={handleClick}
	>
		{@render children()}
	</button>
{/if}
