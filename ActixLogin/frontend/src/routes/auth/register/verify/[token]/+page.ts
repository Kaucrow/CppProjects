import type { Load } from '@sveltejs/kit';

export const load: Load = async ({ params }) => {
    console.log(params);
    return {
        slug: params.token
    }
}