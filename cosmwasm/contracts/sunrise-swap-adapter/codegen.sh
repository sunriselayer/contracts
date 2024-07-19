#!/bin/bash
SCRIPT_DIR=$(cd $(dirname $0); pwd)

cargo run schema
cosmwasm-ts-codegen generate \
    --plugin client \
    --schema ./schema \
    --out ./ts \
    --name sunrise-swap-adapter \
    --no-bundle