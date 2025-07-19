import { Request, Response } from 'express';
import * as userService from '../services/userService';
import { generateJWT } from '../utils/authUtils';

export const registerUser = async (req: Request, res: Response) => {
  try {
    const { username, email, password } = req.body;
    if (!username || !email || !password) {
      return res.status(400).json({ error: 'Username, email, and password are required.' });
    }
    const user = await userService.createUser(username, email, password);
    res.status(201).json({ id: user.id, email: user.email, apiKey: user.apiKey, createdAt: user.createdAt });
  } catch (err: any) {
    if (err.code === 'P2002') {
      return res.status(409).json({ error: 'Username or email already exists.' });
    }
    res.status(500).json({ error: 'Registration failed.' });
  }
};

export const loginUser = async (req: Request, res: Response) => {
  try {
    const { email, password } = req.body;
    if (!email || !password) {
      return res.status(400).json({ error: 'Email and password are required.' });
    }
    const user = await userService.authenticateUser(email, password);
    if (!user) {
      return res.status(401).json({ error: 'Invalid email or password.' });
    }
    // Generate JWT token
    const token = generateJWT({ userId: user.id, email: user.email });
    res.status(200).json({ id: user.id, email: user.email, apiKey: user.apiKey, token });
  } catch (err) {
    res.status(500).json({ error: 'Login failed.' });
  }
};

export const getApiKey = async (req: Request, res: Response) => {
  try {
    const user = (req as any).user;
    if (!user) {
      return res.status(401).json({ error: 'Unauthorized' });
    }
    res.status(200).json({ apiKey: user.apiKey });
  } catch (err) {
    res.status(500).json({ error: 'Failed to retrieve API key.' });
  }
}; 