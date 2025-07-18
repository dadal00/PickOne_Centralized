<script>
	import { onDestroy, onMount } from 'svelte'
	import '../app.css'
	import { browser } from '$app/environment'

	let { children } = $props()

	function setVh() {
		document.documentElement.style.setProperty('--vh', `${window.innerHeight * 0.01}px`)
	}

	onMount(() => {
		setVh()
		window.addEventListener('resize', setVh)
	})

	onDestroy(() => {
		if (browser) {
			window.removeEventListener('resize', setVh)
		}
	})
</script>

<main class="w-screen h-screen">
	{@render children()}
</main>
