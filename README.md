# My personal website

URL: https://javier.fermyon.app/

## Requirements

- Rust
- Node.js
- Spin v2.x

## Installation

Install Rust dependencies:

```sh
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
cargo install cargo-leptos
```

Install Node.js dependencies:

```sh
npm install -g sass
```

## Development deployment

Build and run

```sh
spin watch
```

## Spin variables for Admin Panel

| Name                     | Type    | Default |
| ------------------------ | ------- | ------- |
| admin_enable             | Boolean | false   |
| admin_password           | String  | admin   |
| admin_session_key_length | Integer | 64      |
| admin_username           | String  | admin   |
