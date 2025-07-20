import bcrypt from 'bcrypt';
import crypto from 'crypto';
import jwt from 'jsonwebtoken';
import { Response } from 'express';

const SALT_ROUNDS = 10;
const JWT_SECRET = process.env.JWT_SECRET!;
const JWT_EXPIRES_IN = '1h';
const COOKIE_NAME = 'token';

// Cookie configuration
export const COOKIE_OPTIONS = {
  httpOnly: true,
  secure: process.env.NODE_ENV === 'production',
  sameSite: 'lax' as const,
  maxAge: 60 * 60 * 1000, // 1 hour
  path: '/',
};

export const hashPassword = async (password: string): Promise<string> => {
    return bcrypt.hash(password, SALT_ROUNDS);
};

export const comparePassword = async (password: string, hash: string): Promise<boolean> => {
    return bcrypt.compare(password, hash);
};

export const generateApiKey = (): string => {
    return 'sk_' + crypto.randomBytes(32).toString('hex');
};

export const generateJWT = (payload: object): string => {
    return jwt.sign(payload, JWT_SECRET, { expiresIn: JWT_EXPIRES_IN });
};

export const verifyJWT = (token: string): any => {
    return jwt.verify(token, JWT_SECRET);
};

// Set JWT in HTTP-only cookie
export const setAuthCookie = (res: Response, token: string): void => {
  res.cookie(COOKIE_NAME, token, COOKIE_OPTIONS);
};

// Clear JWT cookie
export const clearAuthCookie = (res: Response): void => {
  res.clearCookie(COOKIE_NAME, COOKIE_OPTIONS);
};

// Get token from request (cookie or Authorization header)
export const getTokenFromRequest = (req: any): string | null => {
  // Try to get token from cookies first
  if (req.cookies?.[COOKIE_NAME]) {
    return req.cookies[COOKIE_NAME];
  }
  
  // Fall back to Authorization header
  const authHeader = req.headers.authorization;
  if (authHeader && authHeader.startsWith('Bearer ')) {
    return authHeader.split(' ')[1];
  }
  
  return null;
};