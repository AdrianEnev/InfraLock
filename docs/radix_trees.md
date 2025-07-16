# IP Lookup Service with Radix Trees

## Overview

This document describes the implementation of an IP lookup service that efficiently categorizes IP addresses using radix trees. The service supports multiple IP categories (VPN, HTTP proxy, SOCKS proxies, Tor exit nodes) and can load data from various sources with different formats.

## Architecture

### Core Components

1. **RadixTree**
   - Efficiently stores IP networks with associated categories
   - Uses separate trees for IPv4 and IPv6 addresses
   - Thread-safe implementation with `SharedRadixTree`

2. **IpLookupService**
   - Manages the radix tree and data loading
   - Handles background updates
   - Provides a clean API for IP lookups

3. **IpRangeLoader**
   - Loads IP ranges from files or downloads them from URLs
   - Supports multiple source formats (default, IP:PORT, Tor exit lists)
   - Implements caching with configurable TTL

4. **Types**
   - `IpCategory`: Enumerates supported IP categories
   - `IpRange`: Represents a range of IPs with metadata
   - `SourceFormat`: Defines supported source formats

## Data Flow

1. **Initialization**
   - Service loads configuration
   - Creates necessary directories
   - Initializes the radix tree

2. **Data Loading**
   - For each configured source:
     1. Check if cached data exists and is fresh
     2. If not, download and parse the source
     3. Convert to `IpRange` objects
     4. Insert into the radix tree

3. **Lookup**
   - Convert input IP to binary format
   - Search the appropriate tree (IPv4/IPv6)
   - Return the most specific matching category

## Source Formats

The service supports three source formats:

1. **Default**
   - Plain IP (e.g., `192.168.1.1`)
   - CIDR notation (e.g., `192.168.1.0/24`)

2. **IP:PORT**
   - Extracts IP from `IP:PORT` format
   - Converts to `/32` (IPv4) or `/128` (IPv6) networks

3. **Tor Exit List**
   - Parses Tor exit node list format
   - Extracts IPs from `ExitNode` entries
   - Skips metadata lines

## Performance

- **Lookup Time**: O(k) where k is the number of bits in the address (32 for IPv4, 128 for IPv6)
- **Memory Usage**: Optimized by sharing common prefixes
- **Concurrency**: Thread-safe with `RwLock` for concurrent reads

## Configuration

```rust
struct IpLookupServiceConfig {
    data_dir: PathBuf,           // Directory for cached data
    check_updates: bool,         // Check for updates on startup
    update_interval_secs: u64,   // How often to check for updates
    max_cache_age_secs: u64,     // Maximum cache age before update
    sources: Vec<IpRangeSource>, // List of data sources
}

struct IpRangeSource {
    url: String,        // Source URL
    category: IpCategory, // Category for these IPs
    name: String,       // Short name/identifier
    enabled: bool,      // Whether to use this source
    format: SourceFormat, // Source format
}
```

## Unused Methods

### In `RadixTree`
- `remove()`: Not currently used but could be useful for dynamic updates
- `stats()`: For collecting lookup statistics, not currently utilized
- `save_to_file()`/`load_from_file()`: Alternative persistence mechanism not in use

### In `SharedRadixTree`
- `save_to_file()`/`load_from_file()`: Not currently used in favor of the service-level loading
- `stats()`: For monitoring, not currently utilized

### In `IpRange`
- `touch()`: For updating timestamps, not currently used

## Error Handling

The service uses a custom `IpRangeError` type for consistent error handling across operations.

## Testing

Unit tests cover:
- Tree insertion and lookup
- Thread safety
- Source format parsing
- File I/O operations

## Future Improvements

1. **Dynamic Updates**: Add/remove ranges at runtime
2. **More Sources**: Support additional IP list providers
3. **Metrics**: Add Prometheus metrics for monitoring
4. **LRU Cache**: For frequently accessed IPs
5. **Bulk Lookups**: Optimize for batch operations

## Dependencies

- `ipnetwork`: IP network manipulation
- `ip_network_table`: Radix tree implementation
- `tokio`: Async runtime
- `reqwest`: HTTP client
- `serde`: Serialization/deserialization
- `tracing`: Structured logging

## Usage Example

```rust
// Initialize the service
let config = IpLookupServiceConfig {
    data_dir: PathBuf::from("data/ip_ranges"),
    check_updates: true,
    update_interval_secs: 3600,
    max_cache_age_secs: 86400,
    sources: vec![
        IpRangeSource {
            url: "https://example.com/ip_list.txt".to_string(),
            category: IpCategory::Vpn,
            name: "vpn-list".to_string(),
            enabled: true,
            format: SourceFormat::Default,
        },
    ],
};

let service = IpLookupService::new(config);
service.start_background_updates();

// Perform lookups
let ip: IpAddr = "192.168.1.1".parse().unwrap();
if let Some(category) = service.tree().lookup(ip) {
    println!("IP {} is a {:?}", ip, category);
}
```

## License

[Your license information here]
