// frontend/src/lib/stores/notification.store.ts
import { writable } from 'svelte/store';

export const notification = writable({
    message: '',
    borderColor: '',
    textTopColor: '',
    textBottomColor: ''
});