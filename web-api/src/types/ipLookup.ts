export interface LookupResponse {
    ip: string;
    geo_info?: {
        country?: string;
        city?: string;
        latitude?: number;
        longitude?: number;
    };
    asn_info?: {
        organization?: string;
    };
    is_vpn_or_datacenter: boolean;
    is_proxy: boolean;
    is_tor_exit_node: boolean;
    threat_score: number;
    recommended_action: string;
}

// For backward compatibility with the frontend
export interface IpLookupResult extends Omit<LookupResponse, 'is_vpn_or_datacenter' | 'is_proxy' | 'is_tor_exit_node' | 'threat_score' | 'recommended_action' | 'geo_info' | 'asn_info'> {
    country?: string;
    city?: string;
    isp?: string;
    isVpn: boolean;
    isProxy: boolean;
    isTor: boolean;
    threatScore: number;
    recommendedAction: string;
    latitude?: number;
    longitude?: number;
}
