// frontend/src/lib/utils/constant.ts
export const API_URI: string = import.meta.env.DEV
    ? import.meta.env.VITE_API_URI_DEV
    : import.meta.env.VITE_API_URI_PROD;

export const CLT_MOD: string = import.meta.env.DEV
    ? ''
    : import.meta.env.VITE_CLT_MOD;