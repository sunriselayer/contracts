#!/bin/bash
SCRIPT_DIR=$(cd $(dirname $0); pwd)
RUSTFLAGS='-C link-arg=-s' cargo wasm