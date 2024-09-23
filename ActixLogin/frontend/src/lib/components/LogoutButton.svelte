<!-- src/lib/components/LogoutButton.svelte -->
<script lang="ts">
    import type { Data } from '$lib/utils/types';
    export let data: Data;

    import { post } from '$lib/utils/requests/post'; 
    import { API_URI } from '$lib/utils/constant';
    import { goto } from '$app/navigation';

    async function logout() {
        const [res, err] = await post(data.fetch, `${API_URI}/users/logout/`, undefined);
        if (res.ok) {
            sessionStorage.removeItem('loggedin')
            goto('/');
        } else {
            console.error("ERROR");
        }
    }
</script>

<button class="px-6 py-2 min-w-52 bg-blue-500 hover:bg-blue-600 text-neutral-200 border-0 rounded-xl font-semibold transition uppercase" on:click={logout}>
    Logout
</button>