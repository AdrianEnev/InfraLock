import { API_BASE_URL } from '@src/constants/api';

const logoutUser = async (): Promise<boolean> => {
    try {
        const response = await fetch(`${API_BASE_URL}/users/logout`, {
            method: 'POST',
            credentials: 'include', // Important for cookies
            headers: {
                'Content-Type': 'application/json',
            },
        });
        
        if (!response.ok) {
            const error = await response.json().catch(() => ({}));
            console.error('Logout failed:', error.message || 'Unknown error');
            return false;
        }

        // Clear any client-side stored tokens
        localStorage.removeItem('apiKey');
        
        return true;
    } catch (error) {
        console.error('Logout error:', error);
        return false;
    }
};

export default logoutUser;