<script lang="ts">
	const { className, value } = $props<{
		className: string
		value: number
	}>()
</script>

<div
	class={`relative h-4 w-full overflow-hidden rounded-full bg-gray-200 ${className}`}
	role="progressbar"
	aria-valuenow={value}
	aria-valuemin="0"
	aria-valuemax="100"
>
	<div
		class="h-full w-full flex-1 bg-gradient-to-r from-yellow-400 to-yellow-600 transition-all rounded-full"
		style="transform: translateX(-{100 - value}%);"
	></div>
</div>
