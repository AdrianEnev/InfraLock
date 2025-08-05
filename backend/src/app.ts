import express, { Application, Request, Response, NextFunction } from 'express';
import dotenv from 'dotenv';
import cors from 'cors';
import cookieParser from 'cookie-parser';
import userRoutes from './routes/user';
import internalRoutes from './routes/internal';
import healthRoutes from './routes/health';
import lookupRoutes from './routes/lookup';
import unlimitedRoutes from './routes/unlimited';
import { globalErrorHandler } from './middleware/globalErrorHandler';
import { cache } from './utils/redis';

dotenv.config({
    quiet: true
});

const app: Application = express();

// Trust first proxy (important for correct IP resolution when behind a proxy)
app.set('trust proxy', true);

// Enable CORS with specific options
const allowedOrigins: string[] = [
    'http://localhost:3000',
    'http://localhost:4000',
    ...(process.env.FRONTEND_URL ? [process.env.FRONTEND_URL] : [])
];

const corsOptions = {
    origin: (origin: string | undefined, callback: (err: Error | null, allow?: boolean) => void) => {
        // Allow requests with no origin (like mobile apps or curl requests)
        if (!origin) return callback(null, true);
        
        const isAllowed = allowedOrigins.some(allowedOrigin => 
            origin === allowedOrigin || 
            origin.startsWith(allowedOrigin.replace(/^https?:\/\//, ''))
        );
        
        if (isAllowed) {
            callback(null, true);
        } else {
            console.error('Not allowed by CORS:', origin);
            callback(new Error('Not allowed by CORS'));
        }
    },
    credentials: true,
    methods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
    allowedHeaders: ['Content-Type', 'Authorization', 'x-api-key', 'x-real-ip', 'x-forwarded-for'],
    exposedHeaders: ['set-cookie']
};

// Middleware
app.use(cors(corsOptions));
app.use(express.json());
app.use(cookieParser());

// Logging middleware
app.use((req: Request, _res: Response, next: NextFunction) => {
    console.log(`${new Date().toISOString()} - ${req.method} ${req.path}`);
    next();
});

// API routes
app.use('/api/users', userRoutes);
app.use('/api/lookup', lookupRoutes);
app.use('/unlimited', unlimitedRoutes);
app.use('/internal', internalRoutes);

// Health check routes
app.use('/health', healthRoutes);

// Add Redis health check endpoint
app.get('/health/redis', async (_req: Request, res: Response) => {
  try {
    const testKey = 'health:test';
    const testValue = { status: 'ok', timestamp: new Date().toISOString() };
    
    await cache.set(testKey, testValue, { ttl: 60 });
    const cachedValue = await cache.get(testKey);
    
    if (!cachedValue) {
      throw new Error('Failed to read from cache');
    }
    
    res.status(200).json({
      status: 'ok',
      redis: 'connected',
      timestamp: new Date().toISOString(),
    });
  } catch (error) {
    console.error('Redis health check failed:', error);
    res.status(503).json({
      status: 'error',
      redis: 'disconnected',
      error: 'Redis health check failed',
      timestamp: new Date().toISOString(),
    });
  }
});

// Global error handler
app.use(globalErrorHandler());

// Example root route
app.get('/', (req: Request, res: Response) => {
    res.json({ message: 'API is running' });
});

export default app;
