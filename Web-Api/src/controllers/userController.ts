import { Request, Response, NextFunction } from 'express';
import * as userService from '../services/userService';
import { generateJWT } from '../utils/authUtils';
import { Validation } from '../errors/validation';
import { Unauthorized } from '../errors/Unauthorized';
import { Conflict } from '../errors/Conflict';

export const registerUser = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const { username, email, password } = req.body;
    if (!username || !email || !password) {
      return next(new Validation({
        array: () => [
          { type: 'field', path: 'username', msg: 'Required' },
          { type: 'field', path: 'email', msg: 'Required' },
          { type: 'field', path: 'password', msg: 'Required' }
        ].filter(f => !eval(f.path)),
      }, 'Username, email, and password are required.'));
    }
    const user = await userService.createUser(username, email, password);
    res.status(201).json({ id: user.id, email: user.email, apiKey: user.apiKey, createdAt: user.createdAt });
  } catch (err: any) {
    if (err.code === 'P2002') {
      return next(new Conflict('api/user-conflict', 'Username or email already exists.'));
    }
    next(err);
  }
};

export const loginUser = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const { email, password } = req.body;
    if (!email || !password) {
      return next(new Validation({
        array: () => [
          { type: 'field', path: 'email', msg: 'Required' },
          { type: 'field', path: 'password', msg: 'Required' }
        ].filter(f => !eval(f.path)),
      }, 'Email and password are required.'));
    }
    const user = await userService.authenticateUser(email, password);
    if (!user) {
      return next(new Unauthorized('Invalid email or password.'));
    }
    // Generate JWT token
    const token = generateJWT({ userId: user.id, email: user.email });
    res.status(200).json({ id: user.id, email: user.email, apiKey: user.apiKey, token });
  } catch (err) {
    next(err);
  }
};

export const getApiKey = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const user = (req as any).user;
    if (!user) {
      return next(new Unauthorized('Unauthorized'));
    }
    res.status(200).json({ apiKey: user.apiKey });
  } catch (err) {
    next(err);
  }
}; 