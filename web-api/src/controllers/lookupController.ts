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
    return {
        ip: response.ip,
        country: response.geo_info?.country,
        city: response.geo_info?.city,
        isp: response.asn_info?.organization,
        isVpn: response.is_vpn_or_datacenter,
        isProxy: response.is_proxy,
        isTor: response.is_tor_exit_node,
        threatScore: response.threat_score,
        recommendedAction: response.recommended_action,
        latitude: response.geo_info?.latitude,
        longitude: response.geo_info?.longitude,
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
            console.log(transformedResult)
            
            // Return the transformed result to the client
            res.json(transformedResult);
        } catch (error) {
            console.error('Error calling rust-service:', error);
            // For demo purposes, return a mock response if the rust-service fails
            const mockResponse: IpLookupResult = {
                ip: clientIp,
                country: 'Node.js Error',
                city: 'Node.js Error',
                isp: 'Node.js Error',
                isVpn: false,
                isProxy: false,
                isTor: false,
                threatScore: 0,
                recommendedAction: 'Node.js Error',
                latitude: 0,
                longitude: 0
            };
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