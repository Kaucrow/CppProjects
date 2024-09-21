<!-- src/lib/components/LogoutButton.svelte -->
<script lang="ts">
    import type { Data } from '$lib/utils/types';
    export let data: Data;

    import { post } from '$lib/utils/requests/post.requests'; 
    import { API_URI } from '$lib/utils/constant';
    import { errStore } from '$lib/stores/common.store';

    async function logout() {
        const [res, err] = await post(data.fetch, `${API_URI}/users/logout/`, undefined);
        if (err.length > 0) {
            errStore.set(err);
        } else {
            console.log("LOGGED OUT");
        }
    }
</script>

<button class="p-3 m-7 bg-slate-950 text-slate-50 border-0 rounded-2xl" on:click={logout}>
    Logout
</button>