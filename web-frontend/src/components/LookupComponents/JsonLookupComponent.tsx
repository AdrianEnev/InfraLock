import { IpLookupResult } from '@interfaces/ApiInterfaces'
import React from 'react'

function JsonLookupComponent({ result }: { result: IpLookupResult }) {
  return (
    <div className="space-y-4">
    <div className="bg-gray-800 rounded-md p-4 font-mono text-sm text-gray-200 overflow-x-auto min-h-[500px] max-h-[50vh] overflow-y-auto">
        <pre className="whitespace-pre">
{
`{
    ip: "${result.ip}",
    country: "${result.country || 'Not Found'}",
    city: "${result.city || 'Not Found'}",
    location: {
    latitude: ${result.latitude || 'Not Found'},
    longitude: ${result.longitude || 'Not Found'},
    asnInfo: {
        autonomous_system_number: ${result.asnInfo?.autonomous_system_number || 'Not Found'},
        autonomous_system_organization: "${result.asnInfo?.autonomous_system_organization || 'Not Found'}"
    },
    isVpn: ${result.isVpn || false},
    isProxy: ${result.isProxy || false},
    isTor: ${result.isTor || false},
    threatScore: ${result.threatScore || 0},
    threatDetails: ${result.threatDetails?.length ? `["${result.threatDetails.join(', ')}"]` : '[]'},
    recommendedAction: "${result.recommendedAction || 'none'}",
    proxyType: "${result.proxyType || 'Not Found'}",
    userAgent: "${result.clientInfo?.userAgent || 'Not Found'}", 
    browser: {
        name: ${result.clientInfo?.browser.name || 'Not Found'},
        version: ${result.clientInfo?.browser.version || 'Not Found'}
    },
    os: {
        name: ${result.clientInfo?.os.name || 'Not Found'},
        version: ${result.clientInfo?.os.version || 'Not Found'}
    },
    device: {
        model: ${result.clientInfo?.device.model || 'Not Found'},
        type: ${result.clientInfo?.device.type || 'Not Found'}
    },
    engine: ${result.clientInfo?.engine || 'Not Found'},
    cpu: ${result.clientInfo?.cpu || 'Not Found'},
    timestamp: ${result.clientInfo?.timestamp || 'Not Found'},
}`
}                               </pre>
    </div>

    
</div>
  )
}

export default JsonLookupComponent