<script lang="ts">
	import StarRating from '$lib/housing/components/submit-review/StarRating.svelte'
	import Button from '$lib/housing/components/templates/Button.svelte'
	import CardPiece from '$lib/housing/components/templates/CardPiece.svelte'
	import { ChevronDown, Send } from '@lucide/svelte'
	import { Select } from 'melt/builders'

	let selectedHousing = ''
	let semester = ''
	let roomType = ''
	let overallRating = 0
	let isSubmitting = false
	let comment = ''

	let categoryRatings: Record<string, number> = {
		livingConditions: 0,
		location: 0,
		amenities: 0,
		value: 0,
		community: 0
	}

	const housingOptions = {
		'On-Campus': [
			'Cary Quadrangle',
			'McCutcheon Hall',
			'Tarkington Hall',
			'Wiley Hall',
			'Owen Hall',
			'Shreve Hall',
			'Earhart Hall',
			'Harrison Hall',
			'Hillenbrand Hall',
			'Meredith Hall',
			'Meredith South',
			'Windsor Halls',
			'First Street Towers',
			'Winifred Parker Hall',
			'Frieda Parker Hall',
			'Hawkins Hall'
		],
		'Off-Campus': [
			'Fuse Apartments',
			'Hub on Campus',
			'Rise on Chauncey',
			'Chauncey Square Apartments',
			'Lark West Lafayette',
			'Alight West Lafayette',
			'Redpoint West Lafayette',
			'Aspire at Discovery Park',
			'The Quarters',
			'Verve West Lafayette',
			'River Market Apartments',
			'Morris Rentals'
		]
	}

	const ratingCategories = [
		{
			key: 'livingConditions',
			label: 'Living Conditions',
			description: 'Cleanliness, maintenance, noise levels, room condition, AC/heating'
		},
		{
			key: 'location',
			label: 'Location & Accessibility',
			description: 'Proximity to campus, bus stops, groceries, accessibility features'
		},
		{
			key: 'amenities',
			label: 'Amenities',
			description: 'Laundry, internet, study areas, gym, kitchen, parking, security'
		},
		{
			key: 'value',
			label: 'Value & Cost',
			description: 'Rent vs quality, utility inclusions, lease flexibility'
		},
		{
			key: 'community',
			label: 'Community & Social',
			description: 'Resident community, events, staff friendliness'
		}
	]

	const handleSubmit = async () => {
		alert('Review submitted successfully! Thank you for helping fellow students.')
	}

	const flatOptions = Object.values(housingOptions).flat()
	type Housing = (typeof flatOptions)[number]
	const housingSelect = new Select<Housing>()

	const semesterOptions = [
		{ value: 'fall-2024', label: 'Fall 2024' },
		{ value: 'spring-2024', label: 'Spring 2024' },
		{ value: 'fall-2023', label: 'Fall 2023' }
	]
	type Semesters = (typeof semesterOptions)[number]
	const semesterSelect = new Select<Semesters['value']>()

	const roomOptions = [
		{ value: 'single', label: 'Single' },
		{ value: 'double', label: 'Double' },
		{ value: 'triple', label: 'Triple' }
	]
	type Rooms = (typeof roomOptions)[number]
	const roomsSelect = new Select<Rooms['value']>()
</script>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900">
	<div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
		<div class="text-center mb-8">
			<h2 class="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-4">
				Share Your Housing Experience
			</h2>
			<p class="text-lg text-gray-600 dark:text-gray-400">
				Help fellow Boilermakers make informed housing decisions with your honest review
			</p>
		</div>

		<CardPiece className="dark:bg-gray-800 dark:border-gray-700" cardPiece="cardCore">
			<CardPiece cardPiece="cardHeader">
				<CardPiece className="flex items-center space-x-2 dark:text-gray-100" cardPiece="cardTitle">
					<Send class="h-5 w-5" />
					<span>Submit Your Review</span>
				</CardPiece>
			</CardPiece>
			<CardPiece cardPiece="cardContent">
				<form on:submit|preventDefault={handleSubmit} class="space-y-6">
					<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
						<div class="space-y-1.5">
							<label
								class="text-base font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 dark:text-gray-300"
							>
								Housing *
								<button
									{...housingSelect.trigger}
									class="flex justify-between rounded-md border border-gray-200 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-200 px-3 py-2 w-full text-left"
								>
									{housingSelect.valueAsString === ''
										? 'Select housing'
										: housingSelect.valueAsString}
									<ChevronDown class="h-4 w-4 opacity-50 right-2" />
								</button>

								<div
									{...housingSelect.content}
									class="mt-1 max-h-96 overflow-auto rounded-md border bg-white dark:bg-gray-800 dark:border-gray-700 shadow-lg z-50"
								>
									{#each Object.entries(housingOptions) as [group, options]}
										<div
											class="px-2 py-1.5 text-sm font-semibold text-gray-900 bg-gray-100 dark:bg-gray-700 dark:text-gray-200"
										>
											{group}
										</div>

										{#each options as option}
											<div
												{...housingSelect.getOption(option)}
												class="cursor-pointer select-none px-3 py-2 text-sm rounded-sm text-gray-900 dark:text-gray-100 hover:bg-yellow-300 dark:hover:bg-yellow-600"
											>
												{#if housingSelect.value === option}
													<span class="inline-block mr-2 text-yellow-600 dark:text-yellow-300"
														>✓</span
													>
												{/if}
												{option}
											</div>
										{/each}
									{/each}
								</div>
							</label>
						</div>
						<div class="space-y-1.5">
							<label
								class="text-base font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 dark:text-gray-300"
							>
								Semester *
								<button
									{...semesterSelect.trigger}
									class="flex justify-between rounded-md border border-gray-200 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-200 px-3 py-2 w-full text-left"
								>
									{semesterSelect.valueAsString === ''
										? 'Select housing'
										: semesterSelect.valueAsString}
									<ChevronDown class="h-4 w-4 opacity-50 right-2" />
								</button>

								<div
									{...semesterSelect.content}
									class="mt-1 max-h-96 overflow-auto rounded-md border bg-white dark:bg-gray-800 dark:border-gray-700 shadow-lg z-50"
								>
									{#each semesterOptions as option}
										<div
											{...semesterSelect.getOption(option.value)}
											class="cursor-pointer select-none px-3 py-2 text-sm rounded-sm text-gray-900 dark:text-gray-100 hover:bg-yellow-300 dark:hover:bg-yellow-600"
										>
											{#if semesterSelect.value === option.value}
												<span class="inline-block mr-2 text-yellow-600 dark:text-yellow-300">✓</span
												>
											{/if}
											{option.label}
										</div>
									{/each}
								</div>
							</label>
						</div>
						<div class="space-y-1.5">
							<label
								class="text-base font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 dark:text-gray-300"
							>
								Room *
								<button
									{...roomsSelect.trigger}
									class="flex justify-between rounded-md border border-gray-200 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-200 px-3 py-2 w-full text-left"
								>
									{roomsSelect.valueAsString === '' ? 'Select housing' : roomsSelect.valueAsString}
									<ChevronDown class="h-4 w-4 opacity-50 right-2" />
								</button>

								<div
									{...roomsSelect.content}
									class="mt-1 max-h-96 overflow-auto rounded-md border bg-white dark:bg-gray-800 dark:border-gray-700 shadow-lg z-50"
								>
									{#each roomOptions as option}
										<div
											{...roomsSelect.getOption(option.value)}
											class="cursor-pointer select-none px-3 py-2 text-sm rounded-sm text-gray-900 dark:text-gray-100 hover:bg-yellow-300 dark:hover:bg-yellow-600"
										>
											{#if roomsSelect.value === option.value}
												<span class="inline-block mr-2 text-yellow-600 dark:text-yellow-300">✓</span
												>
											{/if}
											{option.label}
										</div>
									{/each}
								</div>
							</label>
						</div>
					</div>

					<div>
						<label
							class="leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-base font-semibold dark:text-gray-200"
							>Overall Rating *
							<p class="text-sm text-gray-600 dark:text-gray-400 mb-3">
								How would you rate your overall experience?
							</p>
							<div class="flex items-center space-x-4">
								<StarRating rating={overallRating} action={(r) => (overallRating = r)} />
								{#if overallRating > 0}
									<span class="text-lg font-semibold dark:text-gray-200">{overallRating}/5</span>
								{/if}
							</div>
						</label>
					</div>

					<div>
						<label
							class="mb-4 block leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-base font-semibold dark:text-gray-200"
							>Rate Each Category *
							<div class="space-y-4">
								{#each ratingCategories as category}
									<div class="border rounded-lg p-4 dark:border-gray-700">
										<div class="flex justify-between items-start mb-2">
											<div>
												<h4 class="font-medium dark:text-gray-200">{category.label}</h4>
												<p class="text-sm text-gray-600 dark:text-gray-400">
													{category.description}
												</p>
											</div>
											<div class="flex items-center space-x-2">
												<StarRating
													rating={categoryRatings[category.key]}
													action={(r) => (categoryRatings[category.key] = r)}
												/>
												<span
													class="text-sm font-medium w-8 dark:text-gray-300 transition-opacity duration-200"
													style="opacity: {categoryRatings[category.key] > 0 ? 1 : 0}"
												>
													{categoryRatings[category.key]}/5
												</span>
											</div>
										</div>
									</div>
								{/each}
							</div>
						</label>
					</div>

					<div>
						<label
							class="leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-base font-semibold dark:text-gray-200"
							>Your Review *
							<p class="text-sm text-gray-600 dark:text-gray-400 mb-3">
								Share details about your experience
							</p>
							<textarea
								id="comment"
								bind:value={comment}
								placeholder="Tell us about your experience living here. What did you like? What could be improved? Any tips for future residents?"
								rows="6"
								required
								class="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 resize-none dark:bg-gray-700 dark:border-gray-600 dark:text-gray-200"
							></textarea>
							<p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
								Minimum 50 characters ({comment.length}/50)
							</p>
						</label>
					</div>

					<div class="flex justify-center pt-6">
						<Button
							type="submit"
							className="bg-yellow-600 hover:bg-yellow-700 px-8 py-3 text-lg"
							disabled={!selectedHousing ||
								!semester ||
								!roomType ||
								overallRating === 0 ||
								Object.values(categoryRatings).some((r) => r === 0) ||
								comment.length < 50 ||
								isSubmitting}
						>
							{isSubmitting ? 'Submitting...' : 'Submit Review'}
						</Button>
					</div>
				</form>
			</CardPiece>
		</CardPiece>

		<div class="mt-8 text-center text-sm text-gray-500 dark:text-gray-400">
			<p>
				Reviews are anonymous and help create a better housing experience for all Purdue students.
			</p>
		</div>
	</div>
</div>
