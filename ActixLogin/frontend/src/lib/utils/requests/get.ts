import type {
    ApiResponse,
    NumResponse,
    CustomError,
    LoginUser,
} from "../types";

export const get = async (
    sveltekitFetch: typeof fetch,
    url: string,
): Promise<[Response, Array<CustomError>]> => {
    // used in fetch
    const requestInitOptions: RequestInit = {
        method: 'GET',
        mode: 'cors',
    };

    console.log(`Sending GET request to ${url}`)
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

    console.log('GET request succeeded.');

    return [res, []];
}; 