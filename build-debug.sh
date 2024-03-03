#!/bin/bash -e
cd crates/web
trunk build --release
cd ../..

cd crates/backend
cargo build
cd ../..

