# Rust Service Documentation

## Overview
The Rust Service is the core of the IP lookup system, providing high-performance IP geolocation and threat detection. It's built with Rust for maximum performance and safety.

## Architecture

### 1. Core Components

#### `IpLookupService` (`ip_lookup/service.rs`)
- Manages IP range data and lookups
- Handles background updates
- Thread-safe with `Arc<RwLock<>>`

#### `RadixTree` (`ip_lookup/tree.rs`)
- High-performance IP range lookups
- Separate trees for IPv4 and IPv6
- Thread-safe with `parking_lot::RwLock`
- Serialization/deserialization support

#### `LookupService` (`services/lookup_service.rs`)
- Coordinates IP lookups
- Integrates with MaxMind DB
- Calculates threat scores
- Caches results

### 2. Data Loading

#### `BackgroundUpdater` (`services/background_updater.rs`)
- Periodically updates IP databases
- Supports multiple data sources
- Handles errors and retries

#### Data Sources
- VPN/Datacenter IPs
- HTTP/HTTPS proxies
- SOCKS4/5 proxies
- TOR exit nodes

## Data Flow

1. Request received from web API
2. IP address validated and normalized
3. Lookup in radix tree for IP category
4. Query MaxMind DB for geolocation/ASN
5. Calculate threat score
6. Determine recommended action
7. Cache and return result

## API Endpoints

### `GET /api/lookup/self`
Look up client's IP information.

**Headers:**
- `X-API-Key`: API key
- `X-Real-IP`: Client IP
- `X-Forwarded-For`: Alternative IP header

### `GET /health`
Service health check.

## Configuration

### Environment Variables
```
RUST_LOG=info
DATABASE_URL=./data/GeoLite2-City.mmdb
ASN_DATABASE_URL=./data/GeoLite2-ASN.mmdb
CACHE_TTL_SECONDS=3600
```

## Performance

### Optimizations
- In-memory radix trees
- Lock-free reads
- Efficient memory usage
- Async I/O

### Caching
- In-memory cache with TTL
- Configurable cache size
- Automatic eviction

## Security

### Features
- Input validation
- Secure defaults
- Memory safety
- Thread safety

### Best Practices
- Zero-copy parsing
- Bounds checking
- Safe error handling
- Minimal dependencies

## Development

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_lookup

# Run with logging
RUST_LOG=debug cargo test -- --nocapture
```

## Monitoring

### Logging
- Structured logging with `tracing`
- Configurable log levels
- JSON formatting

### Metrics
- Request counts
- Error rates
- Cache hit/miss
- Lookup latency

## Deployment

### Docker
```dockerfile
FROM rust:1.70-slim

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["./target/release/rust-service"]
```

### Systemd Service
```ini
[Unit]
Description=IP Lookup Service
After=network.target

[Service]
Type=simple
User=rust
WorkingDirectory=/opt/rust-service
ExecStart=/opt/rust-service/rust-service
Restart=always
Environment=RUST_LOG=info
Environment=DATABASE_URL=/var/lib/rust-service/GeoLite2-City.mmdb

[Install]
WantedBy=multi-user.target
```

## Troubleshooting

### Common Issues
1. **Missing Database Files**
   - Ensure MaxMind DB files are present
   - Check file permissions

2. **High Memory Usage**
   - Check for memory leaks
   - Adjust cache size
   - Monitor with `htop` or similar

3. **Performance Issues**
   - Enable logging
   - Check system resources
   - Profile with `perf` or `flamegraph`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit changes
4. Push to the branch
5. Create a pull request

### Code Style
- Follow Rustfmt
- Use clippy for linting
- Document public APIs
- Write unit tests

## License
[Specify License]
