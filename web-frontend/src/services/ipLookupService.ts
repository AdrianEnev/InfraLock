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
        // Return mock data for demo purposes
        return {
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
    }
};  
