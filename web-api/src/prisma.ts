import { PrismaClient, Prisma } from '@prisma/client';

const log: Prisma.LogLevel[] | undefined = process.env.DEBUG 
  ? ['query', 'info', 'warn', 'error'] 
  : undefined;

const prisma = new PrismaClient({
  log,
});

export default prisma;