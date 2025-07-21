import NextAuth from 'next-auth';

declare module 'next-auth' {
  /**
   * Extend the built-in session types
   */
  interface Session {
    user: {
      id?: string;
      name?: string | null;
      email?: string | null;
      image?: string | null;
      apiKey?: string | null;
    };
  }

  /**
   * Extend the built-in user types
   */
  interface User {
    id?: string;
    name?: string | null;
    email?: string | null;
    image?: string | null;
    apiKey?: string | null;
  }
}

declare module 'next-auth/jwt' {
  /** Extend the built-in JWT types */
  interface JWT {
    id?: string;
    apiKey?: string | null;
  }
}
