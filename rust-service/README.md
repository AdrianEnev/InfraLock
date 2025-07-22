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

## Installation

1. Clone the repository (if not already done):
   ```bash
   git clone <repository-url>
   cd geolocation/rust-service
   ```

2. Create the required directories and download the database files:
   ```bash
   mkdir -p data/maxmind
   
   # Download GeoLite2 databases from MaxMind and place them in the data/maxmind/ directory:
   # - GeoLite2-City.mmdb
   # - GeoLite2-ASN.mmdb
   
   # IP ranges and proxy lists will be automatically downloaded to data/ip_ranges/
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

# MaxMind Database Paths
GEO_MAXMIND__DB_PATH=data/maxmind/GeoLite2-City.mmdb
GEO_MAXMIND__ASN_DB_PATH=data/maxmind/GeoLite2-ASN.mmdb

# VPN and Proxy Detection Paths
GEO_VPN_DETECTOR__DB_PATH=data/vpns/ipv4.txt

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
GEO_MAXMIND__ASN_DB_PATH=/path/to/GeoLite2-ASN.mmdb \
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
