// Function to get the value of a specific cookie
export function getCookie(name: String) {
    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);
    if (parts.length === 2) {
        let parts_pop = parts.pop();
        if (parts_pop !== undefined) return parts_pop.split(';').shift();
    }
    return null;
}