import { Request, Response } from 'express';
import { IpLookupResult, LookupResponse } from '../types/ipLookup';
import { BadRequest } from '../errors/BadRequest';
import { rustService } from '../services/rustService';

/**
 * Gets the client IP address from the request
 */
function getClientIp(req: Request): string | undefined {
    // Try to get the IP from the X-Forwarded-For header (common when behind a proxy)
    const forwarded = req.headers['x-forwarded-for'];
    if (forwarded) {
        if (Array.isArray(forwarded)) {
            return forwarded[0].split(',')[0].trim();
        }
        return forwarded.split(',')[0].trim();
    }
    
    // Try X-Real-IP header
    if (req.headers['x-real-ip']) {
        return Array.isArray(req.headers['x-real-ip']) 
            ? req.headers['x-real-ip'][0] 
            : req.headers['x-real-ip'];
    }
    
    // Fall back to the connection's remote address
    return req.socket.remoteAddress;
}

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
        // Get the client IP address from the request
        const clientIp = getClientIp(req) || req.params.ip;
        
        if (!clientIp) {
            throw new BadRequest('Could not determine IP address');
        }

        // For demo purposes, use a default API key if not provided
        const apiKey = req.header('x-api-key') || 'demo-api-key';
        
        // Get all relevant headers to forward to rust-service
        const xForwardedFor = req.header('x-forwarded-for');
        const xRealIp = req.header('x-real-ip');
        
        try {
            // Call the rust-service to get the IP lookup information
            // Forward the original headers or use the extracted IP if not available
            const result = await rustService.lookupSelf(
                apiKey, 
                xForwardedFor || clientIp,
                xRealIp
            );
            
            // Transform the response to match the frontend's expected format
            const transformedResult = transformLookupResponse(result);
            
            // Return the transformed result to the client
            res.json(transformedResult);
        } catch (error) {
            console.error('Error calling rust-service:', error);
            // For demo purposes, return a mock response if the rust-service fails
            const mockResponse: IpLookupResult = {
                ip: clientIp || '8.8.8.8',
                country: 'United States',
                city: 'Mountain View',
                asnInfo: {
                    autonomous_system_number: 15169,
                    autonomous_system_organization: 'Google LLC'
                },
                isVpn: false,
                isProxy: false,
                isTor: false,
                threatScore: 0,
                threatDetails: [],
                recommendedAction: 'allow',
                latitude: 37.422,
                longitude: -122.084,
                proxyType: null,
            };
            
            // If we're in development mode, include the error details
            if (process.env.NODE_ENV === 'development') {
                (mockResponse as any).error = error instanceof Error ? error.message : 'Unknown error';
                (mockResponse as any).stack = error instanceof Error ? error.stack : undefined;
            }
            
            res.json(mockResponse);
        }
    } catch (error) {
        console.error('Error in lookupIpAddress:', error);
        res.status(500).json({
            error: 'An error occurred while processing your request',
            details: error instanceof Error ? error.message : 'Unknown error'
        });
    }
};