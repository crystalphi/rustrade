#!/bin/bash
cargo build

printf "SetSelection" > /tmp/command.txt 
printf "\n" >> /tmp/command.txt
cat selection.json >> /tmp/command.txt
printf "\n" >> /tmp/command.txt
printf "EndSelection" >> /tmp/command.txt
printf "\n" >> /tmp/command.txt
printf "Import" >> /tmp/command.txt
printf "\n" >> /tmp/command.txt
printf "Plot" >> /tmp/command.txt
printf "\n" >> /tmp/command.txt
printf "Terminate" >> /tmp/command.txt
printf "\n" >> /tmp/command.txt
cat /tmp/command.txt | target/debug/rustrade stream
