import axios, { AxiosError, AxiosInstance } from 'axios';
import * as https from 'https';
import * as http from 'http';

// Base URL for the API - hardcoded to match the running server
const API_BASE_URL = 'http://localhost:3000/api';
console.log('Using API base URL:', API_BASE_URL);

// Create a cookie jar to manually handle cookies
const cookieJar: Record<string, string> = {};

// Create an axios instance with cookie handling
const axiosInstance = axios.create({
  baseURL: API_BASE_URL,
  withCredentials: true, // Important for cookies
  httpAgent: new http.Agent({ keepAlive: true }),
  httpsAgent: new https.Agent({ keepAlive: true }),
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add request interceptor to include cookies in requests
axiosInstance.interceptors.request.use((config) => {
  // Add cookies to the request if they exist
  const cookies = Object.entries(cookieJar)
    .map(([key, value]) => `${key}=${value}`)
    .join('; ');
  
  if (cookies) {
    config.headers.Cookie = cookies;
  }
  
  console.log('Sending request to:', config.url);
  console.log('With headers:', JSON.stringify(config.headers, null, 2));
  
  return config;
});

// Add response interceptor to capture cookies from responses
axiosInstance.interceptors.response.use((response) => {
  // Extract cookies from response headers
  const cookies = response.headers['set-cookie'];
  if (cookies) {
    cookies.forEach((cookie: string) => {
      const [cookieKeyValue] = cookie.split(';');
      const [key, value] = cookieKeyValue.split('=');
      cookieJar[key.trim()] = value;
    });
    console.log('Updated cookies:', cookieJar);
  }
  return response;
}, (error) => {
  console.error('Response error:', error.message);
  if (error.response) {
    console.error('Response status:', error.response.status);
    console.error('Response headers:', error.response.headers);
    console.error('Response data:', error.response.data);
  }
  return Promise.reject(error);
});

interface TestUser {
  email: string;
  password: string;
  apiKey?: string;
}

const testUser: TestUser & { username: string } = {
  username: 'testuser',
  email: 'test@example.com',
  password: 'password123',
};

interface ApiResponse<T = any> {
  data: T | null;
  error: any;
  status: number;
  headers?: any;
}

// Helper function to make authenticated requests
const makeRequest = async <T = any>(endpoint: string, options: any = {}): Promise<ApiResponse<T>> => {
  try {
    // Remove leading slash from endpoint if present
    const normalizedEndpoint = endpoint.startsWith('/') ? endpoint.slice(1) : endpoint;
    console.log('Making request to:', `${API_BASE_URL}/${normalizedEndpoint}`);
    
    const response = await axiosInstance({
      url: normalizedEndpoint,
      method: options.method || 'GET',
      data: options.data,
      headers: options.headers,
      // withCredentials is already set in axiosInstance
    });
    
    return { 
      data: response.data, 
      error: null, 
      status: response.status, 
      headers: response.headers 
    };
  } catch (error: any) {
    if (error.response) {
      // The request was made and the server responded with a status code
      // that falls out of the range of 2xx
      console.error('Request failed with status:', error.response.status);
      console.error('Response data:', error.response.data);
      return { 
        data: null, 
        error: error.response.data || `Request failed with status ${error.response.status}`,
        status: error.response.status,
        headers: error.response.headers
      };
    } else if (error.request) {
      // The request was made but no response was received
      console.error('No response received:', error.request);
      return { 
        data: null, 
        error: 'No response received from server',
        status: 0,
        headers: {}
      };
    } else {
      // Something happened in setting up the request
      console.error('Request setup error:', error.message);
      return { 
        data: null, 
        error: error.message,
        status: 0,
        headers: {}
      };
    }
  }
};

// Test registration and handle existing users
const testRegistration = async () => {
  console.log('Testing user registration...');
  
  // First try to register the user
  const { data, error } = await makeRequest('users/register', {
    method: 'POST',
    data: {
      username: testUser.username,
      email: testUser.email,
      password: testUser.password,
    },
  });

  if (error) {
    if (error.detail === 'Username or email already exists.') {
      console.log('User already exists, attempting to log in...');
      // Try to log in with existing credentials
      const loginResponse = await makeRequest('users/login', {
        method: 'POST',
        data: {
          email: testUser.email,
          password: testUser.password,
        },
      });

      if (loginResponse.error) {
        console.error('Login with existing user failed:', loginResponse.error);
        return false;
      }

      console.log('Successfully logged in with existing user');
      if (loginResponse.data) {
        testUser.apiKey = loginResponse.data.apiKey;
      }
      return true;
    }
    
    console.error('Registration failed:', error);
    return false;
  }

  console.log('Registration successful:', data);
  if (data && data.apiKey) {
    testUser.apiKey = data.apiKey;
  }
  return true;
};

// Test login
const testLogin = async () => {
  console.log('Testing login...');
  
  // Clear any existing cookies first
  Object.keys(cookieJar).forEach(key => delete cookieJar[key]);
  
  const { data, error, status, headers } = await makeRequest('users/login', {
    method: 'POST',
    data: {
      email: testUser.email,
      password: testUser.password,
    },
  });

  if (error) {
    console.error('Login failed:', error);
    console.error('Status code:', status);
    if (headers) {
      console.log('Response headers:', JSON.stringify(headers, null, 2));
    }
    return false;
  }

  console.log('Login successful. User data:', data);
  console.log('Current cookies after login:', cookieJar);
  return true;
};

// Test getting current user
const testGetCurrentUser = async () => {
  console.log('Testing get current user...');
  console.log('Current cookies before request:', cookieJar);
  
  const { data, error, status, headers } = await makeRequest('users/me');

  if (error) {
    console.error('Failed to get current user:', error);
    console.error('Status code:', status);
    if (headers) {
      console.log('Response headers:', JSON.stringify(headers, null, 2));
    }
    return false;
  }

  console.log('Current user data:', data);
  return true;
};

// Test logout
const testLogout = async () => {
  console.log('Testing logout...');
  const { data, error } = await makeRequest('/users/logout', {
    method: 'POST',
  });

  if (error) {
    console.error('Logout failed:', error);
    return false;
  }

  console.log('Logout successful:', data);
  return true;
};

// Test protected route without auth
const testUnauthenticatedAccess = async () => {
  console.log('Testing unauthenticated access to protected route...');
  const { data, error } = await makeRequest('/users/me');

  if (error && error.statusCode === 401) {
    console.log('Unauthenticated access blocked as expected');
    return true;
  }

  console.error('Unauthenticated access was not blocked');
  return false;
};

// Run all tests
const runTests = async () => {
  console.log('Starting authentication flow tests...\n');

  // Test registration and login
  if (!(await testRegistration())) return;
  if (!(await testLogin())) return;
  
  // Test authenticated endpoints
  if (!(await testGetCurrentUser())) return;
  
  // Test logout
  if (!(await testLogout())) return;
  
  // Verify unauthenticated access is blocked
  await testUnauthenticatedAccess();

  console.log('\nAll tests completed successfully!');};

// Run the tests
runTests().catch(console.error);
