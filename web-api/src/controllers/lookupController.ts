import { Request, Response } from 'express';
import { isIP } from 'net';
import { IpLookupResult, LookupResponse } from '../types/ipLookup';
import { BadRequest } from '../errors/BadRequest';
import { Unauthorized } from '../errors/Unauthorized';
import { rustService } from '../services/rustService';

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
            cpu: clientInfo.cpu,
            engine: clientInfo.engine,
            timestamp: new Date().toISOString()
        } : undefined
    };
}

export const lookupIpAddress = async (req: Request, res: Response) => {
    // Check if this is a specific IP lookup (e.g., /lookup/8.8.8.8)
    const isSpecificIpLookup = req.params.ip && req.params.ip !== 'self';
    
    try {
        console.log(`[lookupIpAddress] Starting lookup for ${isSpecificIpLookup ? 'specific IP' : 'self'}`);
        
        const apiKey = req.header('x-api-key')!; // Safe to assert non-null as middleware ensures it exists
        
        let result: LookupResponse;
        
        if (isSpecificIpLookup) {
            // For /lookup/{ip}, use the provided IP address for the lookup
            const ipToLookup = req.params.ip;
            if (!ipToLookup || !isIP(ipToLookup)) {
                throw new BadRequest('Invalid IP address format');
            }
            console.log(`[lookupIpAddress] Looking up specific IP: ${ipToLookup}`);
            
            // Call the rust-service with the specific IP
            result = await rustService.lookupIp(apiKey, ipToLookup);
        } else {
            // For /lookup/self, use the client's IP address
            if (!req.clientIp) {
                throw new BadRequest('Could not determine client IP address');
            }
            
            const clientIp = req.clientIp;
            console.log(`[lookupIpAddress] Looking up client IP: ${clientIp}`);
            
            // Call the rust-service with the client's IP
            result = await rustService.lookupSelf(apiKey, clientIp, clientIp);
        }
        
        console.log(`[lookupIpAddress] Successfully retrieved data for IP`);
        
        // Transform the response to match the frontend's expected format
        const transformedResult = transformLookupResponse(result, req.clientInfo);
        
        res.json(transformedResult);
    } catch (error) {
        console.error(`[lookupIpAddress] Error during IP lookup:`, error);
        throw error; // Let the global error handler handle it
    }
};