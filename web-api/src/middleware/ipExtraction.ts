import { Request, Response, NextFunction } from 'express';
import { isIP } from 'net';
import { BadRequest } from '../errors/BadRequest';

// Extend the Express Request type to include clientIp
declare global {
    namespace Express {
        interface Request {
            clientIp?: string;
        }
    }
}

// List of IPs to treat as localhost (for development)
const LOCAL_IPS = new Set([
    '::1',         // IPv6 localhost
    '127.0.0.1',   // IPv4 localhost
    '::ffff:127.0.0.1' // IPv4-mapped IPv6 localhost
]);

// Default demo IP to use in development
const DEMO_IP = '8.8.8.8';

/**
 * Extracts and validates the client IP address from the request.
 * Sets req.clientIp with the extracted IP or DEMO_IP in development.
 */
export const extractClientIp = (req: Request, res: Response, next: NextFunction) => {
    console.log(`[IP Extraction] Starting IP extraction (NODE_ENV=${process.env.NODE_ENV})`);
    
    // Log all request headers for debugging (be careful with this in production)
    if (process.env.NODE_ENV !== 'production') {
        console.log('[IP Extraction] Request headers:', JSON.stringify(req.headers, null, 2));
    }
    
    let ip: string | undefined;
    
    // Check X-Forwarded-For header (common when behind a proxy)
    const forwardedIps = req.headers['x-forwarded-for']?.toString();
    if (forwardedIps) {
        const ips = forwardedIps.split(',').map(ip => ip.trim());
        ip = ips[0];
        console.log(`[IP Extraction] Found IP in x-forwarded-for: ${ip} (full header: ${forwardedIps})`);
    }
    
    // Fall back to X-Real-IP header
    if (!ip) {
        ip = req.headers['x-real-ip']?.toString();
        if (ip) {
            console.log(`[IP Extraction] Found IP in x-real-ip: ${ip}`);
        }
    }
    
    // Fall back to connection remote address
    if (!ip) {
        ip = req.socket.remoteAddress;
        console.log(`[IP Extraction] Using socket.remoteAddress: ${ip}`);
    }
    
    // In development, use demo IP for localhost/loopback
    if (process.env.NODE_ENV !== 'production' && ip && (LOCAL_IPS.has(ip) || isPrivateIp(ip))) {
        console.log(`[IP Extraction] Development mode: Using demo IP (${DEMO_IP}) instead of ${ip}`);
        ip = DEMO_IP;
    }
    
    // Validate IP format
    if (ip && isIP(ip)) {
        req.clientIp = ip;
        const source = req.headers['x-forwarded-for'] ? 'x-forwarded-for' : 
                     req.headers['x-real-ip'] ? 'x-real-ip' : 'socket.remoteAddress';
        console.log(`[IP Extraction] Using IP: ${ip} (source: ${source})`);
        return next();
    }
    
    // In production, reject invalid IPs
    if (process.env.NODE_ENV === 'production') {
        const errorMsg = `Invalid IP address: ${ip}. Headers: ${JSON.stringify({
            'x-forwarded-for': req.headers['x-forwarded-for'],
            'x-real-ip': req.headers['x-real-ip'],
            'socket.remoteAddress': req.socket.remoteAddress
        })}`;
        console.error(`[IP Extraction] ${errorMsg}`);
        return next(new BadRequest('Could not determine a valid client IP address'));
    }
    
    // In development, use demo IP as fallback
    console.log(`[IP Extraction] No valid IP found, using demo IP (${DEMO_IP}) in development`);
    req.clientIp = DEMO_IP;
    next();
};

/**
 * Checks if an IP address is in a private range
 */
function isPrivateIp(ip: string): boolean {
    if (!isIP(ip)) return false;
    
    // Handle IPv4 and IPv6-mapped IPv4
    let ipStr = ip;
    if (ip.startsWith('::ffff:')) {
        ipStr = ip.substring(7); // Extract IPv4 part
    }
    
    // Check for IPv4 private ranges
    if (isIP(ipStr) === 4) {
        const octets = ipStr.split('.').map(Number);
        
        // Private IP ranges:
        // 10.0.0.0/8
        // 172.16.0.0/12
        // 192.168.0.0/16
        // 169.254.0.0/16 (link-local)
        return (
            octets[0] === 10 ||
            (octets[0] === 172 && octets[1] >= 16 && octets[1] <= 31) ||
            (octets[0] === 192 && octets[1] === 168) ||
            (octets[0] === 169 && octets[1] === 254)
        );
    }
    
    // For IPv6, consider all private/loopback addresses as private
    // This includes ::1 (handled by LOCAL_IPS) and link-local addresses
    return ip.startsWith('fe80::') || ip.startsWith('fc00::');
}
