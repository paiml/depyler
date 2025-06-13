module.exports = {
  ci: {
    collect: {
      url: ['http://localhost:8080'],
      numberOfRuns: 1,
      settings: {
        // Don't wait for resources to load
        maxWaitForLoad: 45000,
        // Skip some audits that might cause timeouts
        skipAudits: [
          'uses-long-cache-ttl',
          'works-offline',
          'offline-start-url'
        ],
        // Use desktop configuration for faster runs
        preset: 'desktop',
        // Disable throttling for CI
        throttlingMethod: 'provided',
        throttling: {
          rttMs: 0,
          throughputKbps: 0,
          cpuSlowdownMultiplier: 1,
        },
        // Disable screenshot generation to speed up
        disableFullPageScreenshot: true,
      }
    },
    assert: {
      preset: 'lighthouse:no-pwa',
      assertions: {
        'categories:performance': ['error', {minScore: 0.9}],
        'categories:accessibility': ['error', {minScore: 1}],
        'categories:best-practices': ['warn', {minScore: 1}],
        'categories:seo': ['warn', {minScore: 0.8}],
        // Allow some warnings for CI environment
        'errors-in-console': 'off',
        'csp-xss': 'off',
        'is-crawlable': 'off',
        'robots-txt': 'off',
      }
    },
    upload: {
      target: 'temporary-public-storage',
    },
  },
};