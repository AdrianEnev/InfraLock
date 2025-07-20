export class UnprocessableContent extends Error {
    public type: string;
    constructor(type: string, message = 'Unprocessable Content') {
        super(message);
        this.type = type;
        Object.setPrototypeOf(this, UnprocessableContent.prototype);
    }
} 