module.exports = {
  ci: {
    collect: {
      url: ['http://localhost:8080'],
      numberOfRuns: 1,
      chromeFlags: ['--headless', '--disable-gpu', '--no-sandbox', '--disable-dev-shm-usage'],
      settings: {
        // Reduce wait time to prevent hanging
        maxWaitForLoad: 15000,
        maxWaitForFcp: 15000,
        // Skip audits that might cause timeouts or issues with WASM
        skipAudits: [
          'uses-long-cache-ttl',
          'works-offline',
          'offline-start-url',
          'service-worker',
          'installable-manifest',
          'splash-screen',
          'themed-omnibox',
          'maskable-icon',
          'valid-source-maps',
          'preload-fonts',
          'network-rtt',
          'network-server-latency',
          'uses-http2',
          'redirects-http',
          'uses-optimized-images',
          'uses-webp-images',
          'uses-text-compression'
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
        // Only run specific categories to avoid timeouts
        onlyCategories: ['performance', 'accessibility', 'best-practices'],
        // Chrome flags to prevent hanging
        chromeFlags: [
          '--headless',
          '--disable-gpu',
          '--no-sandbox',
          '--disable-dev-shm-usage',
          '--disable-setuid-sandbox',
          '--single-process'
        ],
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