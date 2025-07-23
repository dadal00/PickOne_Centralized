<script lang="ts">
	import { cn } from '$lib/housing/utils'

	const { class: className = '', ...rest } = $props()
</script>

<div class={cn('flex flex-col space-y-1.5 p-6', className)} {...rest}></div>
