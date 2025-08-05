<script lang="ts">
	import { PUBLIC_SVELTE_HOUSING_ROOT } from '$env/static/public'
	import { Home } from '@lucide/svelte'
	import ThemeToggle from '../ThemeToggle.svelte'
</script>

<header
	class="bg-white/80 backdrop-blur-md shadow-sm border-b border-gray-100 sticky top-0 z-50 dark:bg-gray-900 dark:border-gray-800"
>
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="flex justify-between items-center h-16">
			<a href={PUBLIC_SVELTE_HOUSING_ROOT} class="flex items-center space-x-3">
				<div class="p-2 bg-gradient-to-br from-yellow-400 to-yellow-600 rounded-xl">
					<Home class="h-6 w-6 text-white" />
				</div>
				<h1
					class="text-2xl font-bold bg-gradient-to-r from-gray-900 to-gray-700 bg-clip-text text-transparent dark:from-gray-100 dark:to-gray-300"
				>
					RateMyPWLHousing
				</h1>
			</a>
			<div class="flex items-center space-x-4">
				<nav class="hidden md:flex space-x-8">
					<a
						href={PUBLIC_SVELTE_HOUSING_ROOT}
						class="text-gray-700 hover:text-yellow-600 font-medium transition-colors dark:text-gray-300 dark:hover:text-yellow-400"
						>Home</a
					>
					<a
						href={PUBLIC_SVELTE_HOUSING_ROOT + '/search'}
						class="text-gray-700 hover:text-yellow-600 font-medium transition-colors dark:text-gray-300 dark:hover:text-yellow-400"
						>Search Housing</a
					>
					<a
						href={PUBLIC_SVELTE_HOUSING_ROOT + '/submit-review'}
						class="text-gray-700 hover:text-yellow-600 font-medium transition-colors dark:text-gray-300 dark:hover:text-yellow-400"
						>Write Review</a
					>
					<a
						href={PUBLIC_SVELTE_HOUSING_ROOT + '/about'}
						class="text-gray-700 hover:text-yellow-600 font-medium transition-colors dark:text-gray-300 dark:hover:text-yellow-400"
						>About Us</a
					>
				</nav>
				<ThemeToggle />
			</div>
		</div>
	</div>
</header>
