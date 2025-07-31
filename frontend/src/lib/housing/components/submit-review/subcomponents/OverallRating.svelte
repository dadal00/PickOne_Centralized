<script lang="ts">
	import StarRating from '../StarRating.svelte'

	let overallRating = 0
</script>

<div>
	<div
		class="leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-base font-semibold dark:text-gray-200"
	>
		Overall Rating *
		<p class="text-sm text-gray-600 dark:text-gray-400 mb-3">
			How would you rate your overall experience?
		</p>
		<div class="flex items-center space-x-4">
			<StarRating rating={overallRating} action={(r) => (overallRating = r)} />
			{#if overallRating > 0}
				<span class="text-lg font-semibold dark:text-gray-200">{overallRating}/5</span>
			{/if}
		</div>
	</div>
</div>
