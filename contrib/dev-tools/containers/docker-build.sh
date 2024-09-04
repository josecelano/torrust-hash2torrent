#!/bin/bash

echo "Building docker image ..."

# Enable more verbosity with: #docker build --progress=plain ...
docker build --target release --tag torrust-hash2torrent:release --file Containerfile .
