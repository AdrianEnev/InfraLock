import express, { Application, Request, Response } from 'express';
import dotenv from 'dotenv';

dotenv.config();

const app: Application = express();

app.use(express.json());

// Example root route
app.get('/', (req: Request, res: Response) => {
  res.json({ message: 'API is running' });
});

export default app;
