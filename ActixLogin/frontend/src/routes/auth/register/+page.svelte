<script lang="ts">
    export let data;

    import { post } from '$lib/utils/requests/post.js';
    import { API_URI } from '$lib/utils/constant';
    import type { NewUser } from '$lib/utils/types';
    import type { CustomError } from '$lib/utils/types';
    import { errStore } from '$lib/stores/common.store';
    import Header from '$lib/components/Header.svelte';
    import { isValidEmail } from '$lib/utils/sanitation.js';
    import { isValidPassword } from '$lib/utils/sanitation.js';
    import { goto } from '$app/navigation';

    let email = '';
    let name = '';
    let password = '';
    let retypePassword = '';

    let reqErr: CustomError[] = [];
    let err = {email: '', password: '', retype: ''}

    const unsubscribe = errStore.subscribe(val => {
        reqErr = val;
    });

    async function submitForm() {
        err = {email: '', password: '', retype: ''};
        if(!isValidEmail(email)) {
            err = {email: 'Please input a valid email.', password: '', retype: ''};
            return;
        } else if(!isValidPassword(password)) {
            err = {email: '', password: 'Password must have at least one uppercase character, a number, a symbol, and 8 digits.', retype: ''};
            return;
        } else if (password !== retypePassword) {
            err = {email: '', password: '', retype: 'Passwords do not match.'};
            return;
        }

        // TODO: SEPARATE THE FIRST NAME AND LAST NAME
        const newUser: NewUser = {
            email,
            password,
            first_name: name,
            last_name: ''
        }

        const [res, localReqErr] = await post(data.fetch, `${API_URI}/users/register/`, newUser);

        if (res.ok) {
            console.log('Email sent!');
            goto('/auth/register/email-sent');
        } else {
            console.error('Failed to send verification email');
        }
    }

    import { onDestroy } from 'svelte';
    onDestroy(() => {
        unsubscribe();
    });
</script>

<Header />

<!-- #1 Homie: Ferris -->
<img src="/ferris.png" alt="Ferris" class="fixed h-auto lg:w-[40%] w-[50%] dark:mix-blend-soft-light left-16 -bottom-4 overflow-hidden opacity-45 dark:opacity-85 max-w-full -z-50" />

<section class="flex items-center justify-center my-10">
    <div class="flex items-center justify-center min-w-96 mb-8 z-10 md:w-8/12 lg:w-4/12 border-4 rounded-2xl p-8 border-neutral-200 bg-neutral-200 dark:bg-neutral-800 dark:border-neutral-800">
        <form on:submit={submitForm} class="w-80">
            <!--Sign in section-->
            <div
                class="flex flex-row items-center justify-center mb-4">
                <h1 class="text-neutral-700 dark:text-neutral-200 text-2xl font-semibold">Register</h1>
            </div>

            <!-- Email input -->
            <div class="relative mb-4">
                <input
                    required
                    type="text"
                        class={`peer block min-h-[auto] w-full border-b-2 ${err.email ? "border-red-500" : "border-neutral-400 dark:border-neutral-700"} bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 motion-reduce:transition-none focus:border-blue-500 dark:focus:border-blue-500 dark:text-neutral-200 [&:not([input-active])]:placeholder:opacity-0`}
                        id="EmailInput"
                        placeholder="Email address"
                        autocomplete="new-password"
                        bind:value={email}/>
                <label
                    for="EmailInput"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate mt-[0.37rem] leading-[2.15] text-neutral-500 bg-transparent transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] motion-reduce:transition-none peer-focus:text-blue-500 peer-valid:text-transparent dark:text-neutral-600">
                    Email address
                </label>
            </div>

            <!-- Name input -->
            <div class="relative mb-6">
                <input
                    required
                    type="text"
                    class="peer block min-h-[auto] w-full border-b-2 border-neutral-400 dark:border-neutral-700 bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 motion-reduce:transition-none focus:border-blue-500 dark:focus:border-blue-500 dark:text-neutral-200 [&:not([input-active])]:placeholder:opacity-0"
                    id="NameInput"
                    placeholder="Name"
                    autocomplete="new-password"
                    bind:value={name}/>
                <label
                    for="NameInput"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate mt-[0.37rem] leading-[2.15] text-neutral-500 bg-transparent transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] motion-reduce:transition-none peer-focus:text-blue-500 peer-valid:text-transparent dark:text-neutral-600">
                    Name
                </label>
            </div>

            <!-- Password input -->
            <div class="relative mb-6">
                <input
                    required
                    type="password"
                    class={`peer block min-h-[auto] w-full border-b-2 ${err.password ? "border-red-500" : "border-neutral-400 dark:border-neutral-700"} bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 motion-reduce:transition-none focus:border-blue-500 dark:focus:border-blue-500 dark:text-neutral-200 [&:not([input-active])]:placeholder:opacity-0`}
                    id="PasswordInput"
                    placeholder="Password"
                    autocomplete="new-password"
                    bind:value={password}/>
                <label
                    for="PasswordInput"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate mt-[0.37rem] leading-[2.15] text-neutral-500 bg-transparent transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] motion-reduce:transition-none peer-focus:text-blue-500 peer-valid:text-transparent dark:text-neutral-600">
                    Password
                </label>
            </div>

            <!-- Re-type Password input -->
            <div class="relative mb-6">
                <input
                    required
                    type="password"
                        class={`peer block min-h-[auto] w-full border-b-2 ${err.retype ? "border-red-500" : "border-neutral-400 dark:border-neutral-700"} bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 motion-reduce:transition-none focus:border-blue-500 dark:focus:border-blue-500 dark:text-neutral-200 [&:not([input-active])]:placeholder:opacity-0`}
                        id="RetypePasswordInput"
                        placeholder="Re-type password"
                        autocomplete="new-password"
                        bind:value={retypePassword}/>
                <label
                    for="RetypePasswordInput"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate mt-[0.37rem] leading-[2.15] text-neutral-500 bg-transparent transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] motion-reduce:transition-none peer-focus:text-blue-500 peer-valid:text-transparent dark:text-neutral-600">
                    Re-type password
                </label>
            </div>

            {#each Object.entries(err) as [key, msg]}
                <p class="text-red-500 max-w-72 text-sm mb-4 mx-4 text-wrap">{msg}</p>
            {/each}

            <!-- Register button -->
            <div class="text-center">
                <button
                    type="submit"
                    class="inline-block w-full rounded-2xl bg-blue-500 px-0 pb-2 pt-3 text-sm uppercase leading-normal text-neutral-50 font-bold transition duration-150 ease-in-out shadow-md hover:bg-blue-600 focus:outline-none dark:shadow-black/30">
                    Register
                </button>

                <!-- Login link -->
                <p class="text-neutral-700 dark:text-neutral-200 mb-0 mt-2 pt-1 text-sm font-semibold select-none">
                    Already have an account?
                    <a
                        href="/auth/login"
                        class="text-blue-700 dark:text-blue-500 transition duration-150 ease-in-out hover:text-blue-900 hover:dark:text-blue-700 focus:text-danger-600 active:text-danger-700 pointer-events-auto">
                        Login
                    </a>
                </p>
            </div>
        </form>
    </div>
</section>