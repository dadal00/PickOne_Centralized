<script lang="ts">
	import { cn } from '$lib/housing/utils'

	const { class: className = '', ...rest } = $props()
</script>

<div class={cn('text-sm text-muted-foreground', className)} {...rest}></div>
