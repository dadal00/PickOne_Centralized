<script lang="ts">
	import { appState } from '$lib/swap/app-state.svelte'
	import { Status } from '$lib/swap/models'
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

			<nav class="hidden md:flex items-center space-x-6">
				<a href={PUBLIC_SVELTE_SWAP_ROOT + '/browse'} class="text-gray-600 hover:text-gray-900"
					>Browse</a
				>
				<a href={PUBLIC_SVELTE_SWAP_ROOT + '/post'} class="text-gray-600 hover:text-gray-900"
					>Post Item</a
				>
				<a
					href={PUBLIC_SVELTE_SWAP_ROOT + '/how-it-works'}
					class="text-gray-600 hover:text-gray-900">How It Works</a
				>
				<a href={PUBLIC_SVELTE_SWAP_ROOT + '/about'} class="text-gray-600 hover:text-gray-900"
					>About</a
				>
			</nav>

			<div class="flex items-center space-x-3">
				<a
					href={PUBLIC_SVELTE_SWAP_ROOT + '/auth'}
					class="border border-gray-300 text-gray-700 hover:bg-gray-50 px-4 py-2 rounded"
					>Sign {appState.getStatus(Status.isSignedIn) ? 'Out' : 'In'}</a
				>
				<a
					href={PUBLIC_SVELTE_SWAP_ROOT + '/post'}
					class="bg-gradient-to-r from-yellow-400 to-amber-500 hover:from-yellow-500 hover:to-amber-600 text-white px-4 py-2 rounded"
					>+ Post Item</a
				>
			</div>
		</div>
	</div>
</header>
