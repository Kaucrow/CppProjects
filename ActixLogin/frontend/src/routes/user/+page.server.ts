import type { ServerLoad } from '@sveltejs/kit';
import { API_URI } from '$lib/utils/constant';
import { redirect } from '@sveltejs/kit';
import { get } from '$lib/utils/requests/get';
import { CLT_MOD } from '$lib/utils/constant';

export const load: ServerLoad = async ({ fetch, cookies }) => {
    const id = cookies.get('id');

    if (id) {
        const [res, err] = await get(fetch, `${API_URI}/users/validate`);

        if (res.ok) {
            const data = await res.json();

            if (data.isSuperuser) {
                throw redirect(301, `${CLT_MOD}/`);
            }
        } else {
            throw redirect(301, `${CLT_MOD}/`);
        }
    } else {
        throw redirect(301, `${CLT_MOD}/`);
    }
};