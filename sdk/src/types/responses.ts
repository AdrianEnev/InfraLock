// Export an empty object to make this a module
export {};

// ASN (Autonomous System Number) information
export interface AsnInfo {
  autonomous_system_number?: number;
  autonomous_system_organization?: string;
}

// Client browser information
export interface ClientBrowserInfo {
  name: string;
  version: string;
}

// Client OS information
export interface ClientOsInfo {
  name: string;
  version: string;
}

// Client device information
export interface ClientDeviceInfo {
  model: string;
  type: string;
}

// Detailed client information
export interface ClientInfo {
  userAgent: string;
  browser: ClientBrowserInfo;
  os: ClientOsInfo;
  device: ClientDeviceInfo;
  engine: string;
  cpu: string;
  timestamp: string;
}

// Main lookup response interface
export interface LookupResponse {
  // IP information
  ip: string;
  country?: string;
  city?: string;
  
  // ASN information
  asnInfo?: AsnInfo;
  
  // Threat information
  isVpn: boolean;
  isProxy: boolean;
  isTor: boolean;
  threatScore: number;
  threatDetails: string[];
  recommendedAction: string;
  
  // Location information
  latitude?: number;
  longitude?: number;
  
  // Proxy information
  proxyType?: string | null;
  
  // Client information (if available)
  clientInfo?: ClientInfo;
  
  // Legacy field for backward compatibility (maps to ASN organization)
  isp?: string;
}

export interface ErrorResponse {
  statusCode: number;
  message: string;
  code?: string;
}