import type {
    ApiResponse,
    NumResponse,
    CustomError,
    LoginUser,
} from "../types";

export const get = async (
    sveltekitFetch: typeof fetch,
    url: string,
): Promise<[object, Array<CustomError>]> => {
    try {
        // used in fetch
        const requestInitOptions: RequestInit = {
            method: 'GET',
            mode: 'cors',
            credentials: 'include',
        };

        console.log(`Sending GET request to ${url}`)
        const res = await sveltekitFetch(url, requestInitOptions);

        //handle fetch errors
        if (!res.ok) {
            const response = await res.json();
            const errors: Array<CustomError> = [];

            errors.push( {error: response.error, id: 0});

            return [{}, errors];
        }

        const res_json = await res.json();
 
        let response: ApiResponse | NumResponse;

        if (res_json['message']) {
            response = { message: res_json['message'], status: res_json['status'] };
        } else {
            response = { number: res_json['number'], status: res_json['status'] };
        };

        console.log('GET request succeeded.');

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