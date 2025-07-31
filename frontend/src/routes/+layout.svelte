<script lang="ts">
	import { onDestroy, onMount } from 'svelte'
	import '../app.css'
	import { browser } from '$app/environment'
	import { refreshToken } from '$lib/utils'
	import { SvelteToast } from '@zerodevx/svelte-toast'

	let { children } = $props()
	let interval: number

	function setVh() {
		document.documentElement.style.setProperty('--vh', `${window.innerHeight * 0.01}px`)
	}

	onMount(() => {
		setVh()
		window.addEventListener('resize', setVh)

		refreshToken()

		interval = setInterval(refreshToken, 270_000)
	})

	onDestroy(() => {
		if (browser) {
			window.removeEventListener('resize', setVh)
		}

		clearInterval(interval)
	})
</script>

<SvelteToast />

<main class="w-screen h-screen">
	{@render children()}
</main>
