import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    // Unit tests only by default (quick)
    include: ['tests/unit/**/*.test.{ts,tsx}'],
    environment: 'node',
    testTimeout: 10000,
  },
});
