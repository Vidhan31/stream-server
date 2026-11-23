# Stream server

Stream server is an open-source torrent streaming server written in Rust. It replaces the Node.js implementation of `server.js` used by Stremio, maintaining API parity while reducing memory usage and CPU overhead.

## Features

### Streaming and transcoding
* Streams torrents using libtorrent-rasterbar with piece deadline prioritization.
* Supports HTTP range requests for seeking in media players.
* Transcodes video on the fly into HLS streams using FFmpeg.
* Probes audio, video, and subtitle tracks with FFprobe.
* Streams media directly from RAR, ZIP, 7Z, and TAR archives without extracting them to disk first.
* Extracts embedded subtitles and calculates OpenSubtitles movie hashes.

### API parity
* Implements the Stremio streaming API, including `/stats.json`, `/network-info`, `/heartbeat`, `/settings`, and `/proxy`.
* Configures BitTorrent parameters at runtime, including DHT, peer exchange, local service discovery, encryption, network interface binding, and proxies. See [BitTorrent settings](docs/bittorrent-settings.md).
* Indexes local files and torrents for the local addon endpoint.

## Project structure

```
stream-server/
├── server/                  # HTTP server, routing, casting, and local addon
├── enginefs/                # Torrent engine abstractions, caching, and transcoding
├── bindings/
│   ├── libtorrent-sys/      # FFI bindings to libtorrent-rasterbar
│   ├── async-rar/           # Async UnRAR bindings
│   └── async-sevenz/        # Async 7-Zip bindings
├── settings-gui/            # Slint desktop interface for configuration
├── stremio-runtime-stub/    # Compatibility binary for desktop integration
└── updater-helper/          # In-place update helper binary
```
