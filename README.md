# The Floured Apron — website concept

A working Leptos redesign preview for a home bakery in Florence, Arizona.

**Demo:** https://wolfbreed1829.github.io/the-floured-apron-demo/

## Included

- Responsive storefront with existing branding and documented stock photography.
- Full menu grounded in the bakery's public menu, including portion and flavor selections.
- Order builder with quantity controls, estimated totals, and a copyable request draft. Nothing is sent, paid for, or reserved.
- Content studio for homepage copy, announcements, base prices, and availability. Edits update immediately and can be saved in browser local storage. This is an interaction prototype, not a shared or authenticated production CMS.
- Native accessible dialogs, keyboard navigation, responsive imagery, reduced-motion support, and a no-JavaScript fallback.
- GitHub Actions build and GitHub Pages deployment. Search indexing is discouraged with robots.txt and a noindex meta tag.

## Local development

Requires Rust 1.94.1 or newer and Trunk 0.21.14.

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --version 0.21.14 --locked
trunk serve --address 127.0.0.1 --port 8080
```

Validation and production build:

```sh
cargo test --locked
trunk build --release --locked --public-url /the-floured-apron-demo/
```

`content.json` holds the bundled content and product data. `src/main.rs` contains the Leptos components and interactive behavior. `style.css` contains the responsive design system.

## Content and photography

Product prices and portions were checked against https://theflouredapronaz.com/ on October 26, 2026. Product descriptions and marketing copy are proposed copy for review. No specific lead times, business hours, pickup address, bake dates, stock counts, testimonials, or allergen guarantees have been invented. Availability toggles in the studio are illustrative; current availability must be confirmed with the bakery.

Stock photography is disclosed on the page. See [ASSET_SOURCES.md](ASSET_SOURCES.md) for image provenance. Bakery branding remains the property of its owner. Photos should be replaced with owner-supplied product and process photography before launch.

## Production follow-up

GitHub Pages serves a static WebAssembly application. A production CMS would require authenticated editing, a server/database, durable media uploads, revisions, and publishing. A server-rendered Leptos deployment can use the same content model and visual components. Orders, payment, capacity, and pickup scheduling need separate transactional backend validation.

The preview deliberately retains the current website as the route to real bakery contact. The existing business domain is unchanged.
