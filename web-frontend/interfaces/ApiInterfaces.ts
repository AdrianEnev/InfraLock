export interface IpLookupResult {
    ip: string;
    country?: {
        names?: {
            [key: string]: string;  // e.g., { en: "United States", es: "Estados Unidos" }
        }
    };
    city?: string;
    isp?: string;
    isVpn?: boolean;
    isProxy?: boolean;
    isTor?: boolean;
    threatScore?: number;
    recommendedAction?: string;
    latitude?: number;
    longitude?: number;
}