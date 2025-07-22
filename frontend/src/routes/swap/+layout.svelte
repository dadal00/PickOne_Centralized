<script lang="ts">
	import NavBar from '$lib/swap/components/layout/NavBar.svelte'
	import Footer from '$lib/swap/components/layout/Footer.svelte'
	import PostButton from '$lib/swap/components/layout/PostButton.svelte'
	import { appState } from '$lib/swap/AppState.svelte'
	import { Status } from '$lib/swap/models'
	import { onDestroy, onMount } from 'svelte'
	import { page } from '$app/state'

	let { children, data } = $props()
	let interval: ReturnType<typeof setInterval>

	appState.setStatus(Status.isSignedIn, data.signedIn)

	onMount(() => {
		refreshToken()

		interval = setInterval(refreshToken, 270_000)
	})

	onDestroy(() => {
		clearInterval(interval)
	})

	async function refreshToken() {
		try {
			await fetch('/', {
				method: 'HEAD',
				headers: {
					'x-refresh': 'true'
				}
			})
		} catch (e) {}
	}
</script>

{#if !page.url.pathname.includes('/auth')}
	<NavBar />
{/if}

<main>
	{@render children()}
</main>

{#if !page.url.pathname.includes('/auth')}
	<Footer />
{/if}

{#if !page.url.pathname.includes('/auth') && !page.url.pathname.includes('/post')}
	<PostButton />
{/if}
