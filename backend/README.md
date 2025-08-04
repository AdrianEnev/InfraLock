# Web API

Express.js backend API that serves as a middleware between the frontend and the Rust geolocation service.

## Features

- RESTful API endpoints for geolocation lookups
- Request validation and rate limiting
- Error handling and logging
- Environment-based configuration
- Health check endpoint
- CORS support
- Request/Response logging
- Input validation

## Prerequisites

- Node.js 16+ and npm
- Access to the Rust geolocation service
- Environment variables (see Configuration section)

## Getting Started

1. Clone the repository (if not already done):
   ```bash
   git clone <repository-url>
   cd geolocation/web-api
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Set up environment variables:
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

4. Start the development server:
   ```bash
   npm run dev
   ```

## Project Structure

```
web-api/
├── src/
│   ├── config/        # Configuration files
│   ├── controllers/   # Route controllers
│   ├── middlewares/   # Express middlewares
│   ├── routes/        # Route definitions
│   ├── services/      # Business logic
│   ├── utils/         # Utility functions
│   ├── validators/    # Request validation
│   ├── app.js         # Express app setup
│   └── server.js      # Server entry point
├── .env.example      # Example environment variables
├── package.json      # Project dependencies
└── README.md         # This file
```

## Configuration

Copy `.env.example` to `.env` and update the following variables:

```env
# Server Configuration
PORT=4000
NODE_ENV=development

# Rust Service Configuration
RUST_SERVICE_URL=http://localhost:3000
RUST_SERVICE_TIMEOUT=5000

# Security
CORS_ORIGIN=http://localhost:3001
RATE_LIMIT_WINDOW_MS=900000  # 15 minutes
RATE_LIMIT_MAX_REQUESTS=100  # Max requests per window per IP

# Logging
LOG_LEVEL=info
```

## Available Scripts

- `npm start` - Start the production server
- `npm run dev` - Start the development server with hot-reload
- `npm test` - Run tests
- `npm run lint` - Run ESLint
- `npm run format` - Format code with Prettier

## API Endpoints

### Health Check

```http
GET /api/health
```

**Response:**
```json
{
  "status": "ok",
  "timestamp": "2025-07-20T17:00:00.000Z",
  "version": "1.0.0"
}
```

### IP Lookup

```http
GET /api/lookup?ip=8.8.8.8
```

**Response:**
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

### Environment Setup

1. Install Node.js 16+ from [nodejs.org](https://nodejs.org/)
2. Install dependencies:
   ```bash
   npm install
   ```
3. Set up environment variables (see Configuration section)

### Running Tests

```bash
npm test
```

### Linting

```bash
npm run lint
```

### Formatting

```bash
npm run format
```

## Production Deployment

### Building for Production

```bash
npm run build
```

### Running in Production

```bash
NODE_ENV=production npm start
```

## Docker

Build the Docker image:

```bash
docker build -t geolocation-api .
```

Run the container:

```bash
docker run -p 4000:4000 --env-file .env geolocation-api
```

## License

MIT
