'use client';
import { useRouter } from 'next/navigation';
import { createContext, useContext, useEffect, useState, useCallback } from 'react';
import getUserToken from '@src/hooks/users/getUserToken';
import logoutUser from '@src/hooks/users/logoutUser';

interface User {
    id: string;
    email: string;
    apiKey?: string;
    role: string;
}

interface GlobalContextType {
    isAuthenticated: boolean;
    user: User | null;
    isLoading: boolean;
    onAuthenticate: () => Promise<void>;
    logOut: () => Promise<void>;
}
const GlobalContext = createContext<GlobalContextType | null>(null);

export const GlobalProvider = ({ children }: { children: React.ReactNode }) => {
    const router = useRouter();
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [isLoading, setIsLoading] = useState(true);
    const [user, setUser] = useState<User | null>(null);

    // Check authentication status and get user data
    const checkAuth = useCallback(async () => {
        try {
            setIsLoading(true);
            const userData = await getUserToken();
            
            if (userData) {
                // Ensure all required fields are present
                const userWithRole: User = {
                    id: userData.id,
                    email: userData.email,
                    apiKey: userData.apiKey,
                    role: userData.role || 'user' // Default to 'user' role if not provided
                };
                setUser(userWithRole);
                setIsAuthenticated(true);
            } else {
                setUser(null);
                setIsAuthenticated(false);
            }
        } catch (error) {
            console.error('Authentication check failed:', error);
            setUser(null);
            setIsAuthenticated(false);
            setUser(null);
            setIsAuthenticated(false);
        } finally {
            setIsLoading(false);
        }
    }, []);

    // Handle authentication
    const onAuthenticate = useCallback(async () => {
        await checkAuth();
    }, [checkAuth]);

    // Handle logout
    const logOut = useCallback(async () => {
        try {
            await logoutUser();
            setUser(null);
            setIsAuthenticated(false);
            router.push('/login');
        } catch (error) {
            console.error('Logout failed:', error);
        }
    }, [router]);

    // Initial auth check
    useEffect(() => {
        checkAuth();
    }, [checkAuth]);

    const value = {
        isAuthenticated,
        user,
        isLoading,
        onAuthenticate,
        logOut,
    };

    return (
        <GlobalContext.Provider value={value}>
            {!isLoading && children}
        </GlobalContext.Provider>
    );
};

export const useGlobalContext = (): GlobalContextType => {
    const context = useContext(GlobalContext);
    if (!context) {
        throw new Error('useGlobalContext must be used within a GlobalProvider');
    }
    return context;
};