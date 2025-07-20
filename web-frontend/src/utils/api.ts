import { API_BASE_URL } from '@src/constants/api';

interface RequestOptions extends RequestInit {
  token?: string | null;
  isFormData?: boolean;
}

export async function api<T>(
  endpoint: string,
  { token, isFormData, headers, ...customConfig }: RequestOptions = {}
): Promise<T> {
  const config: RequestInit = {
    method: customConfig.body ? 'POST' : 'GET',
    ...customConfig,
    headers: {
      ...(!isFormData && { 'Content-Type': 'application/json' }),
      ...(token && { Authorization: `Bearer ${token}` }),
      ...headers,
    } as HeadersInit,
    credentials: 'include', // Important for cookies
  };

  const response = await fetch(`${API_BASE_URL}${endpoint}`, config);

  // Handle 204 No Content responses
  if (response.status === 204) {
    return null as unknown as T;
  }

  const data = await response.json().catch(() => ({}));

  if (!response.ok) {
    const error = new Error(response.statusText);
    Object.assign(error, { response, data });
    throw error;
  }

  return data as T;
}

// Helper function to handle API errors consistently
export function handleApiError(error: unknown): string {
  if (typeof error === 'string') return error;
  
  const err = error as Error & { response?: Response; data?: any };
  
  if (err.response) {
    // Handle HTTP errors
    if (err.response.status === 401) {
      return 'Your session has expired. Please log in again.';
    }
    
    if (err.response.status === 403) {
      return 'You do not have permission to perform this action.';
    }
    
    if (err.data?.message) {
      return err.data.message;
    }
  }
  
  return 'An unexpected error occurred. Please try again.';
}
