<script lang="ts">
	import { Search, MapPin, Home, Users, Star, ArrowRight } from '@lucide/svelte'
	import ThemeToggle from '$lib/housing/components/ThemeToggle.svelte'
	import Button from '$lib/housing/components/templates/Button.svelte'
	import Input from '$lib/housing/components/templates/Input.svelte'
	import Card from '$lib/housing/components/templates/card/Card.svelte'
	import CardContent from '$lib/housing/components/templates/card/CardContent.svelte'
	import CardHeader from '$lib/housing/components/templates/card/CardHeader.svelte'
	import CardTitle from '$lib/housing/components/templates/card/CardTitle.svelte'
	import { PUBLIC_SVELTE_HOUSING_ROOT } from '$env/static/public'

	let query = $state('')

	const featuredHousing = [
		{
			id: 1,
			name: 'Cary Quadrangle',
			type: 'On-Campus Dorm',
			rating: 4.2,
			reviewCount: 156
		},
		{
			id: 2,
			name: 'Fuse Apartments',
			type: 'Off-Campus Apartment',
			rating: 3.8,
			reviewCount: 89
		},
		{
			id: 3,
			name: 'Hub on Campus',
			type: 'Off-Campus Apartment',
			rating: 4.0,
			reviewCount: 124
		}
	]

	const stats = [
		{ label: 'Housing Options', value: '50+', icon: Home },
		{ label: 'Student Reviews', value: '2,500+', icon: Users },
		{ label: 'Average Rating', value: '3.9', icon: Star }
	]
</script>

<div
	class="min-h-screen bg-gradient-to-br from-slate-50 via-white to-yellow-50 dark:from-gray-900 dark:via-black dark:to-yellow-900/20"
>
	<header
		class="bg-white/80 backdrop-blur-md shadow-sm border-b border-gray-100 sticky top-0 z-50 dark:bg-gray-900/80 dark:border-gray-800"
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

	<section class="py-24 px-4 sm:px-6 lg:px-8">
		<div class="max-w-5xl mx-auto text-center">
			<div class="mb-8">
				<h2 class="text-6xl font-bold text-gray-900 dark:text-gray-100 mb-6 leading-tight">
					Find Your Perfect
					<span
						class="block bg-gradient-to-r from-yellow-500 to-yellow-600 bg-clip-text text-transparent"
						>Purdue Housing</span
					>
				</h2>
				<p class="text-xl text-gray-600 dark:text-gray-400 max-w-2xl mx-auto leading-relaxed">
					How to choose the best place to live in the land of corns 🌽
				</p>
			</div>

			<div class="max-w-2xl mx-auto mb-16">
				<div class="relative group">
					<div
						class="absolute inset-0 bg-gradient-to-r from-yellow-400 to-yellow-600 rounded-2xl blur opacity-25 group-hover:opacity-40 transition-opacity"
					></div>
					<div
						class="relative bg-white dark:bg-gray-800 rounded-2xl shadow-xl border border-gray-100 dark:border-gray-700"
					>
						<Search
							class="absolute left-6 top-1/2 transform -translate-y-1/2 text-gray-400 h-5 w-5"
						/>
						<Input
							type="text"
							placeholder="Search housing (e.g., Cary Quad, Fuse Apartments, McCutcheon Hall)"
							bind:query
							className="pl-14 pr-32 py-6 text-lg border-0 rounded-2xl focus:ring-2 focus:ring-yellow-500 bg-transparent"
						/>
						<a href={PUBLIC_SVELTE_HOUSING_ROOT + '/search'}>
							<Button
								className="absolute right-2 top-1/2 transform -translate-y-1/2 rounded-xl bg-gradient-to-r from-yellow-500 to-yellow-600 hover:from-yellow-600 hover:to-yellow-700 px-6 py-3 shadow-lg"
								>Search</Button
							>
						</a>
					</div>
				</div>
			</div>

			<div class="flex flex-wrap justify-center gap-4 mb-20">
				<a href={PUBLIC_SVELTE_HOUSING_ROOT + '/search'}>
					<Button
						variant="outline"
						className="rounded-full border-2 border-gray-200 hover:border-yellow-400 hover:bg-yellow-50 bg-white px-6 py-3 text-gray-700 hover:text-yellow-700 transition-all shadow-sm hover:shadow-md dark:bg-gray-800 dark:border-gray-700 dark:text-gray-300 dark:hover:bg-gray-700 dark:hover:border-yellow-500"
					>
						<Home class="h-4 w-4 mr-2" />
						On-Campus Dorms
					</Button>
				</a>
				<a href={PUBLIC_SVELTE_HOUSING_ROOT + '/search'}>
					<Button
						variant="outline"
						className="rounded-full border-2 border-gray-200 hover:border-yellow-400 hover:bg-yellow-50 bg-white px-6 py-3 text-gray-700 hover:text-yellow-700 transition-all shadow-sm hover:shadow-md dark:bg-gray-800 dark:border-gray-700 dark:text-gray-300 dark:hover:bg-gray-700 dark:hover:border-yellow-500"
					>
						<MapPin class="h-4 w-4 mr-2" />
						Off-Campus Apartments
					</Button>
				</a>
			</div>
		</div>
	</section>

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

	<section class="py-20 px-4 sm:px-6 lg:px-8">
		<div class="max-w-7xl mx-auto">
			<div class="text-center mb-16">
				<h3 class="text-4xl font-bold text-gray-900 dark:text-gray-100 mb-4">
					Popular Housing Options
				</h3>
				<p class="text-lg text-gray-600 dark:text-gray-400">
					Discover what fellow Boilermakers are saying
				</p>
			</div>
			<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
				{#each featuredHousing as housing}
					<Card
						class="group overflow-hidden hover:shadow-2xl transition-all duration-300 border-0 shadow-lg bg-white/80 backdrop-blur-sm hover:-translate-y-1 dark:bg-gray-800/80 dark:border dark:border-gray-700"
						key={housing.id}
					>
						<CardHeader class="pb-4">
							<div class="flex justify-between items-start mb-4">
								<div class="flex-1">
									<CardTitle
										class="text-2xl font-bold text-gray-900 group-hover:text-yellow-700 transition-colors mb-2 dark:text-gray-100 dark:group-hover:text-yellow-400"
									>
										{housing.name}
									</CardTitle>
									<p class="text-sm text-gray-500 dark:text-gray-400 font-medium">
										{housing.type}
									</p>
								</div>
								<div class="text-right ml-4">
									<div class="flex items-center space-x-2 mb-1">
										<Star class="h-6 w-6 fill-yellow-400 text-yellow-400" />
										<span class="text-2xl font-bold text-gray-900 dark:text-gray-100"
											>{housing.rating}</span
										>
									</div>
									<p class="text-sm text-gray-500 dark:text-gray-400">
										{housing.reviewCount} reviews
									</p>
								</div>
							</div>
						</CardHeader>
						<CardContent class="pt-0">
							<a href={PUBLIC_SVELTE_HOUSING_ROOT + '/housing'}>
								<Button
									className="w-full bg-gradient-to-r from-yellow-500 to-yellow-600 hover:from-yellow-600 hover:to-yellow-700 text-white font-semibold py-3 rounded-xl shadow-lg hover:shadow-xl transition-all group"
								>
									View Reviews
									<ArrowRight class="h-4 w-4 ml-2 group-hover:translate-x-1 transition-transform" />
								</Button>
							</a>
						</CardContent>
					</Card>
				{/each}
			</div>
		</div>
	</section>

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
</div>
