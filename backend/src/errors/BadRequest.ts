export class BadRequest extends Error {
    constructor(message = 'Bad Request') {
        super(message);
        Object.setPrototypeOf(this, BadRequest.prototype);
    }
} 