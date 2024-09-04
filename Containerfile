# syntax=docker/dockerfile:latest

# Torrust Hash2Torrent

## Builder Image
FROM docker.io/library/rust:bookworm AS chef
WORKDIR /tmp
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm cargo-chef cargo-nextest

## Tester Image
FROM docker.io/library/rust:slim-bookworm AS tester
WORKDIR /tmp

RUN apt-get update; apt-get install -y curl sqlite3 tree; apt-get autoclean
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall --no-confirm cargo-nextest

COPY ./share/ /app/share/torrust

## Su Exe Compile
FROM docker.io/library/gcc:bookworm AS gcc
COPY ./contrib/dev-tools/su-exec/ /usr/local/src/su-exec/
RUN cc -Wall -Werror -g /usr/local/src/su-exec/su-exec.c -o /usr/local/bin/su-exec; chmod +x /usr/local/bin/su-exec


## Chef Prepare (look at project and see wat we need)
FROM chef AS recipe
WORKDIR /build/src
COPY . /build/src
RUN cargo chef prepare --recipe-path /build/recipe.json


## Cook (debug)
FROM chef AS dependencies_debug
WORKDIR /build/src
COPY --from=recipe /build/recipe.json /build/recipe.json
RUN cargo chef cook --tests --benches --examples --workspace --all-targets --all-features --recipe-path /build/recipe.json
RUN cargo nextest archive --tests --benches --examples --workspace --all-targets --all-features --archive-file /build/temp.tar.zst ; rm -f /build/temp.tar.zst

## Cook (release)
FROM chef AS dependencies
WORKDIR /build/src
COPY --from=recipe /build/recipe.json /build/recipe.json
RUN cargo chef cook --tests --benches --examples --workspace --all-targets --all-features --recipe-path /build/recipe.json --release
RUN cargo nextest archive --tests --benches --examples --workspace --all-targets --all-features --archive-file /build/temp.tar.zst --release  ; rm -f /build/temp.tar.zst


## Build Archive (debug)
FROM dependencies_debug AS build_debug
WORKDIR /build/src
COPY . /build/src
RUN cargo nextest archive --tests --benches --examples --workspace --all-targets --all-features --archive-file /build/torrust-hash2torrent-debug.tar.zst

## Build Archive (release)
FROM dependencies AS build
WORKDIR /build/src
COPY . /build/src
RUN cargo nextest archive --tests --benches --examples --workspace --all-targets --all-features --archive-file /build/torrust-hash2torrent.tar.zst --release


# Extract and Test (debug)
FROM tester AS test_debug
WORKDIR /test
COPY . /test/src/
COPY --from=build_debug \
  /build/torrust-hash2torrent-debug.tar.zst \
  /test/torrust-hash2torrent-debug.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --extract-to /test/src/ --no-run --archive-file /test/torrust-hash2torrent-debug.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --target-dir-remap /test/src/target/ --cargo-metadata /test/src/target/nextest/cargo-metadata.json --binaries-metadata /test/src/target/nextest/binaries-metadata.json

RUN mkdir -p /app/bin/; cp -l /test/src/target/debug/torrust-hash2torrent /app/bin/torrust-hash2torrent; cp -l /test/src/target/debug/http_health_check /app/bin/http_health_check
RUN chown -R root:root /app; chmod -R u=rw,go=r,a+X /app; chmod -R a+x /app/bin

# Extract and Test (release)
FROM tester AS test
WORKDIR /test
COPY . /test/src
COPY --from=build \
  /build/torrust-hash2torrent.tar.zst \
  /test/torrust-hash2torrent.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --extract-to /test/src/ --no-run --archive-file /test/torrust-hash2torrent.tar.zst
RUN cargo nextest run --workspace-remap /test/src/ --target-dir-remap /test/src/target/ --cargo-metadata /test/src/target/nextest/cargo-metadata.json --binaries-metadata /test/src/target/nextest/binaries-metadata.json

RUN mkdir -p /app/bin/; cp -l /test/src/target/release/torrust-hash2torrent /app/bin/torrust-hash2torrent; cp -l /test/src/target/release/http_health_check /app/bin/http_health_check
RUN chown -R root:root /app; chmod -R u=rw,go=r,a+X /app; chmod -R a+x /app/bin


## Runtime
FROM gcr.io/distroless/cc-debian12:debug AS runtime
RUN ["/busybox/cp", "-sp", "/busybox/sh","/busybox/cat","/busybox/ls","/busybox/env", "/bin/"]
COPY --from=gcc --chmod=0555 /usr/local/bin/su-exec /bin/su-exec

ARG USER_ID=1000
ARG API_PORT=3000

ENV USER_ID=${USER_ID}
ENV API_PORT=${API_PORT}
ENV TZ=Etc/UTC

EXPOSE ${API_PORT}/tcp
EXPOSE 51000-51010/tcp

RUN mkdir -p /var/lib/torrust/hash2torrent /var/log/torrust/hash2torrent /etc/torrust/hash2torrent

ENV ENV=/etc/profile
COPY --chmod=0555 ./share/container/entry_script_sh /usr/local/bin/entry.sh

VOLUME ["/var/lib/torrust/hash2torrent","/var/log/torrust/hash2torrent","/etc/torrust/hash2torrent"]

ENV RUNTIME="runtime"
ENTRYPOINT ["/usr/local/bin/entry.sh"]


## Debug
FROM runtime AS debug
ENV RUNTIME="debug"
COPY --from=test_debug /app/ /usr/
RUN env
CMD ["sh"]

## Release (default)
FROM runtime AS release
ENV RUNTIME="release"
COPY --from=test /app/ /usr/
HEALTHCHECK --interval=5s --timeout=5s --start-period=3s --retries=3 \  
  CMD /usr/bin/http_health_check http://localhost:${API_PORT}/health_check \
  || exit 1
CMD ["/usr/bin/torrust-hash2torrent"]
