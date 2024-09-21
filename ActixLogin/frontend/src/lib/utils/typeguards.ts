import type { NumResponse } from '../utils/types';

export function isNumResponse(obj: any): obj is NumResponse {
    return typeof obj.number === 'number';
}
    