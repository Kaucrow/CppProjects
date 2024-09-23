<script lang="ts">
    import Header from "$lib/components/Header.svelte";
</script>

<Header />

<section class="flex items-center justify-center my-12">
    <div class="mt-8 flex flex-col items-center justify-center">
        <!-- Email icon -->
        <svg class="fill-neutral-800 dark:fill-neutral-200" xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 512 512">
            <path d="M48 64C21.5 64 0 85.5 0 112c0 15.1 7.1 29.3 19.2 38.4L236.8 313.6c11.4 8.5 27 8.5 38.4 0L492.8 150.4c12.1-9.1 19.2-23.3 19.2-38.4c0-26.5-21.5-48-48-48L48 64zM0 176L0 384c0 35.3 28.7 64 64 64l384 0c35.3 0 64-28.7 64-64l0-208L294.4 339.2c-22.8 17.1-54 17.1-76.8 0L0 176z"/>
        </svg>
        <!-- Box -->
        <div class="flex flex-col items-center justify-center my-8 max-w-[500px] w-full z-10 p-8 rounded-2xl bg-neutral-200 border-neutral-200 dark:bg-neutral-800">
            <p class="text-neutral-800 dark:text-neutral-200 mx-8 text-2xl text-center font-semibold">
                A verification email has been sent to your inbox.
            </p>
            <p class="text-neutral-800 dark:text-neutral-200 mt-4 mx-2 text-center">
                Click the link on your email to verify your account and log in. If you can't find the email, check your spam folder.
            </p>
        </div>
    </div>
</section>
