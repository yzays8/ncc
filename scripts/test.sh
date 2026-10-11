#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."

cargo build

./external/writing-a-c-compiler-tests/test_compiler "$(pwd)/target/debug/ncc" "$@"
