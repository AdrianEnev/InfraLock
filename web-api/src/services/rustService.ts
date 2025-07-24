import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios';
import { LookupResponse } from '../types/ipLookup';

export class RustServiceClient {
  private client: AxiosInstance;
  private baseUrl: string;

  constructor() {
    // Use environment variable or default to the local development URL
    this.baseUrl = process.env.RUST_SERVICE_URL!;
    
    this.client = axios.create({
      baseURL: this.baseUrl,
      timeout: 10000, // 10 seconds
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
    });

    // Add request interceptor for logging
    this.client.interceptors.request.use(
      (config) => {
        console.log(`[RustService] ${config.method?.toUpperCase()} ${config.url}`, {
          headers: config.headers,
          data: config.data,
          params: config.params,
        });
        return config;
      },
      (error) => {
        console.error('[RustService] Request Error:', error);
        return Promise.reject(error);
      }
    );

    // Add response interceptor for error handling
    this.client.interceptors.response.use(
      (response) => response,
      (error) => {
        console.error('[RustService] Response Error:', {
          status: error.response?.status,
          statusText: error.response?.statusText,
          data: error.response?.data,
        });
        return Promise.reject(error);
      }
    );
  }

  /**
   * Look up IP address information from the rust-service
   * @param apiKey The API key for authentication
   * @param xForwardedFor The X-Forwarded-For header value (if behind a proxy)
   * @param xRealIp The X-Real-IP header value (alternative to X-Forwarded-For)
   * @returns Promise with IP lookup results
   */
  async lookupSelf(
    apiKey: string, 
    xForwardedFor?: string, 
    xRealIp?: string
  ): Promise<LookupResponse> {
    const config: AxiosRequestConfig = {
      headers: {
        'X-API-Key': apiKey,
      },
    };

    // Add X-Forwarded-For header if provided
    if (xForwardedFor) {
      config.headers = {
        ...config.headers,
        'X-Forwarded-For': xForwardedFor
      };
    }

    // Add X-Real-IP header if provided
    if (xRealIp) {
      config.headers = {
        ...config.headers,
        'X-Real-IP': xRealIp
      };
    }

    const response: AxiosResponse<LookupResponse> = await this.client.get(
      '/api/lookup/self',
      config
    );

    return response.data;
  }

  /**
   * Look up a specific IP address from the rust-service
   * @param apiKey The API key for authentication
   * @param ip The IP address to look up
   * @returns Promise with IP lookup results
   */
  async lookupIp(
    apiKey: string, 
    ip: string
  ): Promise<LookupResponse> {
    const config: AxiosRequestConfig = {
      headers: {
        'X-API-Key': apiKey,
      },
    };

    const response: AxiosResponse<LookupResponse> = await this.client.get(
      `/api/lookup/${ip}`,
      config
    );

    return response.data;
  }
}

// Export a singleton instance
export const rustService = new RustServiceClient();
