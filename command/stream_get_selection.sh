#!/bin/bash
cargo build --release
echo "GetSelection" | target/release/rustrade stream
