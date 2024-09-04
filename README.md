# Torrust Hash2Torrent

[![Testing](https://github.com/torrust/torrust-hash2torrent/actions/workflows/testing.yaml/badge.svg)](https://github.com/torrust/torrust-hash2torrent/actions/workflows/testing.yaml) [![Container](https://github.com/torrust/torrust-hash2torrent/actions/workflows/container.yaml/badge.svg)](https://github.com/torrust/torrust-hash2torrent/actions/workflows/container.yaml)

A web service to get torrents' metadata from the infohashes.

The API is based on the Rust BitTorrent client [rqbit](<https://github.com/ikatson/rqbit>). The client uses [BEP 9](https://www.bittorrent.org/beps/bep_0009.html) to get the Metadata Files from other peers.

> NOTICE: DHT must be enabled because the client needs to find peers first.

Live demo: <https://hash2torrent.com/>

## Setup

```console
sudo ./contrib/dev-tools/init/install.sh $(id -u)
cargo run
```

### With Docker

Building the image from sources:

```console
sudo ./contrib/dev-tools/init/install.sh $(id -u)
./contrib/dev-tools/containers/docker-build.sh
./contrib/dev-tools/containers/docker-run.sh
```

## Usage

Download the torrent with curl:

```console
curl -o ./ubuntu-23.04-desktop-amd64.iso.torrent http://127.0.0.1:3000/torrents/443c7602b4fde83d1154d6d9da48808418b181b6
```

Or with the browser:

<http://127.0.0.1:3000/torrents/443c7602b4fde83d1154d6d9da48808418b181b6>

> NOTICE: The BitTorrent client may not find the torrent and the HTTP could return a 408 (timeout) error after 10

You can check the API with the health_check endpoint: <http://127.0.0.1:3000/health_check>

## Acknowledgments

[ikatson](<https://github.com/ikatson>) main contributor to [rqbit](https://github.com/ikatson/rqbit).
