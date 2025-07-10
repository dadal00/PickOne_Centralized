<script lang="ts">
	import { PUBLIC_SVELTE_SWAP_ROOT } from '$env/static/public'
</script>

<header class="border-b bg-white sticky top-0 z-50">
	<div class="container mx-auto px-4 py-4">
		<div class="flex items-center justify-between">
			<a href={PUBLIC_SVELTE_SWAP_ROOT} class="flex items-center space-x-2">
				<div
					class="w-8 h-8 bg-gradient-to-br from-yellow-400 to-amber-500 rounded-lg flex items-center justify-center"
				>
					<span class="text-white font-bold text-sm">BS</span>
				</div>
				<h1 class="text-2xl font-bold text-gray-900">BoilerSwap</h1>
			</a>

			<a
				href={PUBLIC_SVELTE_SWAP_ROOT}
				class="border border-gray-300 text-gray-700 hover:bg-gray-50 px-4 py-2 rounded"
			>
				← Back to Home
			</a>
		</div>
	</div>
</header>
