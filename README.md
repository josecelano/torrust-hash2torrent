# Torrust Hash2Torrent

A web service to get torrents' metadata from the infohashes.

## Usage

```console
cargo run
```

Download the torrent with curl:

```console
curl -o ./ubuntu-23.04-desktop-amd64.iso.torrent <http://127.0.0.1:3000/torrents/443c7602b4fde83d1154d6d9da48808418b181b6>
```

Or with the browser:

<http://127.0.0.1:3000/torrents/443c7602b4fde83d1154d6d9da48808418b181b6>

> NOTICE: The BitTorrent client may not find the torrent and the HTTP could return a 408 (timeout) error after 10 seconds.
