# Web API Documentation

## Overview
The Web API is a Node.js/Express application that serves as an intermediary between the frontend and the Rust service. It handles request validation, authentication, and response transformation.

## API Endpoints

### 1. IP Lookup

#### `GET /api/lookup`
Looks up geolocation and threat data for the client's IP address.

**Headers:**
- `x-api-key`: API key for authentication
- `x-real-ip`: (Optional) Override client IP
- `x-forwarded-for`: (Optional) Alternative IP header

#### `GET /api/lookup/:ip`
Looks up geolocation and threat data for a specific IP address.

**Parameters:**
- `ip`: The IP address to look up

## Key Components

### 1. Lookup Controller (`lookupController.ts`)

#### Responsibilities:
- Extract client IP from request headers
- Validate input
- Call Rust service
- Transform response format
- Handle errors

#### Key Functions:
```typescript
async function lookupIpAddress(req: Request, res: Response)
function getClientIp(req: Request): string | undefined
function transformLookupResponse(response: LookupResponse): IpLookupResult
```

### 2. Rust Service Client (`rustService.ts`)

#### Features:
- HTTP client for Rust service communication
- Request/response logging
- Error handling
- Header forwarding

#### Configuration:
```typescript
{
  baseURL: process.env.RUST_SERVICE_URL,
  timeout: 10000, // 10 seconds
  headers: {
    'Content-Type': 'application/json',
    'Accept': 'application/json'
  }
}
```

### 3. Data Models (`types/ipLookup.ts`)

#### Interfaces:
- `LookupResponse`: Raw response from Rust service
- `IpLookupResult`: Transformed response for frontend
- Supporting interfaces: `Country`, `Location`, `GeoInfo`, `AsnInfo`

## Data Flow

1. Request received at `/api/lookup` or `/api/lookup/:ip`
2. Extract and validate client IP
3. Forward request to Rust service with headers
4. Transform Rust service response
5. Return transformed response to client

## Error Handling

- 400 Bad Request: Invalid input
- 401 Unauthorized: Missing or invalid API key
- 500 Internal Server Error: Service error
- 503 Service Unavailable: Rust service unavailable

## Security

- API key authentication
- Input validation
- Rate limiting (via middleware)
- Secure header handling
- CORS configuration

## Configuration

### Environment Variables
```
PORT=3001
NODE_ENV=development
RUST_SERVICE_URL=http://localhost:8080
API_KEYS=key1,key2
```

## Development

### Available Scripts
```bash
# Start development server
npm run dev

# Build for production
npm run build

# Start production server
npm start

# Run tests
npm test
```

## Best Practices

1. Keep controllers thin
2. Move business logic to services
3. Use TypeScript interfaces for type safety
4. Implement proper error handling
5. Log important events
6. Use environment variables for configuration
7. Write unit and integration tests

## Monitoring

- Request logging with timestamps
- Error tracking
- Performance metrics
- Health check endpoint at `/health`
