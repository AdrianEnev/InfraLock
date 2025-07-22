# Web Frontend Documentation

## Overview
The web frontend is a React application built with TypeScript that provides a user interface for IP lookups. It communicates with the Node.js backend to retrieve and display IP information.

## Key Components

### 1. IP Lookup Service (`ipLookupService.ts`)

#### Responsibilities:
- Handles API communication with the backend
- Manages request/response transformation
- Implements error handling and fallback to mock data

#### Key Functions:
```typescript
async function lookupSelfIpAddress(): Promise<IpLookupResult>
```

#### Request Headers:
- `x-api-key`: API key for authentication
- `x-real-ip`: Client IP address (demo mode uses 8.8.8.8)
- `x-forwarded-for`: Alternative IP header for proxy support

### 2. useIpLookup Hook (`useIpLookup.ts`)

#### Features:
- Manages loading and error states
- Provides a refresh function
- Handles API responses and errors
- Returns a consistent response format

#### Usage Example:
```typescript
const { data, isLoading, error, refresh } = useIpLookup();
```

### 3. API Utility (`utils/api.ts`)

#### Features:
- Centralized API request handling
- Request/response interception
- Error handling
- Support for different content types

#### Configuration:
- Base URL from environment variables
- Automatic JSON parsing
- Error handling with `handleApiError`

## Data Flow

1. Component calls `useIpLookup()` hook
2. Hook calls `lookupSelfIpAddress()` from the service
3. Service makes HTTP request to the backend
4. Response is transformed and returned
5. Hook updates its state with the result
6. Component re-renders with the new data

## Error Handling

- Network errors are caught and logged
- Fallback to mock data in development
- User-friendly error messages
- Loading states for better UX

## Development

### Environment Variables
```
NEXT_PUBLIC_API_URL=your_api_url
NEXT_PUBLIC_API_KEY=your_api_key
```

### Available Scripts
- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm start` - Start production server
- `npm test` - Run tests

## Best Practices

1. Always use the `useIpLookup` hook instead of direct service calls
2. Handle loading and error states in components
3. Use TypeScript interfaces for type safety
4. Keep API-related code in the services directory
5. Mock API responses in tests
