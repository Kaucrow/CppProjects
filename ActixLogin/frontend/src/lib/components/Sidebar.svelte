<script lang="ts">
    export let isOpen: boolean;
    export let close: () => void;
</script>

<div class={`fixed z-50 top-0 left-0 h-full w-64 border-r-[6px] border-neutral-300 dark:border-neutral-700 bg-neutral-200 dark:bg-neutral-800 transform ${isOpen ? 'translate-x-0' : '-translate-x-full'} transition-transform duration-300 ease-in-out`}>
    <div class="p-4">
        <div class="flex flex-row items-center justify-between">
            <a href={'/'} class="inline-flex items-center">
                <!-- Rust Icon -->
                <svg class="fill-neutral-700 dark:fill-neutral-200 mr-2" xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 512 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.-->
                    <path d="M508.5 249.8 486.7 236.2c-.2-2-.3-3.9-.6-5.9l18.7-17.5a7.4 7.4 0 0 0 -2.4-12.3l-24-9c-.5-1.9-1.1-3.8-1.7-5.6l15-20.8a7.4 7.4 0 0 0 -4.8-11.5l-25.4-4.2c-.9-1.7-1.8-3.5-2.7-5.2l10.7-23.4a7.4 7.4 0 0 0 -7-10.4l-25.8 .9q-1.8-2.2-3.6-4.4L439 81.8A7.4 7.4 0 0 0 430.2 73L405 78.9q-2.2-1.8-4.4-3.6l.9-25.8a7.4 7.4 0 0 0 -10.4-7L367.7 53.2c-1.7-.9-3.4-1.8-5.2-2.7L358.4 25.1a7.4 7.4 0 0 0 -11.5-4.8L326 35.3c-1.9-.6-3.8-1.1-5.6-1.7l-9-24a7.4 7.4 0 0 0 -12.3-2.4l-17.5 18.7c-2-.2-3.9-.4-5.9-.6L262.3 3.5a7.4 7.4 0 0 0 -12.5 0L236.2 25.3c-2 .2-3.9 .3-5.9 .6L212.9 7.1a7.4 7.4 0 0 0 -12.3 2.4l-9 24c-1.9 .6-3.8 1.1-5.7 1.7l-20.8-15a7.4 7.4 0 0 0 -11.5 4.8l-4.2 25.4c-1.7 .9-3.5 1.8-5.2 2.7L120.9 42.6a7.4 7.4 0 0 0 -10.4 7l.9 25.8c-1.5 1.2-3 2.4-4.4 3.6L81.8 73A7.4 7.4 0 0 0 73 81.8L78.9 107c-1.2 1.5-2.4 2.9-3.6 4.4l-25.8-.9a7.4 7.4 0 0 0 -6.4 3.3 7.4 7.4 0 0 0 -.6 7.1l10.7 23.4c-.9 1.7-1.8 3.4-2.7 5.2L25.1 153.6a7.4 7.4 0 0 0 -4.8 11.5l15 20.8c-.6 1.9-1.1 3.8-1.7 5.7l-24 9a7.4 7.4 0 0 0 -2.4 12.3l18.7 17.5c-.2 2-.4 3.9-.6 5.9L3.5 249.8a7.4 7.4 0 0 0 0 12.5L25.3 275.8c.2 2 .3 3.9 .6 5.9L7.1 299.1a7.4 7.4 0 0 0 2.4 12.3l24 9c.6 1.9 1.1 3.8 1.7 5.7l-15 20.8a7.4 7.4 0 0 0 4.8 11.5l25.4 4.2c.9 1.7 1.8 3.5 2.7 5.1L42.6 391.1a7.4 7.4 0 0 0 .6 7.1 7.1 7.1 0 0 0 6.4 3.3l25.8-.9q1.8 2.2 3.6 4.4L73 430.2A7.4 7.4 0 0 0 81.8 439L107 433.1q2.2 1.8 4.4 3.6l-.9 25.8a7.4 7.4 0 0 0 10.4 7l23.4-10.7c1.7 .9 3.4 1.8 5.1 2.7l4.2 25.4a7.3 7.3 0 0 0 11.5 4.8l20.8-15c1.9 .6 3.8 1.1 5.7 1.7l9 24a7.4 7.4 0 0 0 12.3 2.4l17.5-18.7c2 .2 3.9 .4 5.9 .6l13.5 21.8a7.4 7.4 0 0 0 12.5 0l13.5-21.8c2-.2 3.9-.3 5.9-.6l17.5 18.7a7.4 7.4 0 0 0 12.3-2.4l9-24c1.9-.6 3.8-1.1 5.7-1.7l20.8 15a7.3 7.3 0 0 0 11.5-4.8l4.2-25.4c1.7-.9 3.5-1.8 5.2-2.7l23.4 10.7a7.4 7.4 0 0 0 10.4-7l-.9-25.8q2.2-1.8 4.4-3.6L430.2 439a7.4 7.4 0 0 0 8.8-8.8L433.1 405q1.8-2.2 3.6-4.4l25.8 .9a7.2 7.2 0 0 0 6.4-3.3 7.4 7.4 0 0 0 .6-7.1L458.8 367.7c.9-1.7 1.8-3.4 2.7-5.2l25.4-4.2a7.4 7.4 0 0 0 4.8-11.5l-15-20.8c.6-1.9 1.1-3.8 1.7-5.7l24-9a7.4 7.4 0 0 0 2.4-12.3l-18.7-17.5c.2-2 .4-3.9 .6-5.9l21.8-13.5a7.4 7.4 0 0 0 0-12.5zm-151 129.1A13.9 13.9 0 0 0 341 389.5l-7.6 35.7A187.5 187.5 0 0 1 177 424.4l-7.6-35.7a13.9 13.9 0 0 0 -16.5-10.7l-31.5 6.8a187.4 187.4 0 0 1 -16.3-19.2H258.3c1.7 0 2.9-.3 2.9-1.9V309.6c0-1.6-1.2-1.9-2.9-1.9H213.5l.1-34.4H262c4.4 0 23.7 1.3 29.8 25.9 1.9 7.6 6.2 32.1 9.1 40 2.9 8.8 14.6 26.5 27.1 26.5H407a187.3 187.3 0 0 1 -17.3 20.1zm25.8 34.5A15.2 15.2 0 1 1 368 398.1h.4A15.2 15.2 0 0 1 383.2 413.3zm-225.6-.7a15.2 15.2 0 1 1 -15.3-15.3h.5A15.3 15.3 0 0 1 157.6 412.6zM69.6 234.2l32.8-14.6a13.9 13.9 0 0 0 7.1-18.3L102.7 186h26.6V305.7H75.7A187.7 187.7 0 0 1 69.6 234.2zM58.3 198.1a15.2 15.2 0 0 1 15.2-15.3H74a15.2 15.2 0 1 1 -15.7 15.2zm155.2 24.5 .1-35.3h63.3c3.3 0 23.1 3.8 23.1 18.6 0 12.3-15.2 16.7-27.7 16.7zM399 306.7c-9.8 1.1-20.6-4.1-22-10.1-5.8-32.5-15.4-39.4-30.6-51.4 18.9-12 38.5-29.6 38.5-53.3 0-25.5-17.5-41.6-29.4-49.5-16.8-11-35.3-13.2-40.3-13.2H116.3A187.5 187.5 0 0 1 221.2 70.1l23.5 24.6a13.8 13.8 0 0 0 19.6 .4l26.3-25a187.5 187.5 0 0 1 128.4 91.4l-18 40.6A14 14 0 0 0 408 220.4l34.6 15.3a187.1 187.1 0 0 1 .4 32.5H423.7c-1.9 0-2.7 1.3-2.7 3.1v8.8C421 301 409.3 305.6 399 306.7zM240 60.2A15.2 15.2 0 0 1 255.2 45h.5A15.2 15.2 0 1 1 240 60.2zM436.8 214a15.2 15.2 0 1 1 0-30.5h.4a15.2 15.2 0 0 1 -.4 30.5z"/>
                </svg>
                <!-- Actix Login -->
                <h1 class="text-neutral-700 dark:text-neutral-200 text-2xl font-bold">Actix Login</h1>
            </a>

            <!-- Close button -->
            <button on:click={close} class="text-neutral-800 dark:text-neutral-200 relative group">
                <svg class="w-6 h-6 fill-neutral-800 dark:fill-neutral-200" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path class="stroke-current" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                </svg>
                <div class="absolute top-1/2 left-1/2 w-0 h-0 bg-neutral-600 opacity-25 rounded-full transform -translate-x-1/2 -translate-y-1/2 transition-all duration-200 group-hover:w-10 group-hover:h-10"></div>
            </button>
        </div>

        <!-- Navbar -->
        <nav class="mt-6 inline-flex flex-col space-y-6">
            <a href={'/'} on:click={close} class="text-neutral-700 hover:text-neutral-800 dark:text-neutral-200 hover:dark:text-neutral-500 group transition inline-flex space-x-2">
                <svg class="fill-neutral-700 group-hover:fill-neutral-800 dark:fill-neutral-200 group-hover:dark:fill-neutral-500 transition" xmlns="http://www.w3.org/2000/svg" width="23" height="23" viewBox="0 0 576 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M575.8 255.5c0 18-15 32.1-32 32.1l-32 0 .7 160.2c0 2.7-.2 5.4-.5 8.1l0 16.2c0 22.1-17.9 40-40 40l-16 0c-1.1 0-2.2 0-3.3-.1c-1.4 .1-2.8 .1-4.2 .1L416 512l-24 0c-22.1 0-40-17.9-40-40l0-24 0-64c0-17.7-14.3-32-32-32l-64 0c-17.7 0-32 14.3-32 32l0 64 0 24c0 22.1-17.9 40-40 40l-24 0-31.9 0c-1.5 0-3-.1-4.5-.2c-1.2 .1-2.4 .2-3.6 .2l-16 0c-22.1 0-40-17.9-40-40l0-112c0-.9 0-1.9 .1-2.8l0-69.7-32 0c-18 0-32-14-32-32.1c0-9 3-17 10-24L266.4 8c7-7 15-8 22-8s15 2 21 7L564.8 231.5c8 7 12 15 11 24z"/></svg>
                <span>Home</span>
            </a>
            <a href={'/auth/login'} on:click={close} class="text-neutral-700 hover:text-neutral-800 dark:text-neutral-200 hover:dark:text-neutral-500 group transition inline-flex space-x-2">
                <svg class="fill-neutral-700 group-hover:fill-neutral-800 dark:fill-neutral-200 group-hover:dark:fill-neutral-500 transition" xmlns="http://www.w3.org/2000/svg" width="23" height="23" viewBox="0 0 512 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M217.9 105.9L340.7 228.7c7.2 7.2 11.3 17.1 11.3 27.3s-4.1 20.1-11.3 27.3L217.9 406.1c-6.4 6.4-15 9.9-24 9.9c-18.7 0-33.9-15.2-33.9-33.9l0-62.1L32 320c-17.7 0-32-14.3-32-32l0-64c0-17.7 14.3-32 32-32l128 0 0-62.1c0-18.7 15.2-33.9 33.9-33.9c9 0 17.6 3.6 24 9.9zM352 416l64 0c17.7 0 32-14.3 32-32l0-256c0-17.7-14.3-32-32-32l-64 0c-17.7 0-32-14.3-32-32s14.3-32 32-32l64 0c53 0 96 43 96 96l0 256c0 53-43 96-96 96l-64 0c-17.7 0-32-14.3-32-32s14.3-32 32-32z"/></svg>
                <span>Login</span>
            </a>
            <a href={'/auth/register'} on:click={close} class="text-neutral-700 hover:text-neutral-800 dark:text-neutral-200 hover:dark:text-neutral-500 group transition inline-flex space-x-2">
                <svg class="fill-neutral-700 group-hover:fill-neutral-800 dark:fill-neutral-200 group-hover:dark:fill-neutral-500 transition" xmlns="http://www.w3.org/2000/svg" width="23" height="23" viewBox="0 0 512 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M362.7 19.3L314.3 67.7 444.3 197.7l48.4-48.4c25-25 25-65.5 0-90.5L453.3 19.3c-25-25-65.5-25-90.5 0zm-71 71L58.6 323.5c-10.4 10.4-18 23.3-22.2 37.4L1 481.2C-1.5 489.7 .8 498.8 7 505s15.3 8.5 23.7 6.1l120.3-35.4c14.1-4.2 27-11.8 37.4-22.2L421.7 220.3 291.7 90.3z"/></svg>
                <span>Register</span>
            </a>
            <a href="#!" on:click={close} class="text-neutral-700 hover:text-neutral-800 dark:text-neutral-200 hover:dark:text-neutral-500 group transition inline-flex space-x-2">
                <svg class="fill-neutral-700 group-hover:fill-neutral-800 dark:fill-neutral-200 group-hover:dark:fill-neutral-500 transition" xmlns="http://www.w3.org/2000/svg" width="23" height="23" viewBox="0 0 512 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M256 8C119 8 8 119.1 8 256c0 137 111 248 248 248s248-111 248-248C504 119.1 393 8 256 8zm0 110c23.2 0 42 18.8 42 42s-18.8 42-42 42-42-18.8-42-42 18.8-42 42-42zm56 254c0 6.6-5.4 12-12 12h-88c-6.6 0-12-5.4-12-12v-24c0-6.6 5.4-12 12-12h12v-64h-12c-6.6 0-12-5.4-12-12v-24c0-6.6 5.4-12 12-12h64c6.6 0 12 5.4 12 12v100h12c6.6 0 12 5.4 12 12v24z"/></svg>
                <span>About</span>
            </a>
            <a href="#!" on:click={close} class="text-neutral-700 hover:text-neutral-800 dark:text-neutral-200 hover:dark:text-neutral-500 group transition inline-flex space-x-2">
                <svg class="fill-neutral-700 group-hover:fill-neutral-800 dark:fill-neutral-200 group-hover:dark:fill-neutral-500 transition" xmlns="http://www.w3.org/2000/svg" width="23" height="23" viewBox="0 0 512 512"><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path d="M48 64C21.5 64 0 85.5 0 112c0 15.1 7.1 29.3 19.2 38.4L236.8 313.6c11.4 8.5 27 8.5 38.4 0L492.8 150.4c12.1-9.1 19.2-23.3 19.2-38.4c0-26.5-21.5-48-48-48L48 64zM0 176L0 384c0 35.3 28.7 64 64 64l384 0c35.3 0 64-28.7 64-64l0-208L294.4 339.2c-22.8 17.1-54 17.1-76.8 0L0 176z"/></svg>
                <span>Contact</span>
            </a>
        </nav>
    </div>
</div>
