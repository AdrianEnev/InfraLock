import { ProblemDetails, Violation } from '../types/problemDetails';
import { Validation } from '../errors/validation';
import { Unauthorized } from '../errors/Unauthorized';
import { Conflict } from '../errors/Conflict';
import { UnprocessableContent } from '../errors/UnprocessableContent';
import { Request, Response, NextFunction } from 'express';

export const globalErrorHandler = () => {
    return (err: any, req: Request, res: Response, _next: NextFunction) => {
        console.log('Error caught in global error handler', err);

        if (err instanceof Validation) {
            return buildErrorResponse(res, validationErrorHandler(err));
        }

        if (err instanceof Unauthorized) {
            return buildErrorResponse(res, new ProblemDetails({
                type: err.type || 'api/unauthorized',
                title: 'Unauthorized',
                status: 401,
                detail: err.message
            }));
        }

        if (err instanceof Conflict) {
            return buildErrorResponse(res, new ProblemDetails({
                type: err.type,
                title: 'Conflict',
                status: 409,
                detail: err.message
            }));
        }
        
        if (err instanceof UnprocessableContent) {
            return buildErrorResponse(res, new ProblemDetails({
                type: err.type,
                title: 'InvalidData',
                status: 422,
                detail: err.message
            }));
        }
        return buildErrorResponse(res, new ProblemDetails({
            type: 'api/internal-server-error',
            title: 'Internal Server Error',
            detail: 'Something broke!',
            status: 500
        }));
    };
};

const validationErrorHandler = (error: any) => {
    const violations = error.e.array()
        .filter((error: any) => error.type === 'field')
        .map((error: any) => new Violation({
            property: error.path,
            type: error.msg
        }));

    return new ProblemDetails({
        type: 'api/bad-request',
        title: 'Bad Request',
        detail: 'Request failed validations',
        status: 400,
        violations
    });
};

const buildErrorResponse = (res: Response, problemDetails: ProblemDetails) => {
    return res.set('Content-Type', 'application/problem+json')
        .status(problemDetails.status)
        .json(problemDetails);
}; 