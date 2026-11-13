#!/bin/sh
# Refresh the bundled HTML reference without changing the canonical design kit.
set -eu
app_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
repo_root=$(CDPATH= cd -- "$app_root/../.." && pwd)
mkdir -p "$app_root/assets/reference"
cp -R "$repo_root/design/." "$app_root/assets/reference/"
