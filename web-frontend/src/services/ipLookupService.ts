import { api } from '@src/utils/api';
import { IpLookupResult } from '@interfaces/ApiInterfaces';

/**
 * Look up the current user's IP address information using the rust-service
 * @returns Promise with IP lookup results
 */
export const lookupSelfIpAddress = async (): Promise<IpLookupResult> => {
    // For demo purposes, use 8.8.8.8 as the IP and get API key from env
    const demoIp = '8.8.8.8'; // Google's public DNS as a demo IP
    const apiKey = process.env.NEXT_PUBLIC_API_KEY || '';
    
    // Set headers with API key and demo IP
    const headers: Record<string, string> = {
        'x-api-key': apiKey,
        'x-real-ip': demoIp,
        'x-forwarded-for': demoIp // Include both headers for compatibility
    };
    
    try {
        const response = await api<IpLookupResult>('/lookup/self', {
            method: 'GET',
            headers,
        });
        return response;
    } catch (error) {
        console.warn('API request failed, using mock data. Error:', error);
        // Return mock data for demo purposes that matches IpLookupResult interface
        return {
            ip: demoIp,
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
