import { useState, useCallback, useEffect } from 'react';
import { IpLookupResult } from '@interfaces/ApiInterfaces';
import { lookupSelfIpAddress } from '@src/services/ipLookupService';

export const useIpLookup = () => {
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [result, setResult] = useState<IpLookupResult | null>(null);

    // Look up the current user's IP address
    const lookupSelf = useCallback(async () => {
        setIsLoading(true);
        setError(null);

        try {
            const data = await lookupSelfIpAddress();
            setResult(data);
            return data;
        } catch (error) {
            console.error('[IP Lookup] API request failed:', error);
            throw error;
        } finally {
            setIsLoading(false);
        }
    }, []);

    // Auto-fetch IP data when the hook is used
    useEffect(() => {
        lookupSelf();
    }, [lookupSelf]);

    // Function to manually refresh the IP lookup
    const refresh = useCallback(() => {
        return lookupSelf();
    }, [lookupSelf]);

    return {
        // Data
        data: result,
        
        // State
        isLoading,
        error,
        
        // Actions
        refresh,
        lookupSelf,
    };
};

export default useIpLookup;
