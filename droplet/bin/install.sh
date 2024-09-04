#!/bin/bash

if ! [ -f "./.env" ]; then
	echo "Creating compose .env './.env'"
	cp .env.production .env
fi

## Proxy

mkdir -p ./storage/proxy/etc/nginx-conf
mkdir -p ./storage/proxy/webroot
mkdir -p ./storage/dhparam

if ! [ -f "./storage/proxy/etc/nginx-conf/nginx.conf" ]; then
	echo "Creating proxy config file: './storage/proxy/etc/nginx-conf/nginx.conf'"
	cp ./share/container/default/config/nginx.conf ./storage/proxy/etc/nginx-conf/nginx.conf
fi

## Certbot

mkdir -p ./storage/certbot/etc
mkdir -p ./storage/certbot/lib

## hash2torrent

mkdir -p ./storage/hash2torrent/lib/session
mkdir -p ./storage/hash2torrent/lib/torrents
