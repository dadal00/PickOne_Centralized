<script lang="ts">
	import { goto } from '$app/navigation'
	import { PUBLIC_TEMP_SESSION_DURATION_SECS, PUBLIC_SVELTE_SWAP_ROOT } from '$env/static/public'
	import { appState } from '$lib/swap/app-state.svelte'
	import UpdatePassword from '$lib/swap/components/auth/verify/UpdatePassword.svelte'
	import { Status } from '$lib/swap/models'
	import { onDestroy, onMount } from 'svelte'

	let timer: number | null = null

	onMount(() => {
		if (!appState.getStatus(Status.isVerifyingUpdate)) {
			goto(PUBLIC_SVELTE_SWAP_ROOT + '/browse')
		}

		timer = setTimeout(
			() => {
				appState.setStatus(Status.isVerifyingUpdate, false)
			},
			Number(PUBLIC_TEMP_SESSION_DURATION_SECS) * 1000
		)
	})

	$effect(() => {
		if (!appState.getStatus(Status.isVerifyingUpdate)) {
			goto(PUBLIC_SVELTE_SWAP_ROOT + '/browse')
		}
	})

	onDestroy(() => {
		appState.setStatus(Status.isVerifyingUpdate, false)
		clearTimeout(timer!)
	})
</script>

<div class="container mx-auto px-6 py-16 max-w-md">
	<div class="bg-white rounded-lg shadow-sm border p-6">
		<UpdatePassword />
	</div>
</div>
