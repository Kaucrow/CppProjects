import { writable } from 'svelte/store';
import type { CustomError } from '$lib/utils/types';

export const errStore = writable<CustomError[]>([]);