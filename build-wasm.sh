#!/usr/bin/env bash
set -euo pipefail

echo "Building pdq_rs.wasm via Docker..."

docker buildx build \
  --file Dockerfile.componentize \
  --target artifact \
  --output "type=local,dest=dist" \
  --progress=plain \
  .