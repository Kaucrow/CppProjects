<script lang="ts">
    export let data;

    import Header from "$lib/components/Header.svelte";
    import LogoutButton from "$lib/components/LogoutButton.svelte";
    import { CLT_MOD } from "$lib/utils/constant.js";
</script>

<Header />

<section class="flex items-center justify-center my-12">
    <div class="mt-8 flex flex-col items-center justify-center">
        <!-- Checkmark icon -->
        <svg class="fill-neutral-800 dark:fill-neutral-200" xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 448 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
            <path d="M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z"/>
        </svg>
        <!-- Box -->
        <div class="flex flex-col items-center justify-center my-8 max-w-[500px] w-full z-10 p-8 rounded-2xl bg-neutral-200 border-neutral-200 dark:bg-neutral-800">
            <p class="text-neutral-800 dark:text-neutral-200 mx-8 text-2xl text-center font-semibold">
                You are logged in! 
            </p>
            <p class="text-neutral-600 dark:text-neutral-400 mt-4 mx-2 text-center">
                Yay!
            </p>
            <!-- Cirno -->
            <img src={`${CLT_MOD}/cirno.gif`} alt="Cirno spin" class="w-20 h-auto mt-4"/>
        </div>
        <LogoutButton {data} />
    </div>
</section>