<script lang="ts">
	import { MapPin, Home, ArrowRight } from '@lucide/svelte'
	import { PUBLIC_SVELTE_HOUSING_ROOT } from '$env/static/public'
</script>

<footer class="bg-gradient-to-r from-gray-900 to-gray-800 text-white py-16">
	<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
		<div class="grid grid-cols-1 md:grid-cols-3 gap-12">
			<div>
				<div class="flex items-center space-x-3 mb-6">
					<div class="p-2 bg-gradient-to-br from-yellow-400 to-yellow-600 rounded-xl">
						<Home class="h-6 w-6 text-white" />
					</div>
					<h3 class="text-xl font-bold">RateMyPWLHousing</h3>
				</div>
				<p class="text-gray-300 leading-relaxed">
					Helping Purdue students find their perfect home away from home.
				</p>
			</div>
			<div>
				<h4 class="font-bold mb-6 text-lg">Quick Links</h4>
				<ul class="space-y-3 text-gray-300">
					<li>
						<a
							href={PUBLIC_SVELTE_HOUSING_ROOT + '/search'}
							class="hover:text-yellow-400 transition-colors flex items-center"
						>
							<ArrowRight class="h-4 w-4 mr-2" />
							Search Housing
						</a>
					</li>
					<li>
						<a
							href={PUBLIC_SVELTE_HOUSING_ROOT + '/submit-review'}
							class="hover:text-yellow-400 transition-colors flex items-center"
						>
							<ArrowRight class="h-4 w-4 mr-2" />
							Write a Review
						</a>
					</li>
					<li>
						<a
							href={PUBLIC_SVELTE_HOUSING_ROOT + '/about'}
							class="hover:text-yellow-400 transition-colors flex items-center"
						>
							<ArrowRight class="h-4 w-4 mr-2" />
							About Us
						</a>
					</li>
				</ul>
			</div>
			<div>
				<h4 class="font-bold mb-6 text-lg">Housing Types</h4>
				<ul class="space-y-3 text-gray-300">
					<li>
						<a
							href={PUBLIC_SVELTE_HOUSING_ROOT + '/search'}
							class="hover:text-yellow-400 transition-colors flex items-center"
						>
							<Home class="h-4 w-4 mr-2" />
							On-Campus Dorms
						</a>
					</li>
					<li>
						<a
							href={PUBLIC_SVELTE_HOUSING_ROOT + '/search'}
							class="hover:text-yellow-400 transition-colors flex items-center"
						>
							<MapPin class="h-4 w-4 mr-2" />
							Off-Campus Apartments
						</a>
					</li>
				</ul>
			</div>
		</div>
		<div class="border-t border-gray-700 mt-12 pt-8 text-center text-gray-400">
			<p>&copy; 2025 RateMyPWLHousing. Made by students, for students.</p>
		</div>
	</div>
</footer>
