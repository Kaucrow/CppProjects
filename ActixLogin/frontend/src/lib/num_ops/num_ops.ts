import { loading } from '../stores/loading.store';
import { page } from '$app/stores';
import { API_URI } from '$lib/utils/constant';
import { get } from '$lib/utils/requests/get';
import type { NumResponse, CustomError } from '$lib/utils/types';
import { isNumResponse } from '../utils/typeguards'

export async function getNum(fetchFn: typeof fetch) {
    const [res, errors] = await get(fetchFn, `${API_URI}/get-num/`);
    if (isNumResponse(res)) {
        console.log('Valid num object:', res.number);
        return {
            initialValue: res.number,
            errors: errors,
        }
    } else {
        console.error('Invalid response from API', res);
        return {
            initialValue: undefined,
            errors: errors,
        }
    }
}