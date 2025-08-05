<script lang="ts">
	import SearchSelect from '../templates/SearchSelect.svelte'
	import {
		housingFilterCampusSelect,
		housingFilterCostSelect,
		housingFilterTypeSelect,
		housingSortDirectionSelect,
		housingSortSelect
	} from '$lib/housing/models/housing'
	import { appState } from '$lib/housing/app-state.svelte'
</script>

<SearchSelect
	selectOptions={housingFilterTypeSelect}
	getAction={appState.getHousingTypeFilter.bind(appState)}
	setAction={appState.setHousingTypeFilter.bind(appState)}
/>
<SearchSelect
	selectOptions={housingFilterCampusSelect}
	getAction={appState.getCampusTypeFilter.bind(appState)}
	setAction={appState.setCampusTypeFilter.bind(appState)}
/>
<SearchSelect
	selectOptions={housingFilterCostSelect}
	getAction={appState.getCostSymbolFilter.bind(appState)}
	setAction={appState.setCostSymbolFilter.bind(appState)}
/>
<SearchSelect
	selectOptions={housingSortSelect}
	getAction={appState.getHousingSortCategory.bind(appState)}
	setAction={appState.setHousingSortCategory.bind(appState)}
/>
<SearchSelect
	selectOptions={housingSortDirectionSelect}
	getAction={appState.getHousingSortDirection.bind(appState)}
	setAction={appState.setHousingSortDirection.bind(appState)}
/>
