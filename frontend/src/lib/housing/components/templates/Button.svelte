<script lang="ts">
	import { buttonVariants } from '$lib/housing/variants/buttonVariants'
	import type { VariantProps } from 'class-variance-authority'
	import type { Snippet } from 'svelte'
	import { cn } from '$lib/housing/utils'

	const {
		variant = 'default',
		size = 'default',
		asChild = false,
		className = '',
		disabled = false,
		type = 'button',
		ariaLabel,
		children,
		action
	} = $props<{
		variant?: VariantProps<typeof buttonVariants>['variant']
		size?: VariantProps<typeof buttonVariants>['size']
		asChild?: boolean
		className?: string
		disabled?: boolean
		type?: 'button' | 'submit' | 'reset'
		ariaLabel?: string
		children: Snippet
		action?: (event: MouseEvent) => void
	}>()
</script>

{#if asChild}
	{@render children()}
{:else}
	<button
		onclick={action}
		{type}
		class={cn(buttonVariants({ variant, size, className }))}
		{disabled}
		aria-label={ariaLabel}
	>
		{@render children()}
	</button>
{/if}
