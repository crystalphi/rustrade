#!/bin/bash
cargo build --release

printf "SetSelection\n" > /tmp/command.txt 
cat selection.json >> /tmp/command.txt
printf "\n" >> /tmp/command.txt
printf "EndSelection\n" >> /tmp/command.txt
cat /tmp/command.txt | target/release/rustrade stream
