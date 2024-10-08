<script lang="ts">
    import { CLT_MOD } from "$lib/utils/constant";
</script>

<body data-sveltekit-preload-data="hover">
    <div>
        <span>You have been registered successfully! You can now </span>
        <span>
            <a class="login" href={`${CLT_MOD}/auth/login`}>login</a>
        </span>
    </div>
</body>

<style lang="postcss">
    .login {
        color:seagreen;
    }
    .login:hover {
        color: darkslategray;
    } 
</style>