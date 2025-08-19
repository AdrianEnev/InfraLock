# API Key Validation System

This document provides an overview of the API key validation system, including its architecture and configuration. The rust-service no longer validates API keys; validation is handled exclusively by the backend.

## Architecture

The API key validation system consists of the following components:

1. **Backend (Web API) Service**
   - Handles API key validation requests
   - Implements rate limiting and caching
   - Provides health check endpoints

2. **Rust Service**
   - Performs fast geolocation/threat logic
   - Does not perform API key validation
   - Exposes lookup and threat endpoints only

## Configuration

### Backend Configuration

Environment variables for the Backend:

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
# Optional (with defaults)
CACHE_TTL_SECONDS=300
RUST_LOG=info
```

## API Endpoints

### Backend Endpoints (for validation)

- `POST /internal/validate-key` - Validate an API key (internal use)
- `GET /health` - Health check endpoint

### Rust Service Endpoints (no validation)

- `GET /api/lookup/{ip}` - Lookup IP details
- `GET /api/lookup/self` - Lookup client details from headers
- `GET /api/threat-score/{ip}` - Threat score for an IP
- `GET /api/threat-score/self` - Threat score for client IP
- `GET /health` - Health check endpoint

## Development

Run services directly (Docker Compose not required):

```bash
# Backend
cd backend
npm install
npm run dev

# Rust Service
cd ../rust-service
cargo run
```

## Notes

- API key validation is exclusively enforced by the backend (middleware and internal endpoints).
- rust-service focuses on speed and logic execution only.
