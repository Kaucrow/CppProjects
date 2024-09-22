<script lang="ts">
    export let data;

    import { post } from '$lib/utils/requests/post.requests';
    import { API_URI } from '$lib/utils/constant';
    import type { LoginUser } from '$lib/utils/types';
    import type { CustomError } from '$lib/utils/types';
    import { errStore } from '$lib/stores/common.store';
    import Header from '$lib/components/Header.svelte';

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

<Header />

<section class="flex items-center justify-center my-10">
    <div class="flex items-center justify-center min-w-96 mb-8 md:w-8/12 lg:w-4/12 border-4 rounded-2xl p-8 border-neutral-300 dark:border-neutral-800">
        <form class="w-80">
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
                        class="peer block min-h-[auto] w-full border-b-2 border-neutral-300 dark:border-neutral-800 bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 motion-reduce:transition-none focus:border-blue-500 dark:focus:border-blue-500 dark:text-neutral-50 [&:not([input-active])]:placeholder:opacity-0"
                        id="EmailInput"
                        placeholder="Email address"/>
                <label
                    for="EmailInput"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate mt-[0.37rem] leading-[2.15] text-neutral-500 bg-transparent transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] motion-reduce:transition-none peer-focus:text-blue-500 peer-valid:text-transparent dark:text-neutral-500">
                    Email address
                </label>
            </div>

            <!-- Email input -->
            <div class="relative mb-6">
                <input
                    required
                    type="text"
                        class="peer block min-h-[auto] w-full border-b-2 border-neutral-300 dark:border-neutral-800 bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 motion-reduce:transition-none focus:border-blue-500 dark:focus:border-blue-500 dark:text-neutral-50 [&:not([input-active])]:placeholder:opacity-0"
                        id="EmailInput"
                        placeholder="Email address"/>
                <label
                    for="EmailInput"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate mt-[0.37rem] leading-[2.15] text-neutral-500 bg-transparent transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] motion-reduce:transition-none peer-focus:text-blue-500 peer-valid:text-transparent dark:text-neutral-500">
                    Name
                </label>
            </div>

            <!-- Password input -->
            <div class="relative mb-6">
                <input
                    required
                    type="text"
                        class="peer block min-h-[auto] w-full border-b-2 border-neutral-300 dark:border-neutral-800 bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 motion-reduce:transition-none focus:border-blue-500 dark:focus:border-blue-500 dark:text-neutral-50 [&:not([input-active])]:placeholder:opacity-0"
                        id="PasswordInput"
                        placeholder="Password"/>
                <label
                    for="PasswordInput"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate mt-[0.37rem] leading-[2.15] text-neutral-500 bg-transparent transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] motion-reduce:transition-none peer-focus:text-blue-500 peer-valid:text-transparent dark:text-neutral-500">
                    Password
                </label>
            </div>

            <!-- Re-type Password input -->
            <div class="relative mb-6">
                <input
                    required
                    type="text"
                        class="peer block min-h-[auto] w-full border-b-2 border-neutral-300 dark:border-neutral-800 bg-transparent px-3 py-[0.32rem] leading-[2.15] outline-none transition-all duration-200 ease-linear focus:placeholder:opacity-100 motion-reduce:transition-none focus:border-blue-500 dark:focus:border-blue-500 dark:text-neutral-50 [&:not([input-active])]:placeholder:opacity-0"
                        id="PasswordInput"
                        placeholder="Password"/>
                <label
                    for="PasswordInput"
                    class="pointer-events-none absolute left-3 top-0 mb-0 max-w-[90%] origin-[0_0] truncate mt-[0.37rem] leading-[2.15] text-neutral-500 bg-transparent transition-all duration-200 ease-out peer-focus:-translate-y-[1.15rem] peer-focus:scale-[0.8] motion-reduce:transition-none peer-focus:text-blue-500 peer-valid:text-transparent dark:text-neutral-500">
                    Re-type password
                </label>
            </div>

            <!-- Login button -->
            <div class="text-center">
                <button
                    type="button"
                    class="inline-block w-full rounded bg-blue-400 px-0 pb-2 pt-3 text-sm uppercase leading-normal text-neutral-50 font-bold transition duration-150 ease-in-out shadow-lg hover:bg-blue-500 focus:outline-none dark:shadow-black/30">
                    Register
                </button>

                <!-- Register link -->
                <p class="text-neutral-700 dark:text-neutral-200 mb-0 mt-2 pt-1 text-sm font-semibold select-none">
                    Already have an account?
                    <a
                        href="/auth/login"
                        class="text-green-700 dark:text-green-500 transition duration-150 ease-in-out hover:text-green-900 hover:dark:text-green-700 focus:text-danger-600 active:text-danger-700 pointer-events-auto">
                        Login
                    </a>
                </p>
            </div>
        </form>
    </div>
</section>