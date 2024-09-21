import { loading } from '../stores/loading.store';
import { page } from '$app/stores';
import { API_URI } from '$lib/utils/constant';
import { get } from '$lib/utils/requests/get.requests';
import type { NumResponse, CustomError } from '$lib/utils/types';
import { isNumResponse } from '../utils/typeguards'
/*
let errors: Array<CustomError> = [];

export async function getNum(sveltekitFetch: typeof fetch): Promise<[number, Array<CustomError>]> {
    let value = 0;
    loading.setLoading(true, 'Please wait...');
    const [res, err] = await get(sveltekitFetch, `${API_URI}/get-num/`);

    if (err.length > 0) {
        loading.setLoading(false);
        errors = err;
        return [0, errors];
    } else {
        loading.setLoading(false);
        if (isNumResponse(res)) {
            const response: NumResponse = res;
            value = response['number'];
            return [value, []];
        } else {
            errors = [
                {
                    'error': 'Received a non-NumResponse response from server.'
                },
            ];
            return [0, errors];
        };
    }
};*/

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