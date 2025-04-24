#!/bin/bash
set -e

echo "📦 Building release binaries..."
mkdir -p bin

services=(renderer api-gateway storage reporting jpk)

for svc in "${services[@]}"
do
    echo "🔨 Building $svc..."
    cargo build -p $svc --release
    cp target/release/$svc bin/$svc
done

echo "✅ All binaries built and placed in ./bin/"
