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
        
        // In production, rethrow the error to be handled by the error boundary
        if (process.env.NODE_ENV === 'production') {
            console.error('[IP Lookup] Running in production, propagating error to error boundary');
            throw error;
        }
        
        // For development, return mock data
        const mockIp = '8.8.8.8';
        console.warn(`[IP Lookup] Development mode: Using mock data with IP ${mockIp}`);
        return {
            ip: mockIp, // Default demo IP
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
            threatDetails: ['IP is associated with a VPN or data center'],
            recommendedAction: 'redirect',
            latitude: 37.422,
            longitude: -122.084,
            proxyType: null
        };
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
        
        // In production, rethrow the error to be handled by the error boundary
        if (process.env.NODE_ENV === 'production') {
            console.error('[IP Lookup] Running in production, propagating error to error boundary');
            throw error;
        }
        
        // For development, return mock data
        console.warn(`[IP Lookup] Development mode: Using mock data for IP ${ip}`);
        return {
            ip: ip,
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
            threatDetails: ['IP is associated with a VPN or data center'],
            recommendedAction: 'monitor',
            latitude: 37.422,
            longitude: -122.084,
            proxyType: null
        };
    }
};
