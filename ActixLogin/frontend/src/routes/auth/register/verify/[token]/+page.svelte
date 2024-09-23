<script lang="ts">
    export let data;

    import { get } from '$lib/utils/requests/get';
    import { API_URI } from '$lib/utils/constant';
    import type { CustomError } from '$lib/utils/types.js';
    import Header from '$lib/components/Header.svelte';
    import { onMount } from 'svelte';
    
    let verified = true;
    let err: CustomError[] = [];

    onMount(async () => {
        const [res, reqErr] = await get(data.fetch, `${API_URI}/users/register/verify?token=${data.slug}`);
        if (res.ok) {
            verified = true;
        } else {
            err = reqErr;
        }
    })
</script>

<Header />

{#if verified}
    <div class="flex items-center justify-center mt-20 mx-10 lg:mt-20">
        <p
            class="text-neutral-800 dark:text-neutral-200 text-2xl">
            Your account has been verified! You can now
            <a href="/auth/login" class="text-blue-500 hover:text-blue-600">
                log in</a>.
        </p>
    </div>
{:else}
    <div class="flex items-center justify-center mt-20 mx-10 lg:mt-20">
        <p class="text-neutral-800 dark:text-neutral-200 text-2xl">
            Verifying your account...
        </p>
    </div>
{/if}


{#if err.length > 0}
    <div class="flex-col items-center justify-center text-center mt-5 mx-10 lg:mx-20">
        <p class="text-red-500 text-xl">An error has been produced during your verification: </p>
        {#each err as msg}
            <p class="text-red-500 text-xl">{msg.error}</p>
        {/each}
    </div>
{/if}