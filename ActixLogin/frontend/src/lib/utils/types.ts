export interface ApiResponse {
    message?: string;
    status?: string;
}

export interface NumResponse {
    number: number;
    status?: string;
}

export interface CustomError {
    error?: string;
    id?: number;
}

export interface ErrorResponse {
    error: string;
}

export interface User {
    email: string;
    first_name: string;
    last_name: string;
    id: string;
    is_staff: boolean;
    thumbnail: string;
    is_superuser: boolean;
}

export interface NewUser {
    email: string,
    password: string,
    first_name: string,
    last_name: string,
}

export interface LoginUser {
    email: string,
    password: string,
}

export interface AddNumBody {
    number: number;
}

type Status = 'IDLE' | 'LOADING' | 'NAVIGATING';

export interface Loading {
	status: Status;
	message: string;
}