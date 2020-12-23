#!/bin/bash
cargo build --release

echo "Terminate" | target/release/rustrade stream
