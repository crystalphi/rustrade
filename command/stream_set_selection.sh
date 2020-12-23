#!/bin/bash
cargo build

printf "SetSelection\n" > /tmp/command.txt 
cat selection.json >> /tmp/command.txt
printf "\n" >> /tmp/command.txt
printf "EndSelection\n" >> /tmp/command.txt
cat /tmp/command.txt | target/debug/rustrade stream
