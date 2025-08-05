<script lang="ts">
	import SearchSelect from '../../templates/SearchSelect.svelte'
	import { appState } from '$lib/housing/app-state.svelte'
	import { reviewFilterRatingSelect, reviewSortSelect } from '$lib/housing/models/reviews'
</script>

<SearchSelect
	selectOptions={reviewFilterRatingSelect}
	getAction={appState.getRatingFilter.bind(appState)}
	setAction={appState.setRatingFilter.bind(appState)}
/>
<SearchSelect
	selectOptions={reviewSortSelect}
	getAction={appState.getReviewSortDirection.bind(appState)}
	setAction={appState.setReviewSortDirection.bind(appState)}
/>
