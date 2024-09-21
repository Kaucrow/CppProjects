<script lang="ts">
    export let data;

    import { post } from '$lib/utils/requests/post.requests';
    import { API_URI } from '$lib/utils/constant';
    import type { LoginUser } from '$lib/utils/types';
    import type { CustomError } from '$lib/utils/types';
    import { errStore } from '$lib/stores/common.store';
    import LogoutButton from '$lib/components/LogoutButton.svelte';

    let email = '';
    let password = '';
    let err: CustomError[] = [];

    const unsubscribe = errStore.subscribe(val => {
        err = val;
    });

    async function submitForm() {
        const loginUser: LoginUser = {
            email,
            password
        }

        const [res, err] = await post(data.fetch, `${API_URI}/users/login/`, loginUser);

        if (err.length > 0) {
            console.error('Failed to log in');
        } else {
            console.log('Logged in successfully');
        }
    }

    import { onDestroy } from 'svelte';
    onDestroy(() => {
        unsubscribe();
    });
</script>

<form on:submit|preventDefault={submitForm}>
    <div class="p-2">
        <label for="email">Email:</label>
        <input type="email" id="email" bind:value={email} required>
    </div>

    <div class="p-2">
        <label for="password">Password:</label>
        <input type="password" id="password" bind:value={password} required>
    </div>

    <button class="p-2" type="submit">Log in</button>
</form>

<LogoutButton {data} />

<div>
    {#if err && err.length > 0}
        <ul>
            {#each err as err}
                <li>{err.error}</li>
            {/each}
        </ul>
    {/if}
</div>