export class Validation extends Error {
  public e: any;
  constructor(e: any, message = 'Validation failed') {
    super(message);
    this.e = e;
    Object.setPrototypeOf(this, Validation.prototype);
  }
} 