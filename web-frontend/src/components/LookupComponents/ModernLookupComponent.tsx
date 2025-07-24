import { IpLookupResult } from '@interfaces/ApiInterfaces'
import React from 'react'

function ModernLookupComponent({result}: {result: IpLookupResult}) {
    return (
        <div className="bg-white rounded-lg shadow-md p-6">
            <h2 className="text-2xl font-semibold mb-4">IP Lookup Results</h2>
            <div className="space-y-6">
                {/* IP Information Section */}
                <div className="space-y-4">
                    <h3 className="text-lg font-medium text-gray-700 border-b pb-2">IP Information</h3>
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <h4 className="text-sm font-medium text-gray-500">IP Address</h4>
                            <p className="text-gray-900 font-mono">{result.ip}</p>
                        </div>
                        <div>
                            <h4 className="text-sm font-medium text-gray-500">Location</h4>
                            <p className="text-gray-900">
                                {result.city || "Unknown City"}, {result.country || "Unknown Country"}
                            </p>
                            {result.latitude && result.longitude && (
                                <p className="text-sm text-gray-500">
                                    {result.latitude}, {result.longitude}
                                </p>
                            )}
                        </div>
                        <div>
                            <h4 className="text-sm font-medium text-gray-500">ASN Information</h4>
                            {result.asnInfo?.autonomous_system_number ? (
                                <div>
                                    <p className="text-gray-900">
                                        AS{result.asnInfo.autonomous_system_number}
                                    </p>
                                    {result.asnInfo.autonomous_system_organization && (
                                        <p className="text-gray-600">
                                            {result.asnInfo.autonomous_system_organization}
                                        </p>
                                    )}
                                </div>
                            ) : (
                                <p className="text-gray-500">Not available</p>
                            )}
                        </div>
                        <div>
                            <h4 className="text-sm font-medium text-gray-500">Threat Score</h4>
                            <div className="flex items-center">
                                <div className="w-full bg-gray-200 rounded-full h-4">
                                    <div 
                                        className={`h-4 rounded-full ${result.threatScore > 70 ? 'bg-red-500' : result.threatScore > 30 ? 'bg-yellow-500' : 'bg-green-500'}`}
                                        style={{ width: `${result.threatScore}%` }}
                                    />
                                </div>
                                <span className="ml-2 text-gray-700 font-medium">
                                    {result.threatScore}%
                                </span>
                            </div>
                        </div>
                    </div>
                </div>

                {/* Threat Indicators Section */}
                <div>
                    <h3 className="text-lg font-medium text-gray-700 border-b pb-2 mb-2">Threat Indicators</h3>
                    {result.threatDetails && result.threatDetails.length > 0 ? (
                        <ul className="list-disc list-inside space-y-1">
                            {result.threatDetails.map((detail, index) => (
                                <li key={index} className="text-gray-700">{detail}</li>
                            ))}
                        </ul>
                    ) : (
                        <p className="text-gray-500">No threat indicators detected</p>
                    )}
                </div>

                {/* Proxy/VPN/Tor Detection */}
                <div>
                    <h3 className="text-lg font-medium text-gray-700 border-b pb-2 mb-2">Connection Analysis</h3>
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div className={`p-3 rounded-md ${result.isVpn ? 'bg-red-50' : 'bg-green-50'}`}>
                            <p className="font-medium">VPN/Data Center</p>
                            <p className={result.isVpn ? 'text-red-600' : 'text-green-600'}>
                                {result.isVpn ? 'Detected' : 'Not Detected'}
                            </p>
                        </div>
                        <div className={`p-3 rounded-md ${result.isProxy ? 'bg-red-50' : 'bg-green-50'}`}>
                            <p className="font-medium">Proxy</p>
                            <p className={result.isProxy ? 'text-red-600' : 'text-green-600'}>
                                {result.isProxy ? 'Detected' : 'Not Detected'}
                            </p>
                            {result.proxyType && (
                                <p className="text-xs text-gray-500 mt-1">Type: {result.proxyType}</p>
                            )}
                        </div>
                        <div className={`p-3 rounded-md ${result.isTor ? 'bg-red-50' : 'bg-green-50'}`}>
                            <p className="font-medium">Tor Exit Node</p>
                            <p className={result.isTor ? 'text-red-600' : 'text-green-600'}>
                                {result.isTor ? 'Detected' : 'Not Detected'}
                            </p>
                        </div>
                    </div>
                </div>

                {/* Client Information Section */}
                {result.clientInfo && (
                    <div>
                        <h3 className="text-lg font-medium text-gray-700 border-b pb-2 mb-2">Client Information</h3>
                        <div className="bg-gray-50 p-4 rounded-md">
                            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div>
                                    <h4 className="text-sm font-medium text-gray-500">Browser</h4>
                                    <p className="text-gray-900">
                                        {result.clientInfo.browser.name} {result.clientInfo.browser.version}
                                    </p>
                                </div>
                                <div>
                                    <h4 className="text-sm font-medium text-gray-500">Operating System</h4>
                                    <p className="text-gray-900">
                                        {result.clientInfo.os.name} {result.clientInfo.os.version}
                                    </p>
                                </div>
                                <div>
                                    <h4 className="text-sm font-medium text-gray-500">Device</h4>
                                    <p className="text-gray-900">
                                        {result.clientInfo.device.model} ({result.clientInfo.device.type})
                                    </p>
                                </div>
                                <div>
                                    <h4 className="text-sm font-medium text-gray-500">Engine</h4>
                                    <p className="text-gray-900">{result.clientInfo.engine}</p>
                                </div>
                                <div>
                                    <h4 className="text-sm font-medium text-gray-500">CPU</h4>
                                    <p className="text-gray-900">{result.clientInfo.cpu}</p>
                                </div>
                                <div className="col-span-1 md:col-span-2">
                                    <h4 className="text-sm font-medium text-gray-500">User Agent</h4>
                                    <p className="text-gray-900 text-sm break-all">
                                        {result.clientInfo.userAgent}
                                    </p>
                                </div>
                                <div>
                                    <h4 className="text-sm font-medium text-gray-500">Lookup Time</h4>
                                    <p className="text-gray-900">
                                        {new Date(result.clientInfo.timestamp).toLocaleString()}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                )}
            </div>
        </div>
    )
}

export default ModernLookupComponent