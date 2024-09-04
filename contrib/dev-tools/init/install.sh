#!/bin/bash

# Check if USER_ID is passed as an argument
if [ -z "$1" ]; then
    echo "Usage: sudo $0 <USER_ID>"
    exit 1
fi

USER_ID=$1

# Create directories
mkdir -p /var/lib/torrust/hash2torrent/session
mkdir -p /var/lib/torrust/hash2torrent/torrents

# Change ownership to the current user ID
chown -R "${USER_ID}:${USER_ID}" /var/lib/torrust/hash2torrent

# Set permissions
chmod -R 2770 /var/lib/torrust/hash2torrent
