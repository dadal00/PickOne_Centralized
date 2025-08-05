<script lang="ts">
	import { PUBLIC_HOUSING_MAX_CHARS, PUBLIC_HOUSING_MIN_CHARS } from '$env/static/public'

	let { description = $bindable() } = $props<{
		description: string
	}>()
</script>

<div>
	<div
		class="leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-base font-semibold dark:text-gray-200"
	>
		Your Review *
		<p class=" mt-2 text-sm text-gray-600 dark:text-gray-400 mb-3">
			Share details about your experience
		</p>
		<textarea
			bind:value={description}
			placeholder="Tell us about your experience living here. What did you like? What could be improved? Any tips for future residents?"
			rows="6"
			required
			maxlength={Number(PUBLIC_HOUSING_MAX_CHARS)}
			minlength={Number(PUBLIC_HOUSING_MIN_CHARS)}
			class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 resize-none dark:bg-gray-700 dark:border-gray-600 dark:text-gray-200"
		></textarea>
		<p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
			{#if description.length >= Number(PUBLIC_HOUSING_MIN_CHARS)}
				Maximum {PUBLIC_HOUSING_MAX_CHARS} characters ({description.length}/{PUBLIC_HOUSING_MAX_CHARS})
			{:else}
				Minimum {PUBLIC_HOUSING_MIN_CHARS} characters ({description.length}/{PUBLIC_HOUSING_MIN_CHARS})
			{/if}
		</p>
	</div>
</div>
