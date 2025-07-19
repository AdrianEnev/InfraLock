import express, { Application, Request, Response } from 'express';
import dotenv from 'dotenv';
import userRoutes from './routes/user';
import { globalErrorHandler } from './middlewares/globalErrorHandler';

dotenv.config();

const app: Application = express();

app.use(express.json());
app.use('/api/users', userRoutes);
app.use(globalErrorHandler());

// Example root route
app.get('/', (req: Request, res: Response) => {
  res.json({ message: 'API is running' });
});

export default app;
