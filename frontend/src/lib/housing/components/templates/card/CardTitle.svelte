<script lang="ts">
	import { cn } from '$lib/housing/utils'

	const { class: className = '', ...rest } = $props()
</script>

<div class={cn('text-2xl font-semibold leading-none tracking-tight', className)} {...rest}></div>
