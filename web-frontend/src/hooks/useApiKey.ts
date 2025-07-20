import { useState, useCallback } from 'react';
import { api } from '@src/utils/api';

interface ApiKeyResponse {
  apiKey: string;
  createdAt: string;
}

export function useApiKey() {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [apiKey, setApiKey] = useState<string | null>(() => {
    // Initialize from localStorage if available
    if (typeof window !== 'undefined') {
      return localStorage.getItem('apiKey');
    }
    return null;
  });

  // Generate a new API key
  const generateApiKey = useCallback(async (): Promise<string | null> => {
    setIsLoading(true);
    setError(null);
    
    try {
      const data = await api<ApiKeyResponse>('/users/apikey', {
        method: 'POST',
      });
      
      setApiKey(data.apiKey);
      
      // Store in localStorage for persistence
      if (typeof window !== 'undefined') {
        localStorage.setItem('apiKey', data.apiKey);
      }
      
      return data.apiKey;
    } catch (err) {
      console.error('Failed to generate API key:', err);
      setError('Failed to generate API key. Please try again.');
      return null;
    } finally {
      setIsLoading(false);
    }
  }, []);

  // Get the current API key
  const getApiKey = useCallback(async (): Promise<string | null> => {
    if (apiKey) return apiKey;
    
    setIsLoading(true);
    setError(null);
    
    try {
      const data = await api<ApiKeyResponse>('/users/apikey');
      
      if (data.apiKey) {
        setApiKey(data.apiKey);
        
        // Store in localStorage for persistence
        if (typeof window !== 'undefined') {
          localStorage.setItem('apiKey', data.apiKey);
        }
      }
      
      return data.apiKey || null;
    } catch (err) {
      console.error('Failed to fetch API key:', err);
      setError('Failed to fetch API key. Please try again.');
      return null;
    } finally {
      setIsLoading(false);
    }
  }, [apiKey]);

  // Clear the API key from state and localStorage
  const clearApiKey = useCallback(() => {
    setApiKey(null);
    if (typeof window !== 'undefined') {
      localStorage.removeItem('apiKey');
    }
  }, []);

  return {
    apiKey,
    isLoading,
    error,
    generateApiKey,
    getApiKey,
    clearApiKey,
  };
}
