import { getAccessToken } from "@calimero-network/calimero-client";

// Values from demo-blockchain-integrations/app as a reference
// export const APP_URL = 'app-url';
// export const CONTEXT_IDENTITY = 'context-identity';
// export const CONTEXT_ID = 'context-id';
// export const APPLICATION_ID = 'application-id';
export const NODE_URL_LOCAL_STORAGE_KEY = "NODE_URL";
export const CONTEXT_EXECUTOR_IDENTITY_LOCAL_STORAGE_KEY =
  "CONTEXT_EXECUTOR_IDENTITY";
export const CONTEXT_ID_LOCAL_STORAGE_KEY = "CONTEXT_ID";
export const APPLICATION_ID_LOCAL_STORAGE_KEY = "APPLICATION_ID";

/**
 * Retrieves the node URL from localStorage.
 *
 * This is a renamed version of the `getStorageAppEndpointKey` function from the
 * demo-blockchain-integrations/app repository. The name has been changed to better
 * reflect its purpose: retrieving a node URL from localStorage.
 *
 * @returns {string | null} The stored node URL if it exists and is valid, null otherwise
 *
 * @throws {Error} Catches and logs any JSON parsing or localStorage access errors
 *
 */
export const getNodeUrlFromLocalStorage = (): string | null => {
  try {
    if (typeof window !== "undefined" && window.localStorage) {
      const storedValue: string | null = localStorage.getItem(
        NODE_URL_LOCAL_STORAGE_KEY,
      );
      if (storedValue) {
        const url: string = JSON.parse(storedValue);
        if (url && url.length > 0) {
          return url;
        }
      }
    }
    return null;
  } catch (e) {
    console.error(e);
    return null;
  }
};

/**
 * Sets the node URL in localStorage.
 *
 * This is a renamed version of the `setStorageAppEndpointKey` (see above)
 *
 * @param {string} url - The node URL to store in localStorage
 */
export const setNodeUrlToLocalStorage = (url: string) => {
  localStorage.setItem(NODE_URL_LOCAL_STORAGE_KEY, JSON.stringify(url));
};

/**
 * Former clearStorageAppEndpoint
 * Clears the node URL from localStorage.
 *
 * @throws {Error} Catches and logs any JSON parsing or localStorage access errors
 */
export const clearNodeUrlFromLocalStorage = () => {
  localStorage.removeItem(NODE_URL_LOCAL_STORAGE_KEY);
};

/**
 * Former getStorageExecutorPublicKey
 * Retrieves the public key of the context executor identity from localStorage.
 *
 * @returns {string | null} The stored public key if it exists and is valid, null otherwise
 *
 * @throws {Error} Catches and logs any JSON parsing or localStorage access errors
 */
export const getPublicKeyOfContextExecutorIdentityFromLocalStorage = () => {
  try {
    if (typeof window !== "undefined" && window.localStorage) {
      const storedValue: string | null = localStorage.getItem(
        CONTEXT_EXECUTOR_IDENTITY_LOCAL_STORAGE_KEY,
      );
      if (storedValue) {
        return JSON.parse(storedValue);
      }
    }
    return null;
  } catch (e) {
    console.error(e);
    return null;
  }
};

/**
 * Former getStorageContextId
 * Retrieves the context ID from localStorage.
 *
 * @returns {string | null} The stored context ID if it exists and is valid, null otherwise
 *
 * @throws {Error} Catches and logs any JSON parsing or localStorage access errors
 */
export const getContextIdFromLocalStorage = (): string | null => {
  try {
    if (typeof window !== "undefined" && window.localStorage) {
      const storedValue: string | null = localStorage.getItem(
        CONTEXT_ID_LOCAL_STORAGE_KEY,
      );
      if (storedValue) {
        return JSON.parse(storedValue);
      }
    }
    return null;
  } catch (e) {
    console.error(e);
    return null;
  }
};

/**
 * Former setStorageContextId
 * Sets the context ID in localStorage.
 *
 * @param {string} contextId - The context ID to store in localStorage
 */
export const setContextIdToLocalStorage = (contextId: string) => {
  localStorage.setItem(CONTEXT_ID_LOCAL_STORAGE_KEY, JSON.stringify(contextId));
};

/**
 * Former clearStorageContextId
 * Clears the context ID from localStorage.
 *
 * @throws {Error} Catches and logs any JSON parsing or localStorage access errors
 */
export const clearContextIdFromLocalStorage = () => {
  localStorage.removeItem(CONTEXT_ID_LOCAL_STORAGE_KEY);
};

/**
 * Former getStorageApplicationId
 * Retrieves the application ID from localStorage.
 *
 * @returns {string | null} The stored application ID if it exists and is valid, null otherwise
 *
 * @throws {Error} Catches and logs any JSON parsing or localStorage access errors
 */
export const getApplicationIdFromLocalStorage = (): string | null => {
  try {
    if (typeof window !== "undefined" && window.localStorage) {
      const storedValue: string | null = localStorage.getItem(
        APPLICATION_ID_LOCAL_STORAGE_KEY,
      );
      if (storedValue) {
        return JSON.parse(storedValue);
      }
    }
    return null;
  } catch (e) {
    console.error(e);
    return null;
  }
};

/**
 * Former setStorageApplicationId
 * Sets the application ID in localStorage.
 *
 * @param {string} applicationId - The application ID to store in localStorage
 */
export const setApplicationIdToLocalStorage = (applicationId: string) => {
  localStorage.setItem(
    APPLICATION_ID_LOCAL_STORAGE_KEY,
    JSON.stringify(applicationId),
  );
};

/**
 * Former clearStorageApplicationId
 * Clears the application ID from localStorage.
 *
 * @throws {Error} Catches and logs any JSON parsing or localStorage access errors
 */
export const clearApplicationIdFromLocalStorage = () => {
  localStorage.removeItem(APPLICATION_ID_LOCAL_STORAGE_KEY);
};
/**
 * Interface representing the decoded JWT payload structure
 */
export interface JsonWebToken {
  context_id: string;
  token_type: string;
  exp: number;
  sub: string;
  executor_public_key: string;
}

/**
 * Decodes and returns the JWT payload as an object.
 *
 * This function takes the JWT from localStorage (via getAccessToken),
 * splits it into its components, and decodes the payload section.
 *
 * @returns {JsonWebToken | null} The decoded JWT payload or null if no token exists
 * @throws {Error} If the JWT format is invalid (doesn't have 3 parts)
 *
 * @example
 * const jwtPayload = getJWTObject();
 * if (jwtPayload) {
 *   console.log(jwtPayload.context_id);
 * }
 */
export const getJWTObject = (): JsonWebToken | null => {
  const token = getAccessToken();
  if (!token) return null;

  const parts = token.split(".");
  if (parts.length !== 3) {
    throw new Error("Invalid JWT token");
  }

  const payload = JSON.parse(atob(parts[1]));
  return payload;
};

/**
 * Retrieves the raw JWT token string from localStorage.
 *
 * This is a wrapper around getAccessToken for consistency and clarity.
 *
 * @returns {string | null} The raw JWT token or null if none exists
 */
export const getJWT = (): string | null => {
  return getAccessToken();
};
