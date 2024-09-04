#!/bin/bash

mkdir -p ./storage/hash2torrent/lib/ ./storage/hash2torrent/log/ ./storage/hash2torrent/etc/

docker run -it \
    --env USER_ID"$(id -u)" \
    --publish 3000:3000/tcp \
    --publish 51000-51010 \
    --volume /var/lib/torrust/hash2torrent:/var/lib/torrust/hash2torrent:rw \
    torrust-hash2torrent:release
