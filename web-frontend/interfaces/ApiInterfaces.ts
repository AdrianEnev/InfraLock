/**
 * Represents a country with optional localized names.
 * - `names`: An optional object containing the country name
 *   in different languages, where the key is the language code
 *   and the value is the country name in that language.
 *   - `en`: Represents the country name in English.
 *   - Other language codes can be added as needed.
 */
export interface Country {
    names?: {
        en?: string;
        // Add other language codes as needed
        [key: string]: string | undefined;
    };
}

export interface Location {
    latitude?: number;
    longitude?: number;
}

export interface GeoInfo {
    city?: string;
    country?: Country;
    location?: Location;
}

export interface AsnInfo {
    autonomous_system_number?: number;
    autonomous_system_organization?: string;
}

export interface LookupResponse {
    ip: string;
    geo_info?: GeoInfo;
    asn_info?: AsnInfo;
    is_vpn_or_datacenter: boolean;
    is_proxy: boolean;
    proxy_type?: string | null;
    is_tor_exit_node: boolean;
    threat_score: number;
    threat_details: string[];
    recommended_action: string;
}

// For backward compatibility with the frontend
export interface IpLookupResult extends Omit<LookupResponse, 'is_vpn_or_datacenter' | 'is_proxy' | 'is_tor_exit_node' | 'threat_score' | 'recommended_action' | 'geo_info' | 'asn_info' | 'proxy_type' | 'threat_details'> {
    country?: string;
    city?: string;
    asnInfo?: {
        autonomous_system_number?: number;
        autonomous_system_organization?: string;
    };
    isVpn: boolean;
    isProxy: boolean;
    isTor: boolean;
    threatScore: number;
    threatDetails: string[];
    recommendedAction: string;
    latitude?: number;
    longitude?: number;
    proxyType?: string | null;
}
