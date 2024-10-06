import type { ServerLoad } from '@sveltejs/kit';
import { API_URI } from '$lib/utils/constant';
import { redirect } from '@sveltejs/kit';

export const load: ServerLoad = async ({ fetch, cookies }) => {
    const id = cookies.get('id');

    if (id) {
        const res = await fetch(`${API_URI}/users/validate`, {
            method: 'GET',
            credentials: 'include'
        });

        if (res.ok) {
            const data = await res.json();

            if (data.isSuperuser) {
                throw redirect(301, '/');
            }
        } else {
            throw redirect(301, '/');
        }
    } else {
        throw redirect(301, '/');
    }
};