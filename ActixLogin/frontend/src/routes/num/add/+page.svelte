<script lang='ts'>
    export let data;

    import type { NumResponse, CustomError, AddNumBody } from '$lib/utils/types';
    import { loading } from '$lib/stores/loading.store';
    import { isNumResponse } from '$lib/utils/typeguards';
    import { post } from '$lib/utils/requests/post.requests';
    import { API_URI } from '$lib/utils/constant';

    let errors: Array<CustomError> = data.errors || [];
    let value = data.initialValue;
    let inputValue = '';

    async function addNum() {
        errors = [];
        if (isNaN(Number(inputValue))) {
            errors = [{
                error: 'Input is not a number.',
            }];
            return;
        }
        loading.setLoading(true, 'Please wait...');
        const [res, err] = await post(
            data.fetch,
            `${API_URI}/add-num/`,
            {
                number: Number(inputValue),
            }
        );

        if (err.length > 0) {
            loading.setLoading(false);
            errors = err;
            return;
        }

        loading.setLoading(false);
        if (isNumResponse(res)) {
            const response: NumResponse = res;
            value = response['number'];
        } else {
            errors = [
                {
                    'error': `Received a non-NumResponse response from server: ${JSON.stringify(res)}`
                },
            ];
        };
    };
</script>

<div>
    {value}
</div>

<form on:submit|preventDefault={addNum}>
    <div class="relative border-2 w-1/3 focus-within:border-blue-500">
        <input type="number" name="number" placeholder=" "
            class="block p-4 w-full text-lg appearance-none focus:outline-none bg-transparent peer"
            bind:value={inputValue}
        />
        <label for="number"
            class="absolute top-4 left-4 text-lg -z-1 duration-300 transform scale-100 origin-0
            peer-placeholder-shown:scale-100 peer-placeholder-shown:top-4 peer-focus:top-0
            peer-focus:scale-75 peer-focus:-translate-y-2">
            Number
        </label>
    </div>

    <div class="relative border-2 w-1/6 border-black">
        <button type="submit" class="w-full relative focus-within:border-blue-500">Submit</button>
    </div>
</form>

<div>
    {#if errors.length > 0}
        <ul>
            {#each errors as err}
                <li>{err.error}</li>
            {/each}
        </ul>
    {/if}
</div>