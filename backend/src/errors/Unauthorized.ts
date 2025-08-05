export class Unauthorized extends Error {
    type: string;
    
    constructor(message = 'Unauthorized', type = 'api/unauthorized') {
        super(message);
        this.type = type;
        Object.setPrototypeOf(this, Unauthorized.prototype);
    }
}