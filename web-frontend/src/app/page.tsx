'use client'

import Link from "next/link";
import { useIpLookup } from "@hooks/useIpLookup";

export default function Home() {

    const { result, isLoading, error } = useIpLookup();

    return (
        <div className="w-screen min-h-screen overflow-x-hidden">
            <div className="max-w-4xl mx-auto px-4 py-12">
                <div className="text-center mb-12">
                    <h1 className="text-5xl text-gray-700 font-bold mb-4">Infra<span className="text-blue-500">Lock</span> 🌍</h1>
                    <p className="text-lg text-gray-600 max-w-2xl mx-auto">
                        Powerful geolocation services with secure API key authentication.
                    </p>
                </div>

                <div className="bg-white rounded-lg shadow-md p-6 mb-12">
                    <h2 className="text-2xl font-semibold mb-4">Your IP Information</h2>
                    
                    {isLoading && (
                        <div className="text-center py-4">
                            <p className="text-gray-600">Loading your IP information...</p>
                        </div>
                    )}

                    {error && (
                        <div className="bg-red-50 border-l-4 border-red-500 p-4 mb-4">
                            <p className="text-red-700">Error: {error}</p>
                        </div>
                    )}

                    {result && (
                        <div className="space-y-4">
                            <div className="bg-gray-800 rounded-md p-4 font-mono text-sm text-gray-200 overflow-x-auto">
                                <pre className="whitespace-pre">{
`{
  ip: "${result.ip}",
  country: ${result.country ? JSON.stringify(result.country.names, null, 2) : 'null'},
  city: ${result.city || 'null'},
  isVpn: ${result.isVpn},
  isProxy: ${result.isProxy},
  isTor: ${result.isTor || 'false'},
  threatScore: ${result.threatScore},
  recommendedAction: "${result.recommendedAction}"
}`}
                                </pre>
                            </div>
                            <div className="mt-4 p-4 rounded-md" 
                                 style={{
                                    backgroundColor: result.threatScore! >= 70 ? '#FEF2F2' : 
                                                  result.threatScore! >= 30 ? '#FFFBEB' : '#F0FDF4',
                                    borderLeft: `4px solid ${
                                        result.threatScore! >= 70 ? '#DC2626' : 
                                        result.threatScore! >= 30 ? '#F59E0B' : '#10B981'
                                    }`
                                 }}>
                                <p className="font-medium">
                                    {result.threatScore! >= 70 ? '⚠️ High Risk' : 
                                     result.threatScore! >= 30 ? '⚠️ Medium Risk' : '✅ Low Risk'}
                                </p>
                                <p className="text-sm text-gray-600 mt-1">
                                    {result.recommendedAction === 'allow' ? 'This IP appears to be safe.' : 
                                     result.recommendedAction === 'warn' ? 'Exercise caution with this IP.' :
                                     'This IP has been flagged as potentially risky.'}
                                </p>
                            </div>
                        </div>
                    )}
                </div>

                <div className="grid md:grid-cols-3 gap-6">
                    <div className="bg-white p-6 rounded-lg shadow-md">
                        <h3 className="text-xl font-semibold mb-3">Documentation</h3>
                        <p className="text-gray-600 mb-4">Explore our comprehensive documentation to learn how to integrate with our API.</p>
                        <Link href="/docs" className="text-blue-500 hover:underline">View Docs →</Link>
                    </div>
                    <div className="bg-white p-6 rounded-lg shadow-md">
                        <h3 className="text-xl font-semibold mb-3">Pricing</h3>
                        <p className="text-gray-600 mb-4">Choose the plan that fits your needs. Start for free and upgrade as you grow.</p>
                        <Link href="/pricing" className="text-blue-500 hover:underline">View Plans →</Link>
                    </div>
                    <div className="bg-white p-6 rounded-lg shadow-md">
                        <h3 className="text-xl font-semibold mb-3">Dashboard</h3>
                        <p className="text-gray-600 mb-4">Manage your API keys, view usage statistics, and more in your dashboard.</p>
                        <Link href="/dashboard" className="text-blue-500 hover:underline">Go to Dashboard →</Link>
                    </div>
                </div>
            </div>
        </div>
    );
}
