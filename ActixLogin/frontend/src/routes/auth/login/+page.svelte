<script lang="ts">
    export let data;

    import { post } from '$lib/utils/requests/post.requests';
    import { API_URI } from '$lib/utils/constant';
    import type { LoginUser } from '$lib/utils/types';
    import type { CustomError } from '$lib/utils/types';
    import { errStore } from '$lib/stores/common.store';

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

    import { onMount } from 'svelte';

    let darkMode = false;

    onMount(() => {
        const savedTheme = localStorage.getItem('theme');
        if (savedTheme) {
            darkMode = savedTheme === 'dark';
            updateTheme();
        } else {
            darkMode = window.matchMedia('(prefers-color-scheme: dark)').matches;
            updateTheme();
        }
    });

    // Toggles dark mode
    function toggleDarkMode() {
        darkMode = !darkMode;
        updateTheme();
    }

    // Update theme by adding/removing the "dark" class and saving the preference
    function updateTheme() {
        const html = document.querySelector('html');
        if (darkMode) {
            html?.classList.add('dark');
            localStorage.setItem('theme', 'dark');
        } else {
            html?.classList.remove('dark');
            localStorage.setItem('theme', 'light');
        }
    }
</script>

<section class="flex items-center justify-center h-screen">
    <div class="flex items-center justify-center mb-12 md:mb-0 md:w-8/12 lg:w-4/12 xl:w-4/12 p-8 lg:py-8 lg:px-0 border-4 rounded-2xl text-slate-950">
        <form>
            <!--Sign in section-->
            <div
                class="flex flex-row items-center justify-center lg:justify-start">
                <p class="mb-0 me-4 text-lg inline-block align-middle">Sign in with</p>

                <!-- Google -->
                <button
                    type="button"
                    class="mx-0 inline-block rounded-full bg-primary p-1">
                    <span>
                        <svg width="30px" height="30px" viewBox="0 0 45 45" xmlns="http://www.w3.org/2000/svg" class="inline-block align-middle">
                            <path fill="#FFC107" d="M43.611,20.083H42V20H24v8h11.303c-1.649,4.657-6.08,8-11.303,8c-6.627,0-12-5.373-12-12c0-6.627,5.373-12,12-12c3.059,0,5.842,1.154,7.961,3.039l5.657-5.657C34.046,6.053,29.268,4,24,4C12.955,4,4,12.955,4,24c0,11.045,8.955,20,20,20c11.045,0,20-8.955,20-20C44,22.659,43.862,21.35,43.611,20.083z"></path><path fill="#FF3D00" d="M6.306,14.691l6.571,4.819C14.655,15.108,18.961,12,24,12c3.059,0,5.842,1.154,7.961,3.039l5.657-5.657C34.046,6.053,29.268,4,24,4C16.318,4,9.656,8.337,6.306,14.691z"></path><path fill="#4CAF50" d="M24,44c5.166,0,9.86-1.977,13.409-5.192l-6.19-5.238C29.211,35.091,26.715,36,24,36c-5.202,0-9.619-3.317-11.283-7.946l-6.522,5.025C9.505,39.556,16.227,44,24,44z"></path><path fill="#1976D2" d="M43.611,20.083H42V20H24v8h11.303c-0.792,2.237-2.231,4.166-4.087,5.571c0.001-0.001,0.002-0.001,0.003-0.002l6.19,5.238C36.971,39.205,44,34,44,24C44,22.659,43.862,21.35,43.611,20.083z"></path>
                        </svg>
                    </span>
                </button>
            </div>

            <!-- Separator between social media sign in and email/password sign in -->
            <div
                class="my-4 flex items-center before:mt-0.5 before:flex-1 before:border-t before:border-neutral-300 after:mt-0.5 after:flex-1 after:border-t after:border-neutral-300 dark:before:border-neutral-500 dark:after:border-neutral-500">
                <p
                    class="mx-4 mb-0 text-center font-semibold dark:text-neutral-700">
                    Or
                </p>
            </div>

            <!-- Email input -->
            <div class="relative mb-6" data-twe-input-wrapper-init>
                <input
                    type="text"
                        class="peer block min-h-[auto] w-full rounded border-0 bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 peer-focus:text-primary data-[twe-input-state-active]:placeholder:opacity-100 motion-reduce:transition-none dark:text-white dark:placeholder:text-neutral-300 dark:autofill:shadow-autofill dark:peer-focus:text-primary [&:not([data-twe-input-placeholder-active])]:placeholder:opacity-0"
                        id="exampleFormControlInput2"
                        placeholder="Email address" />
                <label
                    for="exampleFormControlInput2"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate pt-[0.37rem] leading-[2.15] text-neutral-500 transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] motion-reduce:transition-none dark:text-neutral-400 dark:peer-focus:text-primary">
                    Email address
                </label>
            </div>

            <!-- Password input -->
            <div class="relative mb-6" data-twe-input-wrapper-init>
                <input
                    type="password"
                    class="peer block min-h-[auto] w-full rounded border-0 bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 peer-focus:text-primary data-[twe-input-state-active]:placeholder:opacity-100 motion-reduce:transition-none dark:text-white dark:placeholder:text-neutral-300 dark:autofill:shadow-autofill dark:peer-focus:text-primary [&:not([data-twe-input-placeholder-active])]:placeholder:opacity-0"
                    id="exampleFormControlInput22"
                    placeholder="Password" />
                <label
                    for="exampleFormControlInput22"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate pt-[0.37rem] leading-[2.15] text-neutral-500 transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] peer-focus:text-primary peer-data-[twe-input-state-active]:-translate-y-[1.15rem] peer-data-[twe-input-state-active]:scale-[0.8] motion-reduce:transition-none dark:text-neutral-400 dark:peer-focus:text-primary"
                    >Password
                </label>
            </div>

            <div class="mb-6 flex items-center justify-between">
                <!-- Remember me checkbox -->
                <div class="mb-[0.125rem] block min-h-[1.5rem] ps-[1.5rem]">
                    <input
                        class="relative float-left -ms-[1.5rem] me-[6px] mt-[0.15rem] h-[1.125rem] w-[1.125rem] appearance-none rounded-[0.25rem] border-[0.125rem] border-solid border-secondary-500 outline-none before:pointer-events-none before:absolute before:h-[0.875rem] before:w-[0.875rem] before:scale-0 before:rounded-full before:bg-transparent before:opacity-0 before:shadow-checkbox before:shadow-transparent before:content-[''] checked:border-primary checked:bg-primary checked:before:opacity-[0.16] checked:after:absolute checked:after:-mt-px checked:after:ms-[0.25rem] checked:after:block checked:after:h-[0.8125rem] checked:after:w-[0.375rem] checked:after:rotate-45 checked:after:border-[0.125rem] checked:after:border-l-0 checked:after:border-t-0 checked:after:border-solid checked:after:border-white checked:after:bg-transparent checked:after:content-[''] hover:cursor-pointer hover:before:opacity-[0.04] hover:before:shadow-black/60 focus:shadow-none focus:transition-[border-color_0.2s] focus:before:scale-100 focus:before:opacity-[0.12] focus:before:shadow-black/60 focus:before:transition-[box-shadow_0.2s,transform_0.2s] focus:after:absolute focus:after:z-[1] focus:after:block focus:after:h-[0.875rem] focus:after:w-[0.875rem] focus:after:rounded-[0.125rem] focus:after:content-[''] checked:focus:before:scale-100 checked:focus:before:shadow-checkbox checked:focus:before:transition-[box-shadow_0.2s,transform_0.2s] checked:focus:after:-mt-px checked:focus:after:ms-[0.25rem] checked:focus:after:h-[0.8125rem] checked:focus:after:w-[0.375rem] checked:focus:after:rotate-45 checked:focus:after:rounded-none checked:focus:after:border-[0.125rem] checked:focus:after:border-l-0 checked:focus:after:border-t-0 checked:focus:after:border-solid checked:focus:after:border-white checked:focus:after:bg-transparent rtl:float-right dark:border-neutral-400 dark:checked:border-primary dark:checked:bg-primary"
                        type="checkbox"
                        value=""
                        id="exampleCheck2" />
                    <label
                        class="inline-block ps-[0.15rem] hover:cursor-pointer"
                        for="exampleCheck2">
                        Remember me
                    </label>
                </div>

                <!-- Forgot password link -->
                    <a href="#!">Forgot password?</a>
            </div>
                
            <!-- Login button -->
            <div class="text-center lg:text-left">
                <button
                    type="button"
                    class="inline-block w-full rounded bg-primary px-7 pb-2 pt-3 text-sm font-medium uppercase leading-normal text-white shadow-primary-3 transition duration-150 ease-in-out hover:bg-primary-accent-300 hover:shadow-primary-2 focus:bg-primary-accent-300 focus:shadow-primary-2 focus:outline-none focus:ring-0 active:bg-primary-600 active:shadow-primary-2 dark:shadow-black/30 dark:hover:shadow-dark-strong dark:focus:shadow-dark-strong dark:active:shadow-dark-strong"
                    data-twe-ripple-init
                    data-twe-ripple-color="light">
                    Login
                </button>

                <!-- Register link -->
                <p class="mb-0 mt-2 pt-1 text-sm font-semibold">
                Don't have an account?
                    <a
                        href="#!"
                        class="text-danger transition duration-150 ease-in-out hover:text-danger-600 focus:text-danger-600 active:text-danger-700">
                        Register
                    </a>
                </p>
            </div>
        </form>
    </div>
</section>

<button on:click={toggleDarkMode} class="p-2 bg-neutral-500 dark:bg-neutral-300s rounded">
    {#if darkMode}
        Switch to Light Mode
    {:else}
        Switch to Dark Mode
    {/if}
</button>
<!--
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
-->