<script lang='ts'>
    export let data;

    import type { CustomError } from '$lib/utils/types';
    import { getNum } from '$lib/num_ops/num_ops';

    let errors: Array<CustomError> = data.errors || [];
    let value = data.initialValue;

    function increment() {
        if (value !== undefined) {
            getNum(data.fetch);
        }
    }
</script>

<button on:click={increment}>
    {value}
</button>

<div>
    {#if errors && errors.length > 0}
        <ul>
            {#each errors as err}
                <li>{err.error}</li>
            {/each}
        </ul>
    {/if}
</div>