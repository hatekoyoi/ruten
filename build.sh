#!/bin/bash

nasm -f bin -o test/helloworld.bin test/helloworld.asm
cargo run test/helloworld.bin
