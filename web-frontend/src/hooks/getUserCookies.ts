import { cookies } from 'next/headers';

/**
 * Retrieves the authentication token from cookies
 * @returns The JWT token if found, otherwise null
 */
const getUserCookies = async (): Promise<string | null> => {
    try {
        const cookieStore = cookies();
        const token = (await cookieStore).get('token')?.value; // Changed from 'userToken' to 'token' to match backend
        return token || null;
    } catch (error) {
        console.error('Error getting auth cookies:', error);
        return null;
    }
};

export default getUserCookies;