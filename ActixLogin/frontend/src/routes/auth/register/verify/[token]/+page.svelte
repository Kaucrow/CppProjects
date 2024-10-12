<script lang="ts">
    export let data;

    import { get } from '$lib/utils/requests/get';
    import { API_URI } from '$lib/utils/constant';
    import type { CustomError } from '$lib/utils/types.js';
    import Header from '$lib/components/Header.svelte';
    import { onMount } from 'svelte';
    
    let verified = false;
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

<section class="flex flex-col items-center justify-center mt-36">
    {#if verified}
        <!-- SVG Spinner -->
        <svg class="mb-8 fill-neutral-700 dark:fill-neutral-200" xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 448 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
            <path d="M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z"/>
        </svg>
    {:else}
        <img src="/spinner-ring.svg" alt="Spinner" class="mb-8"/>
    {/if}

    <!-- Verifying Message -->
    <div class="text-center">
        <h1 class="text-2xl font-bold text-neutral-800 dark:text-neutral-200 mb-4">
            {#if verified}
                Your account has been verified!
            {:else}
                Verifying your account...
            {/if}
        </h1>
        <p class="text-lg text-neutral-600 dark:text-neutral-400">
            {#if verified}
                You can now <a href={'/auth/login'} class="text-blue-500 hover:text-blue-600">log in</a>.
            {:else}
                Please wait while we verify your details.
            {/if}
        </p>
    </div>
</section>

{#if err.length > 0}
    <div class="flex-col items-center justify-center text-center mt-5 mx-10 lg:mx-20">
        <p class="text-red-500 text-xl">An error has been produced during your verification: </p>
        {#each err as msg}
            <p class="text-red-500 text-xl">{msg.error}</p>
        {/each}
    </div>
{/if}
