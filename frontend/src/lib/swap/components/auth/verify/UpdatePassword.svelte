<script lang="ts">
	import { update } from '$lib/swap/helpers/auth'
	import { PUBLIC_MAX_CHARS, PUBLIC_MIN_PASSWORD_LENGTH } from '$env/static/public'
	import { appState } from '$lib/swap/app-state.svelte'

	let new_password: string = $state('')
</script>

<form onsubmit={() => update(new_password)} class="space-y-4">
	<p class="text-gray-600 text-sm">Enter your new password.</p>
	<div>
		<label class="block text-sm font-medium mb-2">
			New Password
			<input
				type="text"
				bind:value={new_password}
				placeholder="abcdefghijklmnop"
				class="w-full px-4 py-2 border rounded-lg"
				maxlength={Number(PUBLIC_MAX_CHARS)}
				minlength={Number(PUBLIC_MIN_PASSWORD_LENGTH)}
				required
			/>
		</label>
		<p class="text-xs text-gray-500 mt-1">Must be {PUBLIC_MIN_PASSWORD_LENGTH}+ characters</p>
	</div>
	<button
		type="submit"
		class="w-full {appState.getLimited()
			? 'bg-gray-300 cursor-not-allowed'
			: 'bg-yellow-400 hover:bg-yellow-500'} text-gray-800 py-2 rounded-lg transition-colors"
	>
		Submit
	</button>
</form>
