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
    <div class="flex items-center justify-center mb-12 md:w-8/12 lg:w-4/12 xl:w-4/12 p-8 lg:px-0 border-4 border-neutral-200 rounded-2xl text-slate-950">
        <form>
            <!--Sign in section-->
            <div
                class="flex flex-row items-center justify-center lg:justify-start">
                <p class= "text-neutral-800 mb-0 me-4 text-lg inline-block align-middle">Sign in with</p>

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
                <p class="text-neutral-800 mx-4 mb-0 text-center font-semibold dark:text-neutral-700">Or</p>
            </div>

            <!-- Email input -->
            <div class="relative mb-6">
                <input
                    required
                    type="text"
                        class="peer block min-h-[auto] w-full border-b-2 border-neutral-200 bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 motion-reduce:transition-none focus:border-blue-500 dark:text-white dark:placeholder:text-neutral-300 dark:autofill:shadow-autofill dark:peer-focus:text-primary [&:not([input-active])]:placeholder:opacity-0"
                        id="EmailInput"
                        placeholder="Email address"/>
                <label
                    for="EmailInput"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate mt-[0.37rem] leading-[2.15] text-neutral-500 bg-transparent transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] motion-reduce:transition-none peer-focus:text-blue-500 peer-valid:text-transparent dark:text-neutral-400 dark:peer-focus:text-primary">
                    Email address
                </label>
            </div>

            <!-- Password input -->
            <div class="relative mb-6">
                <input
                    required
                    type="password"
                        class="peer block min-h-[auto] w-full border-b-2 border-neutral-200 bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 motion-reduce:transition-none focus:border-blue-500 dark:text-white dark:placeholder:text-neutral-300 dark:autofill:shadow-autofill dark:peer-focus:text-primary [&:not([input-active])]:placeholder:opacity-0"
                        id="PasswordInput"
                        placeholder="Password" />
                <label
                    for="PasswordInput"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate mt-[0.37rem] leading-[2.15] text-neutral-500 bg-transparent transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] motion-reduce:transition-none peer-focus:text-blue-500 peer-valid:text-transparent dark:text-neutral-400 dark:peer-focus:text-primary">
                    Password
                </label>
            </div>

            <div class="mb-6 flex items-center justify-between">
                <!-- Remember me -->
                <div class="flex items-center mb-[0.125rem] ml-0 mr-6 min-h-[1.5rem] ps-[1.5rem]">
                    <!-- Checkbox -->
                    <div class="relative">
                        <input type="checkbox" id="check" class="peer h-6 w-6 cursor-pointer appearance-none rounded border-2 border-neutral-200 bg-transparent transition-all duration-300 hover:border-neutral-400 hover:checked:border-neutral-800 checked:bg-neutral-800 checked:border-neutral-800" />
                        <svg class="pointer-events-none absolute top-[40%] left-1/2 w-4 h-4 fill-white stroke-white transform -translate-x-1/2 -translate-y-1/2 scale-0 transition-transform duration-300 peer-checked:scale-100" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z"/></svg>
                    </div>

                    <label
                        class="text-neutral-800 relative text-md mx-1 ps-[0.15rem] bottom-[0.2rem] hover:cursor-pointer"
                        for="Checkbox1">
                        Remember me
                    </label>
                </div>

                <!-- Forgot password link -->
                    <a href="#!" class="text-neutral-800 relative bottom-[0.2rem] text-sm mr-6 hover:text-blue-500">Forgot password?</a>
            </div>
                
            <!-- Login button -->
            <div class="text-center">
                <button
                    type="button"
                    class="inline-block w-full rounded bg-blue-400 px-7 pb-2 pt-3 text-sm uppercase leading-normal text-neutral-50 font-bold shadow-primary-3 transition duration-150 ease-in-out hover:bg-blue-500 hover:shadow-primary-2 focus:bg-primary-accent-300 focus:shadow-primary-2 focus:outline-none focus:ring-0 active:bg-primary-600 active:shadow-primary-2 dark:shadow-black/30 dark:hover:shadow-dark-strong dark:focus:shadow-dark-strong dark:active:shadow-dark-strong">
                    Login
                </button>

                <!-- Register link -->
                <p class="text-neutral-800 mb-0 mt-2 pt-1 text-sm font-semibold">
                    Don't have an account?
                    <a
                        href="#!"
                        class="text-green-700 transition duration-150 ease-in-out hover:text-green-900 focus:text-danger-600 active:text-danger-700">
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