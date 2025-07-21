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
        } catch (err) {
            const error = err as Error;
            // For demo purposes, return mock data on error
            const mockData: IpLookupResult = {
                ip: 'Frontend Error',
                country: {
                    names: {
                        en: 'Frontend Error'
                    }
                },
                city: 'Frontend Error',
                isp: 'Frontend Error',
                isVpn: false,
                isProxy: false,
                isTor: false,
                threatScore: 0,
                recommendedAction: 'Frontend Error',
                latitude: 0,
                longitude: 0
            };
            setResult(mockData);
            setError('Using demo data due to: ' + error.message);
            return mockData;
        } finally {
            setIsLoading(false);
        }
    }, []);

    // Auto-fetch IP data when the hook is used
    useEffect(() => {
        lookupSelf();
    }, [lookupSelf]);

    return {
        // State
        isLoading,
        error,
        result,
        
        // Action
        lookupSelf,
        
        // Reset state
        reset: () => {
            setError(null);
            setResult(null);
        },
    };
};

export default useIpLookup;
