export const env = {
  websocket: {
    url: process.env.NEXT_PUBLIC_WEBSOCKET_URL ?? 'ws://localhost:3000',
  },
} as const;

// Type check to ensure all environment variables are defined
Object.entries(env).forEach(([key, value]) => {
  if (value === undefined) {
    throw new Error(`Environment variable ${key} is undefined`);
  }
});
