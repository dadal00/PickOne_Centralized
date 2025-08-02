<script lang="ts">
	import NavBar from '$lib/swap/components/layout/NavBar.svelte'
	import Footer from '$lib/swap/components/layout/Footer.svelte'
	import PostButton from '$lib/swap/components/layout/PostButton.svelte'
	import { appState } from '$lib/swap/app-state.svelte.js'
	import { Status } from '$lib/swap/models'
	import { page } from '$app/state'

	let { children, data } = $props()

	appState.setStatus(Status.isSignedIn, data.signedIn)
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
