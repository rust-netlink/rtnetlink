#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root_dir"

cargo public-api -sss --color never > tools/public-api.txt
