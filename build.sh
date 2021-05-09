#!/bin/bash
OUTPUT_DIR="./lv2xdo.lv2"
INSTALL_DIR="$HOME/.lv2"
set -e

# BUILD
cargo build --release

# ASSEMBLE OUTPUT
cp -v ./target/release/liblv2xdo.so "${OUTPUT_DIR}"
