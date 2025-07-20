# Geolocation Frontend

A modern web interface for the Geolocation service, built with React and TypeScript.

## Features

- Responsive design that works on desktop and mobile
- Real-time IP geolocation lookups
- Interactive map display
- Search history
- Dark/light theme support
- Responsive layout
- Error handling and loading states
- Environment-based configuration

## Prerequisites

- Node.js 16+ and npm
- Access to the web-api service

## Getting Started

1. Clone the repository (if not already done):
   ```bash
   git clone <repository-url>
   cd geolocation/web-frontend
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
web-frontend/
├── public/           # Static files
├── src/
│   ├── assets/       # Images, fonts, etc.
│   ├── components/   # Reusable UI components
│   ├── config/       # App configuration
│   ├── hooks/        # Custom React hooks
│   ├── layouts/      # Layout components
│   ├── pages/        # Page components
│   ├── services/     # API services
│   ├── store/        # State management
│   ├── styles/       # Global styles and themes
│   ├── types/        # TypeScript type definitions
│   ├── utils/        # Utility functions
│   ├── App.tsx       # Main App component
│   └── main.tsx      # App entry point
├── .env.example     # Example environment variables
├── package.json     # Project dependencies
└── README.md        # This file
```

## Configuration

Copy `.env.example` to `.env` and update the following variables:

```env
# API Configuration
VITE_API_BASE_URL=http://localhost:4000/api
VITE_MAPBOX_ACCESS_TOKEN=your_mapbox_access_token

# App Configuration
VITE_APP_TITLE="Geolocation App"
VITE_APP_DESCRIPTION="Find location information for any IP address"
VITE_APP_DEFAULT_THEME=light
```

## Available Scripts

- `npm run dev` - Start the development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build locally
- `npm run test` - Run tests
- `npm run lint` - Run ESLint
- `npm run format` - Format code with Prettier
- `npm run type-check` - Check TypeScript types

## Development

### Environment Setup

1. Install Node.js 16+ from [nodejs.org](https://nodejs.org/)
2. Install dependencies:
   ```bash
   npm install
   ```
3. Set up environment variables (see Configuration section)
4. Start the development server:
   ```bash
   npm run dev
   ```

### Running Tests

```bash
# Run tests in watch mode
npm test

# Run tests once
npm run test:ci

# Generate test coverage
npm run test:coverage
```

### Linting and Formatting

```bash
# Run ESLint
npm run lint

# Fix linting issues
npm run lint:fix

# Format code
npm run format
```

## Building for Production

```bash
# Build the app for production
npm run build

# Preview the production build locally
npm run preview
```

The build will be available in the `dist` directory.

## Environment Variables

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `VITE_API_BASE_URL` | Base URL for the API | Yes | `http://localhost:4000/api` |
| `VITE_MAPBOX_ACCESS_TOKEN` | Access token for Mapbox maps | Yes | - |
| `VITE_APP_TITLE` | App title | No | "Geolocation App" |
| `VITE_APP_DESCRIPTION` | App description | No | "Find location information for any IP address" |
| `VITE_APP_DEFAULT_THEME` | Default theme (light/dark) | No | "light" |

## Docker

Build the Docker image:

```bash
docker build -t geolocation-frontend .
```

Run the container:

```bash
docker run -p 3000:80 --env-file .env geolocation-frontend
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

MIT
