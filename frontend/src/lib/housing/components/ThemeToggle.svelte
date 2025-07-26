<script lang="ts">
	import { Sun, Moon } from '@lucide/svelte'
	import Button from '$lib/housing/components/templates/Button.svelte'

	let darkMode = $state(false)

	function toggleTheme() {
		darkMode = !darkMode
		document.documentElement.classList.toggle('dark')
	}
</script>

<Button
	variant="ghost"
	size="icon"
	action={toggleTheme}
	ariaLabel="Toggle theme"
	className="relative"
>
	<Sun
		class={[
			'h-5 w-5 transition-all duration-300',
			darkMode ? '-rotate-90 scale-0' : 'rotate-0 scale-100'
		]}
	/>
	<Moon
		class={[
			'absolute h-5 w-5 transition-all duration-300',
			darkMode ? 'rotate-0 scale-100' : 'rotate-90 scale-0'
		]}
	/>
	<span class="sr-only">Toggle theme</span>
</Button>
