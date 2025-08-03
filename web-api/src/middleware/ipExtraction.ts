import { Request, Response, NextFunction } from 'express';
import { isIP } from 'net';
import { BadRequest } from '../errors/BadRequest';
import { UAParser } from 'ua-parser-js';

// Extend the Express Request type to include clientIp
declare global {
    namespace Express {
        interface Request {
            clientIp?: string;
            clientInfo?: {
                userAgent: string;
                browser: string;
                browserVersion: string;
                os: string;
                osVersion: string;
                device: string;
                deviceType: string;
                cpu: string;
                engine: string;
            };
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
const DEMO_IP = '85.14.44.10';

/**
 * Extracts and validates the client IP address from the request.
 * Also parses user agent information.
 * Sets req.clientIp and req.clientInfo with the extracted data.
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
    
    // In development, for the /lookup/self route, use demo IP for localhost/loopback
    if (req.path.endsWith('/self') && process.env.NODE_ENV !== 'production' && ip && (LOCAL_IPS.has(ip) || isPrivateIp(ip))) {
        console.log(`[IP Extraction] Development mode: Using demo IP (${DEMO_IP}) instead of ${ip} for /lookup/self`);
        ip = DEMO_IP;
    }
    
    // Validate the IP address
    if (ip && !isIP(ip)) {
        console.error(`[IP Extraction] Invalid IP address: ${ip}`);
        return next(new BadRequest('Invalid IP address'));
    }

    // Set the client IP on the request object
    req.clientIp = ip || DEMO_IP;

    // Parse user agent information
    const userAgent = req.headers['user-agent'] || 'unknown';
    const { browser, os, device, cpu } = UAParser(userAgent);

    req.clientInfo = {
        userAgent,
        browser: browser.name || 'unknown',
        browserVersion: browser.version || 'unknown',
        os: os.name || 'unknown',
        osVersion: os.version || 'unknown',
        device: device.model || 'unknown',
        deviceType: device.type || 'desktop',
        cpu: cpu.architecture || 'unknown',
        engine: 'unknown'
    };

    console.log('[IP Extraction] Client info:', JSON.stringify(req.clientInfo, null, 2));
    
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
