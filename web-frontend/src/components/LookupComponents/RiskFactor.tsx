import React from 'react'
import { IpLookupResult } from '@interfaces/ApiInterfaces'

function RiskFactor({result}: {result: IpLookupResult}) {
    return (
        <div className="mt-4 p-4 rounded-md" 
            style={{
                backgroundColor: (result.threatScore || 0) >= 70 ? '#FEF2F2' : 
                            (result.threatScore || 0) >= 30 ? '#FFFBEB' : '#F0FDF4',
                borderLeft: `4px solid ${
                    (result.threatScore || 0) >= 70 ? '#DC2626' : 
                    (result.threatScore || 0) >= 30 ? '#D97706' : '#16A34A'
                }`
            }}>
            <p className="font-medium">
                {(result.threatScore || 0) >= 70 ? '⚠️ High Risk' : 
                (result.threatScore || 0) >= 30 ? '⚠️ Medium Risk' : '✅ Low Risk'}
            </p>
            <p className="text-sm text-gray-600 mt-1">
                {(result.recommendedAction || 'none') === 'allow' ? 'This IP appears to be safe.' : 
                (result.recommendedAction || 'none') === 'warn' ? 'Exercise caution with this IP.' :
                'This IP has been flagged as potentially risky.'}
            </p>
        </div>
    )
}

export default RiskFactor