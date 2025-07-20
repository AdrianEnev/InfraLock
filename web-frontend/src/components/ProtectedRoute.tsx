'use client';

import { useRouter } from 'next/navigation';
import { useEffect } from 'react';
import { useGlobalContext } from './GlobalContext';

interface ProtectedRouteProps {
  children: React.ReactNode;
  requiredRole?: string;
  redirectTo?: string;
}

export default function ProtectedRoute({
  children,
  requiredRole,
  redirectTo = '/login',
}: ProtectedRouteProps) {
  const { isAuthenticated, isLoading, user } = useGlobalContext();
  const router = useRouter();

  useEffect(() => {
    // Only redirect if not loading and not authenticated
    if (!isLoading && !isAuthenticated) {
      const redirectUrl = `${redirectTo}?from=${encodeURIComponent(window.location.pathname)}`;
      router.push(redirectUrl);
    }

    // If role is required and user doesn't have it, redirect to home
    if (!isLoading && isAuthenticated && requiredRole && user?.role !== requiredRole) {
      console.warn(`User does not have required role: ${requiredRole}`);
      router.push('/');
    }
  }, [isAuthenticated, isLoading, requiredRole, router, redirectTo, user?.role]);

  // Show loading state while checking auth
  if (isLoading || !isAuthenticated) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-red-500"></div>
      </div>
    );
  }

  // If role is required and user doesn't have it, don't render children
  if (requiredRole && user?.role !== requiredRole) {
    return null;
  }

  return <>{children}</>;
}
