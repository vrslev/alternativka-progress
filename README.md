# Alternativka Progress

Webpage with my alternative civilian service progress. Built with Rust ([Yew](https://yew.rs/)) and [TailwindCSS](https://tailwindcss.com/)!

### Installation

Install Rust if you don't have it yet.

Add WASM target:

```bash
rustup target add wasm32-unknown-unknown
```

Install [Trunk](https://trunkrs.dev/):

```bash
cargo install trunk wasm-bindgen-cli
```

That's it, we're done!

### Running

```bash
trunk serve
```

### Release

```bash
trunk build --release
```
