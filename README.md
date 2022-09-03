# Alternativka Progress

Webpage with my alternative civilian service progress. Built with Rust ([Yew](https://yew.rs/)) and [TailwindCSS](https://tailwindcss.com/)!

<img width="909" alt="Screenshot 2022-09-03 at 18 48 14" src="https://user-images.githubusercontent.com/75225148/188278246-ef3ef022-a63f-4392-b0d3-a04075fc31a3.png">

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
