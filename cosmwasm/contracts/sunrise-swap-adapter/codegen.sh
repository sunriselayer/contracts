#!/bin/bash

cargo run schema
cosmwasm-ts-codegen generate \
    --plugin client \
    --schema ./schema \
    --out ./ts \
    --name sunrise-swap-adapter \
    --no-bundle