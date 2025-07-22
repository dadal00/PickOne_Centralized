<script lang="ts">
	import { PUBLIC_SVELTE_SWAP_ROOT } from '$env/static/public'
	import Items from '../browse/Items.svelte'
</script>

<section class="py-16 px-6">
	<div class="container mx-auto">
		<h2 class="text-2xl font-bold text-center mb-8">Available Items</h2>
		<Items />

		<div class="text-center mt-8">
			<a
				href={PUBLIC_SVELTE_SWAP_ROOT + '/browse'}
				class="bg-gradient-to-r from-yellow-400 to-amber-500 hover:from-yellow-500 hover:to-amber-600 text-white px-8 py-3 rounded-lg text-lg font-medium"
			>
				View All Items
			</a>
		</div>
	</div>
</section>
