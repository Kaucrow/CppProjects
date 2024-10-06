import type { Load } from '@sveltejs/kit';
import type { UserValidation } from '$lib/utils/types';
import { get } from '$lib/utils/requests/get';
import { redirect } from '@sveltejs/kit';
import { API_URI } from '$lib/utils/constant';

export const load: Load = async ({ fetch }) => {
    const [res, err] = await get(fetch, `${API_URI}/users/validate`);

    if (res.ok) {
        let data: UserValidation = await res.json();
        console.log(data);
        if (data.isSuperuser) {
            throw redirect(301, '/')
        }
    } else {
        throw redirect(301, '/')
    }
}