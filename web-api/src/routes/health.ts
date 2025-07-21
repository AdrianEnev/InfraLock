import { Router } from 'express';
import prisma from '../prisma';
import { Request, Response } from 'express';

const router = Router();

// Simple health check endpoint
router.get('/', async (req: Request, res: Response) => {
  try {
    // Check database connection
    await prisma.$queryRaw`SELECT 1`;
    
    res.json({
      status: 'ok',
      timestamp: new Date().toISOString(),
      uptime: process.uptime(),
      database: 'connected',
    });
  } catch (error) {
    console.error('Health check failed:', error);
    res.status(503).json({
      status: 'error',
      timestamp: new Date().toISOString(),
      error: 'Service Unavailable',
      database: 'disconnected',
    });
  }
});

// Detailed health check with more system information
router.get('/detailed', async (req: Request, res: Response) => {
  try {
    const [dbCheck, memoryUsage] = await Promise.all([
      prisma.$queryRaw`SELECT 1`,
      process.memoryUsage(),
    ]);

    const stats = {
      status: 'ok',
      timestamp: new Date().toISOString(),
      uptime: process.uptime(),
      database: 'connected',
      memory: {
        rss: memoryUsage.rss,
        heapTotal: memoryUsage.heapTotal,
        heapUsed: memoryUsage.heapUsed,
        external: memoryUsage.external,
      },
      node: {
        version: process.version,
        platform: process.platform,
        arch: process.arch,
      },
    };

    res.json(stats);
  } catch (error) {
    console.error('Detailed health check failed:', error);
    res.status(503).json({
      status: 'error',
      timestamp: new Date().toISOString(),
      error: 'Service Unavailable',
      details: error instanceof Error ? error.message : 'Unknown error',
    });
  }
});

export default router;
