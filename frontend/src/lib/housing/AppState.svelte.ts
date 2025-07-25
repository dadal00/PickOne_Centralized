class AppState {
	private housingDetails: Record<
		string,
		{
			id: string
			name: string
			type: string
			category: string
			rating: number
			reviewCount: number
			priceRangeSymbol: string
			priceRange: string
			walkTime: string
			address: string
			description: string
			amenities: string[]
			ratings: {
				livingConditions: number
				location: number
				amenities: number
				value: number
				community: number
			}
		}
	> = {
		'1': {
			id: '1',
			name: 'Cary Quadrangle',
			type: 'On-Campus Dorm',
			category: 'on-campus',
			rating: 4.2,
			reviewCount: 156,
			priceRangeSymbol: '$$$',
			priceRange: '$8,000-$10,000/year',
			walkTime: '5 min to campus',
			address: '1131 3rd Street, West Lafayette, IN 47907',
			description:
				"Cary Quadrangle is one of Purdue's most iconic residence halls, featuring traditional dormitory living with a strong sense of community.",
			amenities: [
				'Dining Court',
				'Study Lounges',
				'Laundry Facilities',
				'24/7 Front Desk',
				'Bike Storage'
			],
			ratings: {
				livingConditions: 4.1,
				location: 4.8,
				amenities: 3.9,
				value: 4.0,
				community: 4.5
			}
		},
		'2': {
			id: '2',
			name: 'Wiley Hall',
			type: 'On-Campus Dorm',
			category: 'on-campus',
			rating: 3.8,
			reviewCount: 122,
			priceRangeSymbol: '$',
			priceRange: '$7,800/year',
			walkTime: '7 min to campus',
			address: '401 N Russell St, West Lafayette, IN 47906',
			description:
				'Wiley Hall is a vibrant and social dorm known for its close-knit floor communities and convenient central campus location.',
			amenities: [
				'Air Conditioning',
				'Community Kitchens',
				'Game Room',
				'Lounge Areas',
				'Laundry Facilities'
			],
			ratings: {
				livingConditions: 3.7,
				location: 4.2,
				amenities: 3.6,
				value: 3.8,
				community: 4.1
			}
		},
		'3': {
			id: '3',
			name: 'Shreve Hall',
			type: 'On-Campus Dorm',
			category: 'on-campus',
			rating: 4.0,
			reviewCount: 145,
			priceRangeSymbol: '$$',
			priceRange: '$8,200/year',
			walkTime: '6 min to campus',
			address: '1275 3rd St, West Lafayette, IN 47906',
			description:
				'Shreve Hall offers a balance of quiet living and social opportunity, located close to the engineering buildings.',
			amenities: [
				'Air Conditioning',
				'In-Hall Dining Court',
				'Fitness Room',
				'Study Rooms',
				'Vending Areas'
			],
			ratings: {
				livingConditions: 4.0,
				location: 4.5,
				amenities: 4.1,
				value: 4.0,
				community: 4.2
			}
		},
		'4': {
			id: '4',
			name: 'Earhart Hall',
			type: 'On-Campus Dorm',
			category: 'on-campus',
			rating: 4.3,
			reviewCount: 110,
			priceRangeSymbol: '$',
			priceRange: '$8,000/year',
			walkTime: '8 min to campus',
			address: '1275 First Street, West Lafayette, IN 47906',
			description:
				'Earhart Hall is named after Amelia Earhart and is known for being a clean, modern dorm with a great dining court and strong academic focus.',
			amenities: [
				'Dining Court',
				'Study Rooms',
				'Air Conditioning',
				'Community Kitchen',
				'Front Desk Services'
			],
			ratings: {
				livingConditions: 4.3,
				location: 4.0,
				amenities: 4.2,
				value: 4.1,
				community: 4.0
			}
		},
		'5': {
			id: '5',
			name: 'Hillenbrand Hall',
			type: 'On-Campus Dorm',
			category: 'on-campus',
			rating: 4.4,
			reviewCount: 98,
			priceRangeSymbol: '$',
			priceRange: '$8,400/year',
			walkTime: '10 min to campus',
			address: '1301 Third Street, West Lafayette, IN 47906',
			description:
				'Hillenbrand is a suite-style residence hall with larger rooms and modern amenities, popular among upperclassmen and athletes.',
			amenities: [
				'Private Bathrooms',
				'Dining Court',
				'Fitness Room',
				'Indoor Lounge Areas',
				'Air Conditioning'
			],
			ratings: {
				livingConditions: 4.5,
				location: 3.9,
				amenities: 4.5,
				value: 4.2,
				community: 4.1
			}
		}
	}

	private reviews: Record<
		string,
		{
			id: number
			rating: number
			date: string
			semester: string
			roomType: string
			helpful: number
			notHelpful: number
			ratings: {
				livingConditions: number
				location: number
				amenities: number
				value: number
				community: number
			}
			comment: string
		}[]
	> = {
		'1': [
			{
				id: 1,
				rating: 5,
				date: '2024-12-15',
				semester: 'Fall 2024',
				roomType: 'Double',
				helpful: 12,
				notHelpful: 2,
				ratings: {
					livingConditions: 5,
					location: 5,
					amenities: 4,
					value: 4,
					community: 5
				},
				comment:
					"Absolutely loved living in Cary! The location is perfect - you can literally roll out of bed and be in class in 5 minutes. The community here is amazing, made so many lifelong friends. The dining court downstairs is super convenient. Only downside is the rooms are a bit small, but that's expected for traditional dorms. Would definitely recommend to incoming freshmen!"
			},
			{
				id: 2,
				rating: 3,
				date: '2024-11-28',
				semester: 'Fall 2024',
				roomType: 'Single',
				helpful: 8,
				notHelpful: 1,
				ratings: {
					livingConditions: 3,
					location: 5,
					amenities: 3,
					value: 3,
					community: 4
				},
				comment:
					"Mixed experience at Cary. Location is unbeatable and the people are great, but the building shows its age. My room had some maintenance issues that took a while to fix. The walls are pretty thin so it can get noisy, especially on weekends. Dining court food gets repetitive. It's a classic dorm experience but don't expect luxury."
			},
			{
				id: 3,
				rating: 4,
				date: '2024-10-10',
				semester: 'Fall 2024',
				roomType: 'Double',
				helpful: 15,
				notHelpful: 0,
				ratings: {
					livingConditions: 4,
					location: 5,
					amenities: 4,
					value: 4,
					community: 5
				},
				comment:
					"Great first-year experience! Cary has such a strong community feel. The RAs are awesome and there are always events happening. Yes, the rooms are small and the building is old, but that's part of the charm. The location cannot be beat - I never had to worry about being late to class. The dining court has decent variety. Perfect for freshmen who want the traditional college experience."
			}
		],
		'2': [
			{
				id: 1,
				rating: 4,
				date: '2025-01-10',
				semester: 'Spring 2025',
				roomType: 'Double',
				helpful: 10,
				notHelpful: 1,
				ratings: {
					livingConditions: 4,
					location: 4,
					amenities: 4,
					value: 4,
					community: 5
				},
				comment:
					'Stayed in Hawkins for a semester—overall really solid. Rooms were decently sized and felt newer than some other dorms. Community was welcoming and had quiet hours that were actually respected. Proximity to the bus stop was great, and the bathrooms were usually clean. Would stay here again.'
			}
		],
		'3': [
			{
				id: 1,
				rating: 2,
				date: '2024-09-18',
				semester: 'Fall 2024',
				roomType: 'Single',
				helpful: 5,
				notHelpful: 3,
				ratings: {
					livingConditions: 2,
					location: 3,
					amenities: 2,
					value: 2,
					community: 3
				},
				comment:
					"Not the best experience. Hillenbrand is old and the heating in my room barely worked for the first month. Maintenance was slow and unresponsive. The location is decent but far from some classes. It was pretty quiet, which I appreciated, but I wouldn't recommend unless it's your only option."
			},
			{
				id: 2,
				rating: 3,
				date: '2024-10-02',
				semester: 'Fall 2024',
				roomType: 'Double',
				helpful: 3,
				notHelpful: 2,
				ratings: {
					livingConditions: 3,
					location: 4,
					amenities: 3,
					value: 3,
					community: 3
				},
				comment:
					'An average place to live. The rooms are okay, nothing special. It’s pretty far from the dining courts, which was a bit annoying. However, the staff was friendly and the front desk was helpful. Don’t expect a modern dorm, but it gets the job done.'
			}
		],
		'4': [
			{
				id: 1,
				rating: 5,
				date: '2025-02-22',
				semester: 'Spring 2025',
				roomType: 'Double',
				helpful: 14,
				notHelpful: 0,
				ratings: {
					livingConditions: 5,
					location: 5,
					amenities: 5,
					value: 5,
					community: 5
				},
				comment:
					'Best dorm experience I’ve had! Honors North is super clean, quiet, and feels more like an apartment than a dorm. Private bathrooms are a huge plus. The study lounges and common areas are fantastic. Worth the extra cost if you value comfort and peace of mind.'
			}
		],
		'5': [
			{
				id: 1,
				rating: 3,
				date: '2024-11-01',
				semester: 'Fall 2024',
				roomType: 'Double',
				helpful: 6,
				notHelpful: 1,
				ratings: {
					livingConditions: 3,
					location: 2,
					amenities: 4,
					value: 3,
					community: 3
				},
				comment:
					"Earhart was okay—nothing really stood out. It's kind of far from most lecture halls, so expect a decent walk every day. The rooms were fine, and the dining court was surprisingly good. Not super social, but I liked having my own space. Wouldn't choose it again, but it wasn’t terrible."
			}
		]
	}

	getHousingDetails(id: string) {
		return this.housingDetails[id]
	}

	getReviews(id: string) {
		return this.reviews[id]
	}

	getAllHousing() {
		return Object.values(this.housingDetails)
	}

	sampleHousing(count: number) {
		return Object.values(this.housingDetails).slice(0, count)
	}
}

export const appState = new AppState()
