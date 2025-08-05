<script lang="ts">
	import { appState } from '$lib/housing/app-state.svelte'
	import Button from '../../templates/Button.svelte'
</script>

<div class="flex justify-center pt-6">
	<Button
		type="submit"
		disabled={appState.getLimited()}
		className={appState.getLimited()
			? 'bg-gray-300 cursor-not-allowed'
			: 'bg-yellow-600 hover:bg-yellow-700 px-8 py-3 text-lg'}
	>
		Submit Review
	</Button>
</div>
