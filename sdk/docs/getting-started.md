# Getting Started

This guide will help you quickly get up and running with the IP Intelligence & Threat Detection SDK.

## Prerequisites

- Node.js 14.x or later
- npm or yarn package manager
- A valid API key from your service provider

## Installation

Install the SDK using npm or yarn:

```bash
# Using npm
npm install infralock

# Or using yarn
yarn add infralock
```

## Basic Usage

### Importing the SDK

```typescript
// Using ES modules
import { IPIntelClient } from 'infralock';

// Or using CommonJS
// const { IPIntelClient } = require('infralock');
```

### Initializing the Client

Create a new instance of the `IPIntelClient` with your API key:

```typescript
const client = new IPIntelClient('your-api-key');
```

### Making Your First Request

Analyze an IP address for threats and gather intelligence:

```typescript
async function analyzeIp(ipAddress: string) {
  try {
    const result = await client.analyze(ipAddress);
    
    // Basic IP information
    console.log(`Analysis for ${result.ip}:`);
    console.log(`- Location: ${result.city}, ${result.country}`);
    
    // Threat assessment
    console.log(`- Threat Score: ${result.threatScore}/100`);
    if (result.threatScore > 70) {
      console.warn('⚠️  High risk IP detected!');
      console.log('Threat details:', result.threatDetails.join(', '));
    }
    
    // Network information
    if (result.asnInfo) {
      console.log(`- ASN: ${result.asnInfo.autonomous_system_number}`);
      console.log(`- Organization: ${result.asnInfo.autonomous_system_organization}`);
    }
    
    // Client information (when available)
    if (result.clientInfo) {
      console.log('\nClient Information:');
      console.log(`- Device: ${result.clientInfo.device.model} (${result.clientInfo.device.type})`);
      console.log(`- Browser: ${result.clientInfo.browser.name} ${result.clientInfo.browser.version}`);
      console.log(`- OS: ${result.clientInfo.os.name} ${result.clientInfo.os.version}`);
    }
    
    // Security indicators
    const securityIndicators = [
      result.isVpn && 'VPN',
      result.isProxy && 'Proxy',
      result.isTor && 'Tor'
    ].filter(Boolean);
    
    if (securityIndicators.length > 0) {
      console.log('\n⚠️  Detected:', securityIndicators.join(', '));
    }
    
    return result;
  } catch (error) {
    console.error('Analysis failed:', error.message);
    throw error;
  }
}

// Example usage
await analyzeIp('8.8.8.8');
```

## Key Features

### Threat Detection

```typescript
const result = await client.analyze('185.220.101.4'); // Example Tor exit node

console.log('Threat Assessment:');
console.log(`- Score: ${result.threatScore}/100`);
console.log(`- Is Tor: ${result.isTor}`);
console.log(`- Is VPN: ${result.isVpn}`);
console.log(`- Is Proxy: ${result.isProxy}`);
console.log(`- Recommended Action: ${result.recommendedAction}`);
```

### Client Fingerprinting

```typescript
const result = await client.analyze('client-ip-address');

if (result.clientInfo) {
  console.log('Client Fingerprint:');
  console.log('- Device:', result.clientInfo.device);
  console.log('- Browser:', result.clientInfo.browser);
  console.log('- OS:', result.clientInfo.os);
  console.log('- User Agent:', result.clientInfo.userAgent);
  console.log('- Timestamp:', result.clientInfo.timestamp);
}
```

### Batch Processing

```typescript
const ips = [
  '8.8.8.8',
  '1.1.1.1',
  '185.220.101.4', // Known Tor exit node
  'invalid-ip',    // Will trigger a validation error
  'client-ip'      // Will show client info if available
];

const results = await client.batchAnalyze(ips);

// Process results
results.forEach((result, index) => {
  console.log(`\nResult for ${ips[index]}:`);
  
  if (result instanceof Error) {
    console.error('❌ Error:', result.message);
  } else {
    const indicators = [
      result.isVpn && 'VPN',
      result.isProxy && 'Proxy',
      result.isTor && 'Tor'
    ].filter(Boolean);
    
    console.log(`✅ ${result.city}, ${result.country}`);
    console.log(`   Risk: ${result.threatScore}/100`);
    
    if (indicators.length > 0) {
      console.log(`   ⚠️  Detected: ${indicators.join(', ')}`);
    }
    
    if (result.clientInfo) {
      console.log(`   Client: ${result.clientInfo.device.type} on ${result.clientInfo.os.name}`);
    }
  }
});
```

## Error Handling

The SDK provides comprehensive error handling with the following error types:

- `ApiError`: For API-related errors (4xx, 5xx responses)
- `NetworkError`: For network-related issues
- `ValidationError`: For input validation failures

### Example Error Handling

```typescript
try {
  await client.analyze('invalid-ip');
} catch (error) {
  if (error.name === 'ValidationError') {
    console.error('Invalid input:', error.message);
  } else if (error.name === 'ApiError') {
    console.error(`API Error (${error.statusCode}):`, error.message);
  } else if (error.name === 'NetworkError') {
    console.error('Network issue:', error.message);
  } else {
    console.error('Unexpected error:', error);
  }
}
```

## Next Steps

- [Configuration Guide](./configuration.md) - Learn about all available options
- [API Reference](./api.md) - Complete API documentation
- [Error Handling Guide](./error-handling.md) - Detailed error handling information

## Troubleshooting

### Common Issues

1. **Invalid API Key**
   - Ensure you're using a valid API key
   - Check for typos or extra spaces
   - Verify your account is active and has sufficient credits

2. **Network Issues**
   - Check your internet connection
   - Verify the API endpoint is accessible from your network
   - Check if a firewall is blocking the requests

3. **Rate Limiting**
   - The SDK automatically handles rate limits with retries
   - For 429 errors, the SDK will respect the `Retry-After` header
   - Consider implementing request queuing for high-volume applications

### Getting Help

If you encounter any issues, please:
1. Check the [error handling](#error-handling) section for details
2. Enable debug logging for more information
3. Contact support with the request ID from the error message

## Support

For additional help, please contact support@your-org.com or visit our [documentation website](https://docs.your-org.com).
