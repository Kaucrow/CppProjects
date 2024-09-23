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
): Promise<[Response, Array<CustomError>]> => {
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

    console.log(`Sending POST request to ${url}`)
    const res = await sveltekitFetch(url, requestInitOptions);

    // Handle fetch errors
    if (!res.ok) {
        // Check if the response is JSON before trying to parse it
        const contentType = res.headers.get("content-type");
        const errors: Array<CustomError> = [];

        if (contentType && contentType.includes("application/json")) {
            const response = await res.json();
            errors.push({ error: response.error, id: 0 });
        } else {
            errors.push( { error: "Unknown error", id: 0 })
        }

        return [res, errors];
    } 

    console.log('POST request succeeded.');

    return [res, []];
}; 