<script lang="ts">
	import { cn } from '$lib/housing/utils'

	const { class: className = '', ...rest } = $props()
</script>

<div class={cn('flex items-center p-6 pt-0', className)} {...rest}></div>
