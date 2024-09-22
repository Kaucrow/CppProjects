<script lang="ts">
    export let isOpen: boolean;
    export let close: () => void;
</script>

<div class={`fixed z-50 top-0 left-0 h-full w-64 bg-neutral-200 dark:bg-neutral-800 transform ${isOpen ? 'translate-x-0' : '-translate-x-full'} transition-transform duration-300 ease-in-out`}>
    <div class="p-4">
        <button on:click={close} class="text-neutral-800 dark:text-neutral-200 relative group">
            <svg class="w-6 h-6 fill-neutral-800 dark:fill-neutral-200" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path class="stroke-current" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
            <div class="absolute top-1/2 left-1/2 w-0 h-0 bg-neutral-600 opacity-25 rounded-full transform -translate-x-1/2 -translate-y-1/2 transition-all duration-200 group-hover:w-10 group-hover:h-10"></div>
        </button>
        
        <nav class="mt-4 flex flex-col space-y-4">
            <a href="/" class="hover:text-neutral-400 transition">Home</a>
            <a href="/auth/login" class="hover:text-neutral-400 transition">Login</a>
            <a href="/auth/register" class="hover:text-neutral-400 transition">Register</a>
            <a href="#!" class="hover:text-neutral-400 transition">About</a>
            <a href="#!" class="hover:text-neutral-400 transition">Contact</a>
        </nav>
    </div>
</div>