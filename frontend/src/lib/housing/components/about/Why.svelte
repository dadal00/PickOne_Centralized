<script lang="ts">
	import { Shield, Star, Target, Users } from '@lucide/svelte'
	import CardPiece from '../templates/CardPiece.svelte'

	const features = [
		{
			icon: Users,
			title: 'Student-Driven Reviews',
			description:
				'All reviews come from real Purdue students who have lived in these housing options.'
		},
		{
			icon: Shield,
			title: 'Anonymous & Safe',
			description: 'Share your honest experiences without revealing your identity.'
		},
		{
			icon: Star,
			title: 'Comprehensive Ratings',
			description: 'Rate housing across 5 key categories that matter most to students.'
		},
		{
			icon: Target,
			title: 'Purdue-Focused',
			description: 'Specifically designed for Purdue West Lafayette housing options.'
		}
	]
</script>

<div class="mb-20">
	<h2 class="text-3xl font-bold text-gray-900 text-center mb-12 dark:text-gray-100">
		Why Choose RateMyPWLHousing?
	</h2>
	<div class="grid grid-cols-1 md:grid-cols-2 gap-8">
		{#each features as feature (feature.title)}
			<CardPiece
				className="bg-white/80 backdrop-blur-sm shadow-xl border border-gray-100 rounded-2xl hover:shadow-2xl transition-shadow dark:bg-gray-800/80 dark:border-gray-700"
				cardPiece="cardCore"
			>
				<CardPiece cardPiece="cardHeader">
					<div class="flex items-center space-x-4">
						<div class="p-3 bg-gradient-to-br from-yellow-400 to-yellow-600 rounded-xl">
							<svelte:component this={feature.icon} class="h-6 w-6 text-white" />
						</div>
						<CardPiece
							className="text-xl font-bold text-gray-900 dark:text-gray-100"
							cardPiece="cardTitle">{feature.title}</CardPiece
						>
					</div>
				</CardPiece>
				<CardPiece cardPiece="cardContent">
					<p class="text-gray-700 dark:text-gray-300 leading-relaxed">{feature.description}</p>
				</CardPiece>
			</CardPiece>
		{/each}
	</div>
</div>
