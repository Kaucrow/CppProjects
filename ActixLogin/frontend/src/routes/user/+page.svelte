<script lang="ts">
    import { redirect } from "@sveltejs/kit";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";

    onMount(() => {
        let loggedin = sessionStorage.getItem('loggedin');
        if (loggedin !== 'true') {
            goto('/'); // Redirect to home if not logged in
        }
    })

</script>
<p>User page</p>