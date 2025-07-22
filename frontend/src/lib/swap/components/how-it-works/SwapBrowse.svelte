<script lang="ts">
	import { PUBLIC_SVELTE_SWAP_ROOT } from '$env/static/public'
</script>

<div class="text-center mt-16 p-8 bg-yellow-50 rounded-lg">
	<h2 class="text-2xl font-bold text-gray-900 mb-4">Ready to Start Swapping?</h2>
	<p class="text-gray-600 mb-6">Join hundreds of Purdue students making campus more sustainable</p>
	<div class="flex flex-col sm:flex-row gap-4 justify-center">
		<a
			href={PUBLIC_SVELTE_SWAP_ROOT + '/post'}
			class="bg-yellow-400 text-gray-800 hover:bg-yellow-500 px-8 py-3 rounded-lg font-medium"
			>Post Your First Item</a
		>
		<a
			href={PUBLIC_SVELTE_SWAP_ROOT + '/browse'}
			class="border border-gray-300 text-gray-700 hover:bg-gray-50 px-8 py-3 rounded-lg font-medium"
			>Browse Available Items</a
		>
	</div>
</div>
