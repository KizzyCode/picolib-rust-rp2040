#!/bin/sh
set -eu

# Go into basedir
BASEDIR=`dirname $0`
cd "$BASEDIR"

# Call bindgen
bindgen "pico_delegates.h" \
    --use-core --no-layout-tests \
    --allowlist-function "pico_.*" \
    --allowlist-var "pico_.*" \
    --output "../src/delegates/bindgen.rs"
