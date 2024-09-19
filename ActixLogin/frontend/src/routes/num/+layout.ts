import { getNum } from '$lib/num_ops/num_ops';
import type { Load } from '@sveltejs/kit';

export const load: Load = async ({ fetch }) => {
    const { initialValue, errors } = await getNum(fetch);
    return {
        initialValue,
        errors,
        fetch,
    };
}