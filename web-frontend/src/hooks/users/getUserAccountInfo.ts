import { UserAccountInfo } from "@interfaces/UserInterfaces";
import { API_BASE_URL } from "@src/constants/api";

const getUserAccountInfo = async (): Promise<UserAccountInfo | null> => {
    try {
        const response = await fetch(`${API_BASE_URL}/users/me`, {
            method: 'GET',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
            },
        });
        
        if (!response.ok) {
            console.error("Failed to fetch user account info:", await response.text());
            return null;
        }

        const data = await response.json();
        
        // Map the backend response to match UserAccountInfo interface
        return {
            _id: data.id,
            email: data.email,
            username: data.email.split('@')[0], // Using email prefix as username if not provided
            role: data.role,
            apiKey: data.apiKey,
            createdAt: new Date().toISOString() // Using current date if not provided
        };

    } catch (error) {
        console.error("Error fetching user account info:", error);
        return null;
    }
}

export default getUserAccountInfo;