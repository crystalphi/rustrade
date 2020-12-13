#!/bin/bash
cargo build --release
printf "SetSelection\n" && cat selection.json && printf "\n" && printf "EndSelection\n" | target/release/rustrade stream
