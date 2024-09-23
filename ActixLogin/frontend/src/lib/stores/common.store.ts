import { writable } from 'svelte/store';
import type { CustomError } from '$lib/utils/types';

export const errStore = writable<CustomError[]>([]);
export const darkModeStore = writable<boolean>(false);
export const loggedin = writable<boolean>(false);