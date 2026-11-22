#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root_dir"

forbidden_modules='rtnetlink::(link|route|traffic_control|addr|addrlabel|neighbour|rule|ns|connection)::'

api_file="$(mktemp)"
trap 'rm -f "$api_file"' EXIT

cargo public-api -sss --color never > "$api_file"

if grep -Eq "$forbidden_modules" "$api_file"; then
    echo 'error: public API exposes items through private modules:' >&2
    grep -E "$forbidden_modules" "$api_file" >&2
    exit 1
fi

if [[ "${CHECK_PUBLIC_API_SNAPSHOT:-0}" == "1" ]]; then
    if ! diff -u tools/public-api.txt "$api_file"; then
        echo 'error: public API snapshot is stale; run tools/update-public-api.sh' >&2
        exit 1
    fi
fi
