# Geolocation System

A comprehensive geolocation solution consisting of three main components:

1. **rust-service**: High-performance geolocation service built with Rust
2. **backend**: REST API layer that interfaces with the rust-service
3. **web-frontend**: User interface for interacting with the geolocation service

## Project Structure

```
geolocation/
├── rust-service/    # Core geolocation service (Rust)
├── backend/         # Backend API layer (Node.js/Express)
└── web-frontend/    # Web interface (React/Vue/Next.js)
```

## Prerequisites

- Node.js 18+ (for backend and web-frontend)
- Rust (latest stable version, for rust-service development)
- MaxMind GeoLite2 City database (MMDB format)
- VPN/Datacenter IP database (text file with CIDR ranges)
- Proxy IP lists (text files with IP:PORT format)

## Manual Setup

### 1. rust-service

See [rust-service/README.md](rust-service/README.md) for detailed setup instructions.

### 2. backend

See [backend/README.md](backend/README.md) for detailed setup instructions.

### 3. web-frontend

See [web-frontend/README.md](web-frontend/README.md) for detailed setup instructions.

## Configuration

Configuration is handled through environment variables. Copy `.env.example` to `.env` and modify as needed.

### Common Environment Variables

- `NODE_ENV`: Environment (development/production)
- `PORT`: Port for web services
- `RUST_LOG`: Logging level for rust-service

### Service-Specific Configuration

Each component has its own configuration. Please refer to the respective README files:

- [rust-service configuration](rust-service/README.md#configuration)
- [backend configuration](backend/README.md#configuration)
- [web-frontend configuration](web-frontend/README.md#configuration)

## Development

### Starting Services Individually

1. Start rust-service:
   ```bash
   cd rust-service
   cargo run --release
   ```

2. Start backend:
   ```bash
   cd backend
   npm install
   npm start
   ```

3. Start web-frontend:
   ```bash
   cd web-frontend
   npm install
   npm run dev
   ```

## Ip Lookup

Get geolocation information for a specific IP address.

```http
GET /api/lookup/{ip}
```

**Parameters:**
- `ip` (path): The IP address to look up (IPv4 or IPv6)

**Example Request:**
```http
GET /api/lookup/8.8.8.8
```

**Example Response:**
```json
{
  "ip": "8.8.8.8",
  "is_vpn_or_datacenter": false,
  "is_proxy": false,
  "proxy_type": null,
  "is_tor_exit_node": false,
  "threat_score": 0,
  "threat_details": [],
  "recommended_action": "allow",
  "geo_info": {
    "city": {
      "names": {
        "en": "Mountain View"
      }
    },
    "country": {
      "names": {
        "en": "United States"
      }
    },
    "location": {
      "latitude": 37.386,
      "longitude": -122.0838
    }
  },
  "asn_info": {
    "autonomous_system_number": 15169,
    "autonomous_system_organization": "Google LLC"
  }
}
```

### Self Lookup

Get geolocation information for the client making the request. This endpoint is designed to work behind proxies and load balancers by checking the following HTTP headers in order:

1. `X-Forwarded-For` - Used when behind a reverse proxy or load balancer. If multiple IPs are present (comma-separated), the first IP in the list is used.
2. `X-Real-IP` - An alternative header that may contain the original client IP.

**Example Request:**
```http
GET /api/lookup/self
X-Forwarded-For: 203.0.113.1, 198.51.100.1
```

**Example Response:**
```json
{
  "ip": "203.0.113.1",
  "is_vpn_or_datacenter": false,
  "is_proxy": false,
  "proxy_type": null,
  "is_tor_exit_node": false,
  "threat_score": 100,
  "threat_details": [
    "IP is associated with a data center"
  ],
  "recommended_action": "block",
  "geo_info": {
    "city": "New York",
    "country": "United States",
    "coordinates": {
      "latitude": 40.7128,
      "longitude": -74.006
    }
  },
  "asn_info": {
    "asn": 15169,
    "org": "Google LLC"
  }
}
```

**Note:** The self-lookup endpoint requires either `X-Forwarded-For` or `X-Real-IP` header to be set. If neither header is present, the request will fail with a 400 Bad Request error.

### License

MIT

### Note

Docker Compose references were removed. Run services via cargo/npm as shown above.