// This file contains type declarations for the SDK

// Extend the global ErrorConstructor interface to include captureStackTrace
declare global {
  interface ErrorConstructor {
    captureStackTrace(targetObject: object, constructorOpt?: Function): void;
  }

  // Add any other global type declarations needed for your SDK
  interface Window {
    // Add any browser-specific globals here if needed
  }
}

// This export is needed to make this a module
export {};
