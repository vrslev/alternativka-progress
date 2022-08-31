# Alternativka Progress

Webpage with my alternative civilian service progress. Built with Rust ([Yew](https://yew.rs/)) and [TailwindCSS](https://tailwindcss.com/)!

<img width="1097" alt="screenshot" src="https://user-images.githubusercontent.com/75225148/187735298-6b755856-71cd-486e-a2ca-23a935b47b39.png">

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
