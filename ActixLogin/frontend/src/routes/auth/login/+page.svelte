<script lang="ts">
    export let data;

    import { post } from '$lib/utils/requests/post.js';
    import { API_URI } from '$lib/utils/constant';
    import type { LoginUser } from '$lib/utils/types';
    import type { CustomError } from '$lib/utils/types';
    import { errStore } from '$lib/stores/common.store';
    import Header from '$lib/components/Header.svelte';
    import { isValidEmail } from '$lib/utils/sanitation.js';
    import { goto } from '$app/navigation';

    let email = '';
    let password = '';
    let reqErr: CustomError[] = [];
    let err = {email: '', notFound: ''};

    const unsubscribe = errStore.subscribe(val => {
        reqErr = val;
    });

    async function submitForm() {
        err = {email: '', notFound: ''};
        if (!isValidEmail(email)) {
            err.email = 'Please input a valid email.';
            return;
        }

        const loginUser: LoginUser = {
            email,
            password
        }

        const [res, localReqErr] = await post(data.fetch, `${API_URI}/users/login`, loginUser);

        if (res.ok) {
            console.log('Logged in successfully');
            goto('/user');
        } else {
            err.notFound = 'No user with these credentials was found. If you registered with these credentials, ensure you verified your account by clicking on the link sent to your email.';
        }
    }

    import { onDestroy } from 'svelte';
    onDestroy(() => {
        unsubscribe();
    })
</script>

<Header />

<!-- #1 Homie: Ferris -->
<img src={'/ferris.png'} alt="Ferris" class="fixed h-auto lg:w-[40%] w-[50%] dark:mix-blend-soft-light left-16 -bottom-4 overflow-hidden opacity-45 dark:opacity-85 max-w-full -z-50" />

<section class="flex items-center justify-center my-12">
    <div class="flex items-center justify-center mb-8 z-10 md:w-8/12 lg:w-4/12 p-8 lg:px-0 border-4 rounded-2xl bg-neutral-200 border-neutral-200 dark:bg-neutral-800 dark:border-neutral-800">
        <form on:submit|preventDefault={submitForm} class="max-w-sm">
            <!--Sign in section-->
            <div
                class="flex flex-row items-center justify-center lg:justify-start lg:ml-4">
                <p class= "text-neutral-700 dark:text-neutral-200 mb-0 me-4 text-lg inline-block align-middle select-none">Sign in with</p>

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
                class="my-4 flex items-center before:mt-0.5 before:flex-1 before:border-t before:border-neutral-400 after:mt-0.5 after:flex-1 after:border-t after:border-neutral-400 dark:before:border-neutral-700 dark:after:border-neutral-700">
                <p class="text-neutral-700 dark:text-neutral-200 mx-4 mb-0 text-center font-semibold select-none">Or</p>
            </div>

            <!-- Email input -->
            <div class="relative mb-6">
                <input
                    required
                    type="text"
                        class={`peer block min-h-[auto] w-full border-b-2 ${err.email ? "border-red-500" : "border-neutral-400 dark:border-neutral-700"} bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 motion-reduce:transition-none focus:border-blue-500 dark:focus:border-blue-500 dark:text-neutral-200 [&:not([input-active])]:placeholder:opacity-0`}
                        id="EmailInput"
                        placeholder="Email address"
                        autocomplete="off"
                        bind:value={email}/>
                <label
                    for="EmailInput"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate mt-[0.37rem] leading-[2.15] text-neutral-500 bg-transparent transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] motion-reduce:transition-none peer-focus:text-blue-500 peer-valid:text-transparent dark:text-neutral-600">
                    Email address
                </label>
            </div>

            <!-- Password input -->
            <div class="relative mb-6">
                <input
                    required
                    type="password"
                        class="peer block min-h-[auto] w-full border-b-2 border-neutral-400 dark:border-neutral-700 bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 motion-reduce:transition-none focus:border-blue-500 dark:focus:border-blue-500 dark:text-neutral-200 [&:not([input-active])]:placeholder:opacity-0"
                        id="PasswordInput"
                        placeholder="Password"
                        bind:value={password}/>
                <label
                    for="PasswordInput"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate mt-[0.37rem] leading-[2.15] text-neutral-500 bg-transparent transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] motion-reduce:transition-none peer-focus:text-blue-500 peer-valid:text-transparent dark:text-neutral-600">
                    Password
                </label>
            </div>

            {#if err.email}
                <p class="text-red-500 max-w-72 text-sm mb-4 mx-4 text-wrap">{err.email}</p>
            {:else if err.notFound}
                <p class="text-red-500 max-w-72 text-sm mb-4 mx-4 text-wrap">{err.notFound}</p>
            {/if}

            <div class="mb-6 flex items-center justify-between">
                <!-- Remember me -->
                <div class="flex items-center mb-[0.125rem] ml-0 mr-6 min-h-[1.5rem] ps-[1.5rem]">
                    <!-- Checkbox -->
                    <div class="relative">
                        <input type="checkbox" id="Checkbox" class="peer h-6 w-6 cursor-pointer appearance-none rounded border-2 border-neutral-500 dark:border-neutral-700 bg-transparent transition-all duration-300 checked:dark:bg-neutral-700 hover:border-neutral-600 hover:dark:border-neutral-400  hover:checked:dark:border-neutral-700 hover:checked:border-neutral-700 checked:bg-neutral-700 checked:border-neutral-700 checked:dark:border-neutral-700" />
                        <svg class="pointer-events-none absolute top-[40%] left-1/2 w-4 h-4 fill-white stroke-white transform -translate-x-1/2 -translate-y-1/2 scale-0 transition-transform duration-300 peer-checked:scale-100" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M438.6 105.4c12.5 12.5 12.5 32.8 0 45.3l-256 256c-12.5 12.5-32.8 12.5-45.3 0l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0L160 338.7 393.4 105.4c12.5-12.5 32.8-12.5 45.3 0z"/></svg>
                    </div>

                    <label
                        class="text-neutral-700 dark:text-neutral-200 relative text-md mx-1 ps-[0.15rem] bottom-[0.2rem] hover:cursor-pointer select-none"
                        for="Checkbox">
                        Remember me
                    </label>
                </div>

                <!-- Forgot password link -->
                    <a href="#!" class="text-neutral-700 dark:text-neutral-200 relative bottom-[0.2rem] text-sm mr-6 hover:text-blue-500 hover:dark:text-blue-500">Forgot password?</a>
            </div>
                
            <!-- Login button -->
            <div class="text-center">
                <button
                    type="submit"
                    class="inline-block w-full rounded-2xl bg-blue-500 px-7 pb-2 pt-3 text-sm uppercase leading-normal text-neutral-200 font-bold transition duration-150 ease-in-out shadow-md hover:bg-blue-600 focus:outline-none dark:shadow-black/30">
                    Login
                </button>

                <!-- Register link -->
                <p class="text-neutral-700 dark:text-neutral-200 mb-0 mt-2 pt-4 text-sm font-semibold select-none">
                    New user?
                    <a
                        href="/auth/register"
                        class="text-blue-700 dark:text-blue-500 transition duration-150 ease-in-out hover:text-blue-800 hover:dark:text-blue-600 focus:text-danger-600 active:text-danger-700 pointer-events-auto">
                        Register
                    </a>
                </p>
            </div>
        </form>
    </div>
</section>
