import { UAParser } from 'ua-parser-js';

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

export {};
