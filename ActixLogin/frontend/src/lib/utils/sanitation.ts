export function isValidEmail(email: string): boolean {
    const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return re.test(email.toLowerCase());
}

export function isValidPassword(password: string): boolean {
    const re = /^(?=.*[0-9])(?=.*[!@#$%^&*()_+{}\[\]:;"'<>,.?~`-])(?=.*[a-z])(?=.*[A-Z])[A-Za-z\d!@#$%^&*()_+{}\[\]:;"'<>,.?~`-]{8,}$/;
    return re.test(password); 
}