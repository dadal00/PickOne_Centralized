<script lang="ts">
	import { Home, Star, Users } from '@lucide/svelte'

	const stats = [
		{ label: 'Housing Options', value: '-1', icon: Home },
		{ label: 'Student Reviews', value: '-1', icon: Users },
		{ label: 'Average Rating', value: '-1', icon: Star }
	]
</script>

<section class="py-20 bg-transparent backdrop-blur-sm">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="grid grid-cols-1 md:grid-cols-3 gap-8">
			{#each stats as stat}
				<div class="text-center group">
					<div class="flex justify-center mb-6">
						<div
							class="p-4 bg-gradient-to-br from-yellow-400 to-yellow-600 rounded-2xl shadow-lg group-hover:shadow-xl transition-shadow"
						>
							<stat.icon class="h-8 w-8 text-white" />
						</div>
					</div>
					<div class="text-4xl font-bold text-gray-900 dark:text-gray-100 mb-2">
						{stat.value}
					</div>
					<div class="text-gray-600 dark:text-gray-400 font-medium">
						{stat.label}
					</div>
				</div>
			{/each}
		</div>
	</div>
</section>
