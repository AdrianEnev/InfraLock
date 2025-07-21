import { Request, Response, NextFunction } from 'express';
import prisma from '../prisma';
import { Unauthorized } from '../errors/Unauthorized';

interface ValidateApiKeyRequest extends Request {
  body: {
    apiKey: string;
  };
}

/**
 * Validates an API key
 * @param req Request object containing the API key
 * @param res Response object
 * @param next Next function
 */
export const validateApiKey = async (
    req: Request,
    res: Response,
    next: NextFunction
  ) => {
    try {
      const apiKey = req.header('x-api-key');
  
      if (!apiKey) {
        return next(new Unauthorized('API key is required'));
      }
  
      // Rest of the validation logic remains the same
      const user = await prisma.user.findFirst({
        where: { apiKey },
        select: { id: true, email: true, role: true }
      });
  
      if (!user) {
        return next(new Unauthorized('Invalid or inactive API key'));
      }
  
      res.status(200).json({
        valid: true,
        user_id: user.id, // snake case because of rust
        email: user.email,
        role: user.role,
      });
    } catch (error) {
      console.error('Error validating API key:', error);
      next(error);
    }
  };