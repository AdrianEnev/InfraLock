import { Request, Response } from 'express';
import { IpLookupResult, LookupResponse } from '../types/ipLookup';
import { BadRequest } from '../errors/BadRequest';
import { rustService } from '../services/rustService';

// Default demo IP to use in development
const DEMO_IP = '85.14.44.10';

/**
 * Transforms the rust-service's response to match the frontend's expected format
 */
function transformLookupResponse(response: LookupResponse, clientInfo?: any): IpLookupResult {
    // Extract nested values with proper null/undefined checking
    const countryName = response.geo_info?.country?.names?.en;
    const cityName = response.geo_info?.city?.names?.en;
    const latitude = response.geo_info?.location?.latitude;
    const longitude = response.geo_info?.location?.longitude;
    
    return {
        // IP information
        ip: response.ip,
        country: countryName,
        city: cityName,
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
        proxyType: response.proxy_type,
        
        // Client information if available
        clientInfo: clientInfo ? {
            userAgent: clientInfo.userAgent,
            browser: {
                name: clientInfo.browser,
                version: clientInfo.browserVersion,
            },
            os: {
                name: clientInfo.os,
                version: clientInfo.osVersion,
            },
            device: {
                model: clientInfo.device,
                type: clientInfo.deviceType,
            },
            engine: clientInfo.engine,
            cpu: clientInfo.cpu,
            timestamp: new Date().toISOString(),
        } : undefined
    };
}

export const lookupIpAddress = async (req: Request, res: Response) => {
    try {
        // Get the API key from the request headers
        const apiKey = req.header('x-api-key') || 'demo-api-key';
        
        // Check if this is a specific IP lookup (e.g., /lookup/8.8.8.8)
        const isSpecificIpLookup = req.params.ip && req.params.ip !== 'self';
        
        try {
            let result: any;
            
            if (isSpecificIpLookup) {
                // For /lookup/{ip}, use the provided IP address for the lookup
                const ipToLookup = req.params.ip;
                console.log(`[Lookup] Looking up specific IP: ${ipToLookup}`);
                
                // Call the rust-service with the specific IP
                result = await rustService.lookupIp(apiKey, ipToLookup);
            } else {
                // For /lookup/self, use the client's IP address
                const clientIp = req.clientIp || DEMO_IP;
                console.log(`[Lookup] Looking up client IP: ${clientIp}`);
                
                // Call the rust-service with the client's IP
                result = await rustService.lookupSelf(
                    apiKey, 
                    clientIp, // X-Forwarded-For
                    clientIp  // X-Real-IP
                );
            }
            
            // Transform the response to match the frontend's expected format
            const transformedResult = transformLookupResponse(result, req.clientInfo);
            
            // Return the transformed result to the client
            res.json(transformedResult);
        } catch (error) {
            console.error('Error calling rust-service:', error);
            
            // In production, rethrow the error to be handled by the global error handler
            if (process.env.NODE_ENV === 'production') {
                throw error;
            }
            
            // For demo purposes, return a mock response if the rust-service fails
            const isSelfLookup = req.path === '/api/lookup/self' || req.path === '/api/lookup';
            const ipToUse = isSelfLookup ? DEMO_IP : (req.params.ip || DEMO_IP);
            
            const mockResponse: IpLookupResult = {
                ip: ipToUse,
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
                recommendedAction: isSelfLookup ? 'monitor' : 'allow',
                latitude: 37.422,
                longitude: -122.084,
                proxyType: null,
                clientInfo: req.clientInfo ? {
                    userAgent: req.clientInfo.userAgent,
                    browser: {
                        name: req.clientInfo.browser,
                        version: req.clientInfo.browserVersion,
                    },
                    os: {
                        name: req.clientInfo.os,
                        version: req.clientInfo.osVersion,
                    },
                    device: {
                        model: req.clientInfo.device,
                        type: req.clientInfo.deviceType,
                    },
                    engine: req.clientInfo.engine,
                    cpu: req.clientInfo.cpu,
                    timestamp: new Date().toISOString(),
                } : undefined
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
            details: process.env.NODE_ENV !== 'production' ? (error as Error).message : undefined
        });
    }
};