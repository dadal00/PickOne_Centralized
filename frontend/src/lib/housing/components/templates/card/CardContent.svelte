<script lang="ts">
	import { cn } from '$lib/housing/utils'

	const { class: className = '', ...rest } = $props()
</script>

<div class={cn('p-6 pt-0', className)} {...rest}></div>
