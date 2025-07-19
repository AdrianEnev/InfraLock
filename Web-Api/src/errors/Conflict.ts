export class Conflict extends Error {
  public type: string;
  constructor(type: string, message = 'Conflict') {
    super(message);
    this.type = type;
    Object.setPrototypeOf(this, Conflict.prototype);
  }
} 