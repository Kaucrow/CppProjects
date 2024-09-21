// frontend/src/lib/utils/requests/posts.requests.ts
import type {
    ApiResponse,
    CustomError,
    User,
    NewUser,
    LoginUser,
    AddNumBody,
    NumResponse,
} from '../types';

/**
 * Handle all POST-related requests.
 * @file lib/utils/requests/post.requests.ts
 * @param {typeof fetch} sveltekitFetch - Fetch object from sveltekit
 * @param {typeof fetch} url - The URL whose resource will be fetched
 * @param {AddNumBody | undefined} body - Body of the POST request
 * @param {RequestCredentials} [credentials='omit'] - Request credential. Defaults to 'omit'.
 * @param {'POST' | 'PUT' | 'PATCH' | 'DELETE'} [method='POST'] - Request method. Defaults to 'POST'.
 */
export const post = async (
    sveltekitFetch: typeof fetch,
    url: string,
    body: AddNumBody | NewUser | LoginUser | undefined,
    method: 'POST' | 'PUT' | 'PATCH' | 'DELETE' = 'POST'
): Promise<[object, Array<CustomError>]> => {
    try {
        const headers = { 'Content-Type': '' };
        // used in sveltekitFetch
        const requestInitOptions: RequestInit = {
            method: method,
            mode: 'cors',
            credentials: 'include',
        };
        // make preliminary checks
        if(!(body instanceof FormData)) {
            headers['Content-Type'] = 'application/json';
            requestInitOptions['headers'] = headers;
            if (body !== undefined) {
                requestInitOptions.body = JSON.stringify(body);
            }
        } else if (body instanceof FormData) {
            headers['Content-Type'] = 'multipart/form-data';
            if (body !== undefined) {
                requestInitOptions['body'] = body;
            }
        }
        /*if (body === undefined && method !== 'DELETE') {
            const errors: Array<CustomError> = [
                { error: 'Unless you are performing a DELETE operation, you must have a body.', id: 0 }
            ];
            return [{}, errors];
        }*/

        console.log(`Sending POST request to ${url}`)
        const res = await sveltekitFetch(url, requestInitOptions);

        // handle fetch errors
        if (!res.ok) {
            const response = await res.json();
            const errors: Array<CustomError> = [];

            errors.push( {error: response.error, id: 0 });

            return [{}, errors];
        }

        const res_json = await res.json();

        let response: NumResponse | ApiResponse | User;

        if (res_json['message']) {
            response = { message: res_json['message'], status: res_json['status'] };
        } else if (res_json['number']) {
            response = { number: res_json['number'] };
        } else {
            response = {
                id: res_json['id'],
                email: res_json['email'],
                first_name: res_json['first_name'],
                last_name: res_json['last_name'],
                is_staff: res_json['is_staff'],
                thumbnail: res_json['thumbnail'],
                is_superuser: res_json['is_superuser'],
            };
        }

        console.log('POST request succeeded.');

        return [response, []];
    } catch (error) {
        console.error(`Error outside: ${error}`);
        const err = `${error}`;
        const errors: Array<CustomError> = [
            { error: 'An unknown error occurred.', id: 0 },
            { error: err, id: 1 }
        ];
        return [{}, errors];
    }
};