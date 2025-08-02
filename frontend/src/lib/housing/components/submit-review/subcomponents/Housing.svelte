<script lang="ts">
	import { appState } from '$lib/housing/app-state.svelte'
	import { writeReviewHousingSelect } from '$lib/housing/constants/housing'
	import HousingSelect from './HousingSelect.svelte'
</script>

<div class="grid gap-4">
	<div class="space-y-1.5">
		<div
			class="text-base font-semibold leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 dark:text-gray-300"
		>
			Housing *
			<HousingSelect
				selectOptions={writeReviewHousingSelect}
				getAction={appState.getWriteReviewHousing.bind(appState)}
				setAction={appState.setWriteReviewHousing.bind(appState)}
			/>
		</div>
	</div>
</div>
