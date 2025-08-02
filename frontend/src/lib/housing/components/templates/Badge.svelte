<script lang="ts">
	import { badgeVariants } from '$lib/housing/variants/badges'
	import type { VariantProps } from 'class-variance-authority'
	import { cn } from '$lib/housing/helpers/utils'
	import type { Snippet } from 'svelte'

	const {
		variant = 'default',
		className = '',
		children
	} = $props<{
		variant?: VariantProps<typeof badgeVariants>['variant']
		className?: string
		children: Snippet
	}>()
</script>

<div class={cn(badgeVariants({ variant }), className)}>
	{@render children()}
</div>
