import { api } from '@src/utils/api';
import { IpLookupResult } from '@interfaces/ApiInterfaces';

/**
 * Look up the current user's IP address information using the rust-service
 * @returns Promise with IP lookup results
 */
export const lookupSelfIpAddress = async (): Promise<IpLookupResult> => {
    // Get API key from environment variables
    const apiKey = process.env.NEXT_PUBLIC_API_KEY || '';
    
    // Set headers with just the API key
    const headers: Record<string, string> = {
        'x-api-key': apiKey,
        // Let the backend handle IP extraction
    };
    
    try {
        console.log('[IP Lookup] Fetching IP information from API...');
        const response = await api<IpLookupResult>('/lookup/self', {
            method: 'GET',
            headers,
        });
        console.log('[IP Lookup] Successfully retrieved IP information:', response);
        return response;
    } catch (error) {
        console.error('[IP Lookup] API request failed:', error);
        throw error;
    }
};

/**
 * Look up information for a specific IP address
 * @param ip The IP address to look up
 * @returns Promise with IP lookup results
 */
export const lookupIpAddress = async (ip: string): Promise<IpLookupResult> => {
    // Get API key from environment variables
    const apiKey = process.env.NEXT_PUBLIC_API_KEY || '';
    
    // Set headers with just the API key
    const headers: Record<string, string> = {
        'x-api-key': apiKey,
    };
    
    try {
        console.log(`[IP Lookup] Fetching information for IP: ${ip}`);
        const response = await api<IpLookupResult>(`/lookup/${ip}`, {
            method: 'GET',
            headers,
        });
        console.log(`[IP Lookup] Successfully retrieved information for IP ${ip}:`, response);
        return response;
    } catch (error) {
        console.error(`[IP Lookup] API request failed for IP ${ip}:`, error);
        throw error;
    }
};
