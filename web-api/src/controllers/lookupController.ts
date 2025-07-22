import { Request, Response } from 'express';
import { IpLookupResult, LookupResponse } from '../types/ipLookup';
import { BadRequest } from '../errors/BadRequest';
import { rustService } from '../services/rustService';

// Default demo IP to use in development
const DEMO_IP = '8.8.8.8';

/**
 * Transforms the rust-service's response to match the frontend's expected format
 */
function transformLookupResponse(response: LookupResponse): IpLookupResult {
    // Extract nested values with proper null/undefined checking
    const countryName = response.geo_info?.country?.names?.en;
    const city = response.geo_info?.city;
    const latitude = response.geo_info?.location?.latitude;
    const longitude = response.geo_info?.location?.longitude;
    
    return {
        ip: response.ip,
        country: countryName,
        city: city,
        asnInfo: response.asn_info ? {
            autonomous_system_number: response.asn_info.autonomous_system_number,
            autonomous_system_organization: response.asn_info.autonomous_system_organization
        } : undefined,
        isVpn: response.is_vpn_or_datacenter,
        isProxy: response.is_proxy,
        isTor: response.is_tor_exit_node,
        threatScore: response.threat_score,
        threatDetails: response.threat_details || [],
        recommendedAction: response.recommended_action,
        latitude: latitude,
        longitude: longitude,
        proxyType: response.proxy_type
    };
}

export const lookupIpAddress = async (req: Request, res: Response) => {
    try {
        // Get the client IP from the request (set by our middleware)
        const clientIp = req.clientIp || DEMO_IP;
        
        // For demo purposes, use a default API key if not provided
        const apiKey = req.header('x-api-key') || 'demo-api-key';
        
        try {
            // Call the rust-service to get the IP lookup information
            const result = await rustService.lookupSelf(
                apiKey, 
                clientIp, // Use the IP from the request
                clientIp  // Same for both headers for compatibility
            );
            
            // Transform the response to match the frontend's expected format
            const transformedResult = transformLookupResponse(result);
            
            // Return the transformed result to the client
            res.json(transformedResult);
        } catch (error) {
            console.error('Error calling rust-service:', error);
            
            // In production, rethrow the error to be handled by the global error handler
            if (process.env.NODE_ENV === 'production') {
                throw error;
            }
            
            // For demo purposes, return a mock response if the rust-service fails
            const mockResponse: IpLookupResult = {
                ip: clientIp,
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
            
            res.json(mockResponse);
        }
    } catch (error) {
        console.error('Error in lookupIpAddress:', error);
        
        // If we have a custom error with status code, use it
        if ('statusCode' in (error as any)) {
            const statusCode = (error as any).statusCode || 500;
            return res.status(statusCode).json({
                error: (error as Error).message || 'An error occurred',
                details: process.env.NODE_ENV !== 'production' ? (error as Error).stack : undefined
            });
        }
        
        // Default error response
        res.status(500).json({
            error: 'Internal server error',
            details: process.env.NODE_ENV !== 'production' ? (error as Error).stack : undefined
        });
    }
};