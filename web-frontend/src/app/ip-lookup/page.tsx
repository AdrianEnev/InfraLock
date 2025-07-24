'use client'

import { useState } from 'react';
import { useIpLookupCustom } from '@hooks/useIpLookupCustom';
import Link from 'next/link';
import ModernLookupComponent from '@src/components/LookupComponents/ModernLookupComponent';
import JsonLookupComponent from '@src/components/LookupComponents/JsonLookupComponent';
import generateRandomIp from '@src/utils/generateRandomIp';

export default function IpLookupPage() {
    const [ipInput, setIpInput] = useState('');
    const [showJson, setShowJson] = useState(false);
    const { lookupIp, result, isLoading, error, reset } = useIpLookupCustom();

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        lookupIp(ipInput);
    };

    const handleReset = () => {
        setIpInput('');
        reset();
    };

    const toggleJson = () => setShowJson((prev) => !prev);

    return (
        <div className="w-screen min-h-screen overflow-x-hidden">
            <div className="max-w-4xl mx-auto px-4 py-12">
                <div className="text-center mb-8">
                    <h1 className="text-4xl font-bold text-gray-800 mb-2">
                        <p>Specify IP to look up</p>
                    </h1>
                    <p className="text-gray-600">Enter an IP address to look up geolocation and threat information</p>
                    <Link href="/" className="text-blue-500 hover:underline mt-2 inline-block">
                        <p>&larr; Back to Home</p>
                    </Link>
                </div>

                <div className="bg-white rounded-lg shadow-md p-6 mb-8">
                    <form onSubmit={handleSubmit} className="space-y-4">
                        <div>
                            <label htmlFor="ip" className="block text-sm font-medium text-gray-700 mb-1">
                                <p>IP Address</p>
                            </label>
                            <div className="flex space-x-2">
                                <input
                                    type="text"
                                    id="ip"
                                    value={ipInput}
                                    onChange={(e) => setIpInput(e.target.value)}
                                    placeholder="Enter an IP address (e.g., 8.8.8.8)"
                                    className="flex-1 px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                                    disabled={isLoading}
                                />
                                <button
                                    type="submit"
                                    disabled={isLoading || !ipInput}
                                    className="px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    <p>{isLoading ? 'Looking up...' : 'Lookup'}</p>
                                </button>
                                <button
                                    type="button"
                                    onClick={handleReset}
                                    className="px-4 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2"
                                >
                                    <p>Reset</p>
                                </button>
                                {result && (
                                    <button
                                        type="button"
                                        onClick={toggleJson}
                                        className="px-4 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2"
                                    >
                                        <p>{showJson ? 'Modern format' : 'JSON format'}</p>
                                    </button>
                                )}
                                {!result && (
                                    <button
                                        type="button"
                                        onClick={() => {
                                            const randomIp = generateRandomIp();
                                            setIpInput(randomIp);
                                        }}
                                        className="px-4 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2"
                                    >
                                        <p>Random</p>
                                    </button>
                                )}
                            </div>
                        </div>
                    </form>

                    {error && (
                        <div className="mt-4 p-4 bg-red-50 border-l-4 border-red-500">
                            <p className="text-red-700">{error}</p>
                        </div>
                    )}
                </div>

                {result && (
                    <div>
                        {showJson ? (
                            <JsonLookupComponent result={result} />
                        ) : (
                            <ModernLookupComponent result={result} />
                        )}
                    </div>
                )}
            </div>
        </div>
    );
}
