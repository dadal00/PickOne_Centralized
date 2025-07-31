<script lang="ts">
	import { Send } from '@lucide/svelte'
	import CardPiece from '../../templates/CardPiece.svelte'
</script>

<CardPiece cardPiece="cardHeader">
	<CardPiece className="flex items-center space-x-2 dark:text-gray-100" cardPiece="cardTitle">
		<Send class="h-5 w-5" />
		<span>Submit Your Review</span>
	</CardPiece>
</CardPiece>
