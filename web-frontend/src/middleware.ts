import { NextResponse, type NextRequest } from 'next/server';

// List of public paths that don't require authentication
const publicPaths = ['/login', '/register'];

// List of protected paths that require authentication
const protectedPaths = ['/account', '/api-keys', '/ip-lookup'];

export async function middleware(request: NextRequest) {
    const { pathname } = request.nextUrl;
    const token = request.cookies.get('token')?.value;

    // Allow public assets and API routes
    if (
        pathname.startsWith('/_next') ||
        pathname.startsWith('/api/') ||
        pathname.endsWith('.ico') ||
        pathname.endsWith('.png') ||
        pathname.endsWith('.jpg') ||
        pathname.endsWith('.jpeg') ||
        pathname.endsWith('.svg')
    ) {
        return NextResponse.next();
    }

    // Redirect authenticated users away from auth pages
    if (token && publicPaths.some(path => pathname.startsWith(path))) {
        return NextResponse.redirect(new URL('/', request.url));
    }

    // Redirect unauthenticated users from protected pages to login
    if (!token && protectedPaths.some(path => pathname.startsWith(path))) {
        const loginUrl = new URL('/login', request.url);
        loginUrl.searchParams.set('from', pathname);
        return NextResponse.redirect(loginUrl);
    }

    return NextResponse.next();
}

export const config = {
    matcher: [
        '/((?!_next/static|_next/image|favicon.ico).*)',
        '/',
        '/login',
        '/register',
        '/account',
        '/api-keys',
        '/ip-lookup',
    ],
};