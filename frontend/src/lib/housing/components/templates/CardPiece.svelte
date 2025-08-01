<script lang="ts">
	import { cn } from '$lib/housing/helpers/utils'
	import type { Snippet } from 'svelte'

	const {
		className = '',
		cardPiece,
		children
	} = $props<{
		className?: string
		cardPiece: string
		children: Snippet
	}>()

	const cardStyles: Record<string, string> = {
		cardCore: 'rounded-lg border bg-card text-card-foreground shadow-sm',
		cardContent: 'p-6 pt-0',
		cardDescription: 'text-sm text-muted-foreground',
		cardFooter: 'flex items-center p-6 pt-0',
		cardHeader: 'flex flex-col space-y-1.5 p-6',
		cardTitle: 'text-2xl font-semibold leading-none tracking-tight'
	}
</script>

<div class={cn(cardStyles[cardPiece], className)}>
	{@render children()}
</div>
