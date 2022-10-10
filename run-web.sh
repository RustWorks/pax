#!/bin/sh

set -ex

# Run the `wasm-pack` CLI tool to build and process the Rust wasm file

# Manual command is commented below; note difference in relative manifest path:
# cargo build --manifest-path=piet/Cargo.toml --all-targets --all-features

#cargo build --manifest-path=../../../piet/Cargo.toml --all-targets --all-features
cd pax-chassis-web

# before adding serde: 92.94kb
wasm-pack build --release -d pax-dev-harness-web/dist

# Finally, package everything up using Webpack and start a server so we can
# browse the result
cd pax-dev-harness-web
yarn && yarn serve
