import { API_BASE_URL } from '@src/constants/api';

interface UserResponse {
    id: string;
    email: string;
    apiKey: string;
    role: string;
}

const getUserToken = async (): Promise<UserResponse | null> => {
    try {
        const response = await fetch(`${API_BASE_URL}/users/me`, {
            method: 'GET',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
            },
        });

        if (response.status === 401) {
            //console.log('User not authenticated');
            return null;
        }

        if (!response.ok) {
            const errorText = await response.text();
            console.error('Failed to fetch user. Status:', response.status);
            console.error('Response text:', errorText);
            try {
                const errorJson = JSON.parse(errorText);
                console.error('Error details:', errorJson);
            } catch (e) {
                console.error('Could not parse error response as JSON');
            }
            return null;
        }

        const data: UserResponse = await response.json();
        return data;
    } catch (error) {
        console.error('Error fetching user:', error);
        return null;
    }
};

export default getUserToken;