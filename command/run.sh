#!/bin/bash
cargo build --release
target/release/rustrade $1
