<script lang="ts">
	import { PUBLIC_SVELTE_SWAP_ROOT } from '$env/static/public'
</script>

<a
	href={PUBLIC_SVELTE_SWAP_ROOT + '/post'}
	class="fixed bottom-6 right-6 w-14 h-14 rounded-full bg-gradient-to-r from-yellow-400 to-amber-500 hover:from-yellow-500 hover:to-amber-600 text-white shadow-lg transition-all hover:scale-110 flex items-center justify-center"
>
	<span class="text-2xl">+</span>
</a>
