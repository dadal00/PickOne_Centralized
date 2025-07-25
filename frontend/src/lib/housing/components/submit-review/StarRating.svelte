<script lang="ts">
	import { StarIcon } from '@lucide/svelte'

	const { rating, action } = $props<{
		rating: Number
		action: (r: number) => void
	}>()
</script>

<div class="flex space-x-1">
	{#each [1, 2, 3, 4, 5] as star}
		<button type="button" class="focus:outline-none" onclick={() => action(star)}>
			<StarIcon
				class={`h-6 w-6 transition-colors ${star <= rating ? 'fill-yellow-500 text-yellow-500' : 'text-gray-300 dark:text-gray-600 hover:text-yellow-500 dark:hover:text-yellow-500'}`}
			/>
		</button>
	{/each}
</div>
