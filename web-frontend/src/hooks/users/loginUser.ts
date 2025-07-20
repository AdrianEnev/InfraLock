import { API_BASE_URL } from '@src/constants/api';

interface LoginResponse {
    id: string;
    email: string;
    apiKey: string;
    token: string;
}

const loginUser = async (email: string, password: string): Promise<LoginResponse | null> => {
    try {
        const response = await fetch(`${API_BASE_URL}/users/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            credentials: 'include', // Important for cookies
            body: JSON.stringify({
                email,
                password,
            }),
        });
        
        if (!response.ok) {
            const errorData = await response.json().catch(() => ({}));
            console.error('Login failed:', errorData.error || 'Unknown error');
            return null;
        }

        const data: LoginResponse = await response.json();
        
        // The backend should set the JWT in an HTTP-only cookie
        // We'll store the API key in localStorage for now (can be moved to secure storage if needed)
        if (data.apiKey) {
            localStorage.setItem('apiKey', data.apiKey);
        }

        return data;
    } catch (error) {
        console.error('Login error:', error);
        return null;
    }
};

export default loginUser;