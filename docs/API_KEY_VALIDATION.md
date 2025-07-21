# API Key Validation System

This document provides an overview of the API key validation system, including its architecture, configuration, and monitoring capabilities.

## Architecture

The API key validation system consists of the following components:

1. **Web API Service**
   - Handles API key validation requests
   - Implements rate limiting and caching
   - Provides health check endpoints

2. **Rust Service**
   - Validates API keys by calling the Web API
   - Implements resilient client with retries and circuit breaking
   - Provides metrics and alerting

3. **Monitoring Stack**
   - Prometheus for metrics collection
   - Alertmanager for alert routing and deduplication
   - Grafana for visualization

## Configuration

### Web API Configuration

Environment variables for the Web API:

```bash
# Required
DATABASE_URL=postgres://user:password@localhost:5432/db
JWT_SECRET=your_jwt_secret
REDIS_URL=redis://localhost:6379

# Optional (with defaults)
PORT=3000
CACHE_TTL_SECONDS=300
RATE_LIMIT_WINDOW_SECONDS=60
RATE_LIMIT_MAX_REQUESTS=100
```

### Rust Service Configuration

Environment variables for the Rust service:

```bash
# Required
WEB_API_URL=http://web-api:3000
INTERNAL_SERVICE_TOKEN=your_service_token

# Optional (with defaults)
PORT=8080
CACHE_TTL_SECONDS=300
CIRCUIT_BREAKER_FAILURE_THRESHOLD=5
CIRCUIT_BREAKER_RESET_TIMEOUT_SECS=30
```

## API Endpoints

### Web API Endpoints

- `POST /api/v1/validate-key` - Validate an API key
  ```json
  {
    "apiKey": "your_api_key_here"
  }
  ```

- `GET /health` - Health check endpoint
- `GET /health/redis` - Redis health check

### Rust Service Endpoints

- `GET /api/v1/validate?api_key=your_api_key` - Validate an API key
- `GET /metrics` - Prometheus metrics endpoint
- `GET /health` - Health check endpoint

## Monitoring and Alerting

### Metrics

The following metrics are exposed via the `/metrics` endpoint:

- `api_key_validation_total` - Total number of validation attempts
- `api_key_validation_success_total` - Number of successful validations
- `api_key_validation_failed_total` - Number of failed validations by reason
- `api_key_validation_duration_seconds` - Duration of validation requests
- `circuit_breaker_state_changes_total` - Number of circuit breaker state changes
- `circuit_breaker_rejected_requests_total` - Number of requests rejected by circuit breaker
- `cache_hits_total` - Number of cache hits
- `cache_misses_total` - Number of cache misses

### Alerting Rules

Alerting is configured to trigger in the following scenarios:

1. **High Error Rate**
   - Triggered when error rate exceeds 5% for 5 minutes
   - Severity: Warning

2. **Circuit Breaker Open**
   - Triggered when circuit breaker is open
   - Severity: Critical

3. **High Latency**
   - Triggered when p99 latency exceeds 1 second for 5 minutes
   - Severity: Warning

## Implementation Details

### Resilient Client

The Rust service uses a resilient HTTP client with the following features:

- **Retries**: Automatic retries with exponential backoff
- **Circuit Breaker**: Prevents cascading failures when the Web API is down
- **Caching**: Reduces load on the Web API by caching validation results
- **Timeouts**: Configurable timeouts for all requests

### Security Considerations

1. **API Key Storage**
   - API keys are stored as salted hashes in the database
   - Plaintext API keys are never logged

2. **Rate Limiting**
   - Implemented in the Web API to prevent abuse
   - Uses Redis for distributed rate limiting

3. **Service-to-Service Authentication**
   - Internal services authenticate using JWT tokens
   - Tokens are rotated regularly

## Troubleshooting

### Common Issues

1. **Validation Failures**
   - Check if the API key is valid and not expired
   - Verify the service token is correct
   - Check the Web API logs for errors

2. **High Latency**
   - Check the Web API health
   - Verify Redis is running and responsive
   - Check for network issues between services

3. **Circuit Breaker Tripped**
   - Check the Web API health
   - Look for recent deployments or configuration changes
   - Check for high load on the Web API

## Monitoring Dashboard

A Grafana dashboard is available at `/grafana` with the following panels:

- Request rate and error rate
- Latency percentiles
- Cache hit/miss ratio
- Circuit breaker state
- Top failing API keys

## Deployment

### Prerequisites

- Docker and Docker Compose
- Kubernetes cluster (for production)

### Local Development

```bash
# Start all services
docker-compose up -d

# Run migrations
cd web-api
npm run db:migrate

# Run tests
cd ../rust-service
cargo test
```

### Production Deployment

1. Update the environment variables in `docker-compose.prod.yml`
2. Run `docker-compose -f docker-compose.prod.yml up -d`
3. Set up monitoring and alerting

## Maintenance

### Rotating Secrets

1. Generate new secrets
2. Update environment variables
3. Restart services

### Scaling

- The Web API can be scaled horizontally
- Use a load balancer in front of the Web API instances
- Configure Redis for high availability

## Support

For support, please contact the platform team at platform@example.com.
