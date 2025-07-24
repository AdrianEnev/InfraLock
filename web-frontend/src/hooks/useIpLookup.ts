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
            // For demo purposes, return mock data on error that matches IpLookupResult
            const mockData: IpLookupResult = {
                ip: '8.8.8.8',
                country: 'United States',
                city: 'Mountain View',
                asnInfo: {
                    autonomous_system_number: 15169,
                    autonomous_system_organization: 'Google LLC'
                },
                isVpn: true,
                isProxy: false,
                isTor: false,
                threatScore: 100,
                threatDetails: ['Using demo data due to error'],
                recommendedAction: 'monitor',
                latitude: 37.422,
                longitude: -122.084,
                proxyType: null
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
