<script lang="ts">
	import { cn } from '$lib/housing/utils'

	const { class: className = '', ...rest } = $props()
</script>

<div
	class={cn('rounded-lg border bg-card text-card-foreground shadow-sm', className)}
	{...rest}
></div>
