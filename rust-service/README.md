# Rust Geolocation Service

High-performance geolocation service built with Rust and Axum, using the MaxMind GeoIP2 database for IP geolocation lookups.

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

## Prerequisites

- Rust (latest stable version)
- MaxMind GeoLite2 City database (MMDB format)
- VPN/Datacenter IP database (text file with CIDR ranges)
- Proxy IP lists (text files with IP:PORT format)

## Installation

1. Clone the repository (if not already done):
   ```bash
   git clone <repository-url>
   cd geolocation/rust-service
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

Configuration is done via environment variables. Copy `.env.example` to `.env` and modify as needed:

```env
# Server Configuration
GEO_SERVER__HOST=0.0.0.0
GEO_SERVER__PORT=3000

# Database Paths
GEO_MAXMIND__DB_PATH=data/GeoLite2-City.mmdb
GEO_VPN_DETECTOR__DB_PATH=data/vpns/ipv4.txt
GEO_PROXY_DETECTOR__HTTP_DB_PATH=data/proxies/http.txt
GEO_PROXY_DETECTOR__SOCKS4_DB_PATH=data/proxies/socks4.txt
GEO_PROXY_DETECTOR__SOCKS5_DB_PATH=data/proxies/socks5.txt

# Logging
RUST_LOG=geolocation=info,tower_http=info
```

## Running the Service

```bash
# Run with default settings
cargo run --release

# Or with custom configuration
GEO_SERVER__PORT=8080 \
GEO_MAXMIND__DB_PATH=/path/to/GeoLite2-City.mmdb \
cargo run --release
```

## API Endpoints

### Health Check

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

### IP Lookup

Get geolocation information for a specific IP address.

```http
GET /api/lookup/{ip}
```

**Example Response:**
```json
{
  "ip": "8.8.8.8",
  "country": "United States",
  "city": "Mountain View",
  "is_vpn": false,
  "is_proxy": false,
  "latitude": 37.386,
  "longitude": -122.0838
}
```

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Linting

```bash
cargo clippy -- -D warnings
```

### Formatting

```bash
cargo fmt
```

## Docker

Build the Docker image:

```bash
docker build -t geolocation-service .
```

Run the container:

```bash
docker run -p 3000:3000 -v $(pwd)/data:/app/data geolocation-service
```

## License

MIT
