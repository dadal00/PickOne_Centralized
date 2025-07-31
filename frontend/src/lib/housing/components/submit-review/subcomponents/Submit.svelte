<script lang="ts">
	import Button from '../../templates/Button.svelte'

	let isSubmitting = false
</script>

<div class="flex justify-center pt-6">
	<Button type="submit" className="bg-yellow-600 hover:bg-yellow-700 px-8 py-3 text-lg">
		{isSubmitting ? 'Submitting...' : 'Submit Review'}
	</Button>
</div>
