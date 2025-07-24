import { useState, useCallback } from 'react';
import { IpLookupResult } from '@interfaces/ApiInterfaces';
import { lookupIpAddress } from '@src/services/ipLookupService';

export const useIpLookupCustom = () => {
    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [result, setResult] = useState<IpLookupResult | null>(null);

    const lookupIp = useCallback(async (ip: string) => {
        // Basic IP validation
        if (!ip || !/^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$/.test(ip)) {
            setError('Please enter a valid IPv4 address');
            return null;
        }

        setIsLoading(true);
        setError(null);

        try {
            const data = await lookupIpAddress(ip);
            setResult(data);
            return data;
        } catch (err) {
            const error = err as Error;
            setError(error.message || 'Failed to fetch IP information');
            return null;
        } finally {
            setIsLoading(false);
        }
    }, []);

    return {
        lookupIp,
        result,
        isLoading,
        error,
        reset: () => {
            setResult(null);
            setError(null);
        }
    };
};

export default useIpLookupCustom;
