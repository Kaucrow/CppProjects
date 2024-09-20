<script lang="ts">
    export let data;

    import { post } from '$lib/utils/requests/post.requests';
    import { API_URI } from '$lib/utils/constant'

    let email = '';
    let password = '';
    let first_name = '';
    let last_name = '';

    async function submitForm() {
        const newUser = {
            email,
            password,
            first_name,
            last_name
        };

        const [res, err] = await post(data.fetch, `${API_URI}/users/register/`, newUser);

        if (err.length > 0) {
            console.error('Failed to create user');
        } else {
            console.log('User created successfully');
        }
    }
</script>

<form on:submit|preventDefault={submitForm}>
    <div class="p-2">
        <label for="email">Email:</label>
        <input type="email" id="email" bind:value={email} required>
    </div>

    <div class="p-2">
        <label for="password">Password:</label>
        <input type="password" id="password" bind:value={password} required>
    </div>

    <div class="p-2">
        <label for="first_name">First Name:</label>
        <input type="text" id="first_name" bind:value={first_name} required>
    </div>

    <div class="p-2">
        <label for="last_name">Last Name:</label>
        <input type="text" id="last_name" bind:value={last_name} required>
    </div>

    <button class="p-2" type="submit">Submit</button>
</form>