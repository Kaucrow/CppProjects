// frontend/src/lib/stores/api.response.store.ts
import { writable } from 'svelte/store';

export const apiResponse = writable({ message: '', status: '' });