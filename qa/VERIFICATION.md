# Demo verification

Verified October 26, 2026.

- `cargo test --locked`: two catalog tests passed (unique IDs, variant/base consistency, verified menu prices and portions).
- `trunk build --release --locked`: passed. Optimized WebAssembly approximately 290 KB before HTTP compression.
- `qa/browser-check.js`: 25 browser checks passed in Chromium against the release build.
- Checked category filtering; variant selection; mixed-cart totals; quantity changes; removal; note inclusion; clipboard output; Escape dismissal; content editing; price changes; availability; local save/reload; reset; mobile navigation; mobile order drawer; FAQ expansion; and image loading.
- No horizontal page overflow at 1440, 768, 390, or 320 pixels.
- No browser JavaScript or WebAssembly runtime errors during the automated checks.
- Desktop full-page and mobile viewport screenshots reviewed for layout and imagery.

The browser-check file exports an async function taking a Playwright `page`. Run it after navigating to a fresh preview. It changes browser-local demo state during testing, restores the original content, and never submits an order or contacts the business.

The content studio is a browser-local prototype. Payment processing, shared CMS publishing, real inventory, and pickup scheduling are intentionally outside this GitHub Pages demo.
