# Geolocation API

A high-performance geolocation API service built with Rust and Axum, using the MaxMind GeoIP2 database for IP geolocation lookups.

## Features

- Fast IP geolocation lookups
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

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd geolocation
   ```

2. Download the MaxMind GeoLite2 City database (MMDB format) and place it in the `data` directory:
   ```bash
   mkdir -p data
   # Download GeoLite2-City.mmdb from MaxMind and place it in the data/ directory
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
- `RUST_LOG`: Logging level (default: `geolocation=info,tower_http=info`)

## Running the Server

```bash
# Run with default settings
cargo run --release

# Or with custom configuration
GEO_SERVER__PORT=8080 \
GEO_MAXMIND__DB_PATH=/path/to/GeoLite2-City.mmdb \
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
```

### Integration Tests

Integration tests are located in the `tests/` directory. To run them:

```bash
cargo test --test integration
```

## Performance

The service is built with performance in mind:
- Asynchronous I/O with Tokio
- Thread-safe state management with `Arc` and `RwLock`
- Efficient memory usage with zero-copy deserialization

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [MaxMind](https://www.maxmind.com/) for the GeoIP2 database
- [Axum](https://github.com/tokio-rs/axum) for the web framework
- [Tokio](https://tokio.rs/) for async runtime