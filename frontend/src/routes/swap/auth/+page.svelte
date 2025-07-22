<script lang="ts">
	import AuthHeader from '$lib/swap/components/auth/AuthHeader.svelte'
	import { onMount } from 'svelte'
	import { signout } from '$lib/swap/helpers/auth'
	import AuthInput from '$lib/swap/components/auth/AuthInput.svelte'

	onMount(() => {
		signout()
	})
</script>

<svelte:head>
	<title>Login - BoilerSwap</title>
	<meta name="description" content="Sign in, create an account, or change your password." />
</svelte:head>

<AuthHeader />
<AuthInput />
