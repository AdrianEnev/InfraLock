# Geolocation API

A high-performance geolocation API service built with Rust and Axum, using the MaxMind GeoIP2 database for IP geolocation lookups.

## Features

- Fast IP geolocation lookups
- VPN and datacenter IP detection
- Proxy detection with type identification (HTTP/HTTPS, SOCKS4, SOCKS5)
- Support for both single IP and CIDR range checks
- RESTful API endpoints with JSON responses
- Built with async/await for high concurrency
- Structured logging with `tracing`
- Configuration via environment variables
- Health check endpoint
- Self IP detection
- Thread-safe with `Arc` and `RwLock`

## Getting Started

### Prerequisites

- Rust (latest stable version)
- MaxMind GeoLite2 City database (MMDB format)
- VPN/Datacenter IP database (text file with CIDR ranges)
- Proxy IP lists (text files with IP:PORT format)

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd geolocation
   ```

2. Download the required database files and place them in the `data` directory:
   ```bash
   mkdir -p data/proxies
   # Download GeoLite2-City.mmdb from MaxMind and place it in the data/ directory
   # Place your VPN/Datacenter IP list in data/vpns/ipv4.txt
   # Place your proxy lists in:
   # - data/proxies/http.txt (HTTP/HTTPS proxies)
   # - data/proxies/socks4.txt (SOCKS4 proxies)
   # - data/proxies/socks5.txt (SOCKS5 proxies)
   ```

3. Build the application:
   ```bash
   cargo build --release
   ```

## Configuration

Configuration can be provided via environment variables. The following variables are available:

- `GEO_SERVER__HOST`: Server host (default: `0.0.0.0`)
- `GEO_SERVER__PORT`: Server port (default: `3000`)
- `GEO_MAXMIND__DB_PATH`: Path to the MaxMind database file (default: `data/GeoLite2-City.mmdb`)
- `GEO_VPN_DETECTOR__DB_PATH`: Path to the VPN/Datacenter IP database file (default: `data/vpns/ipv4.txt`)
- `GEO_PROXY_DETECTOR__HTTP_DB_PATH`: Path to the HTTP/HTTPS proxy list (default: `data/proxies/http.txt`)
- `GEO_PROXY_DETECTOR__SOCKS4_DB_PATH`: Path to the SOCKS4 proxy list (default: `data/proxies/socks4.txt`)
- `GEO_PROXY_DETECTOR__SOCKS5_DB_PATH`: Path to the SOCKS5 proxy list (default: `data/proxies/socks5.txt`)
- `RUST_LOG`: Logging level (default: `geolocation=info,tower_http=info`)

## Running the Server

```bash
# Run with default settings
cargo run --release

# Or with custom configuration
GEO_SERVER__PORT=8080 \
GEO_MAXMIND__DB_PATH=/path/to/GeoLite2-City.mmdb \
GEO_VPN_DETECTOR__DB_PATH=/path/to/vpn_networks.txt \
GEO_PROXY_DETECTOR__HTTP_DB_PATH=/path/to/http_proxies.txt \
GEO_PROXY_DETECTOR__SOCKS4_DB_PATH=/path/to/socks4_proxies.txt \
GEO_PROXY_DETECTOR__SOCKS5_DB_PATH=/path/to/socks5_proxies.txt \
cargo run --release
```

## API Endpoints

### 1. Health Check

Check if the service is running.

```http
GET /api/health
```

**Example Response:**
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

### 2. IP Lookup

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
  "is_vpn_or_datacenter": true,
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
  }
}
```

### 3. Self Lookup

Get geolocation information for the client's IP address.

```http
GET /api/lookup/self
```

**Example Request:**
```http
GET /api/lookup/self
```

**Example Response:**
```json
{
  "ip": "192.168.1.1",
  "is_vpn_or_datacenter": false,
  "geo_info": {
    "city": {
      "names": {
        "en": "New York"
      }
    },
    "country": {
      "names": {
        "en": "United States"
      }
    },
    "location": {
      "latitude": 40.7128,
      "longitude": -74.006
    }
  }
}
```

### 4. Proxy Check

Check if an IP is a known proxy and get its type.

```http
GET /api/is_proxy/{ip_or_range}
```

**Parameters:**
- `ip_or_range`: IP address or CIDR range to check

**Example Request:**
```http
GET /api/is_proxy/1.2.3.4
```

**Example Response:**
```json
{
  "is_proxy": true,
  "proxy_type": "HTTP/HTTPS"
}
```

**Example Response for non-proxy IP:**
```json
{
  "is_proxy": false,
  "proxy_type": null
}
```

**Example Request for CIDR range:**
```http
GET /api/is_proxy/1.2.3.0/24
```

**Example Response for CIDR range:**
```json
{
  "is_proxy": true,
  "proxy_type": null
}
```

### 5. VPN/Datacenter Check

Check if an IP or CIDR range is associated with a VPN or datacenter.

```http
GET /api/is_vpn_or_datacenter/{ip_or_range}
```

**Parameters:**
- `ip_or_range`: IP address or CIDR range to check

**Example Request:**
```http
GET /api/is_vpn_or_datacenter/1.2.3.4
```

## Error Responses

All error responses follow this format:

```json
{
  "error": "Error message"
}
```

**Status Codes:**
- `400 Bad Request`: Invalid IP address format
- `404 Not Found`: IP address not found in the database
- `500 Internal Server Error`: Server error

## Testing

### Unit Tests

Run the unit tests:

```bash
cargo test
```

### Manual Testing

You can test the API using `curl`:

```bash
# Health check
curl http://localhost:3000/api/health

# IP lookup
curl http://localhost:3000/api/lookup/8.8.8.8

# Self lookup
curl http://localhost:3000/api/lookup/self

# Proxy check
curl http://localhost:3000/api/is_proxy/1.2.3.4

# VPN/Datacenter check
curl http://localhost:3000/api/is_vpn_or_datacenter/1.2.3.4
```

## Deployment

### Building for Production
```bash
cargo build --release
```

### Running with Environment Variables
```bash
GEO_MAXMIND__DB_PATH=/path/to/GeoLite2-City.mmdb \
GEO_VPN_DETECTOR__DB_PATH=/path/to/vpn_networks.txt \
GEO_PROXY_DETECTOR__HTTP_DB_PATH=/path/to/http_proxies.txt \
GEO_PROXY_DETECTOR__SOCKS4_DB_PATH=/path/to/socks4_proxies.txt \
GEO_PROXY_DETECTOR__SOCKS5_DB_PATH=/path/to/socks5_proxies.txt \
RUST_LOG=info \
./target/release/geolocation
```

## Performance

The service is built with performance in mind:
- Asynchronous I/O with Tokio
- Thread-safe state management with Arc and RwLock
- Efficient memory usage with zero-copy deserialization
- Optimized IP range lookups for VPN/datacenter detection

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [MaxMind](https://www.maxmind.com/) for the GeoIP2 database
- [Axum](https://github.com/tokio-rs/axum) for the web framework
- [Tokio](https://tokio.rs/) for async runtime
- [ipnetwork](https://crates.io/crates/ipnetwork) for IP network handling