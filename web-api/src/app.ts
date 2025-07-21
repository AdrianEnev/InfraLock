import express, { Application, Request, Response, NextFunction } from 'express';
import dotenv from 'dotenv';
import cors from 'cors';
import cookieParser from 'cookie-parser';
import userRoutes from './routes/user';
import { globalErrorHandler } from './middleware/globalErrorHandler';

dotenv.config({
    quiet: true
});

const app: Application = express();

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
    allowedHeaders: ['Content-Type', 'Authorization', 'x-api-key'],
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

// Health check endpoint
app.get('/health', (_req: Request, res: Response) => {
    res.status(200).json({ status: 'ok' });
});

// Global error handler
app.use(globalErrorHandler());

// Example root route
app.get('/', (req: Request, res: Response) => {
    res.json({ message: 'API is running' });
});

export default app;
