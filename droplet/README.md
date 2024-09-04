# Droplet configuration

It's a sample production configuration to deploy the webapp on a Digital Ocean droplet.

## Requirements

- Docker version 24.0.7, build afdd53b.
- Docker Compose version v2.3.3.
- GNU bash, version 5.2.15(1)-release (x86_64-pc-linux-gnu).

## Install

```console
cd \
  && mkdir -p github/torrust \
  && cd torrust/ \
  && git clone --single-branch --branch main https://github.com/torrust/torrust-hash2torrent.git \
  && cd torrust-hash2torrent/ \
  && git status \
  && cd droplet/ \
  && ./bin/install.sh
```

### HTTPS

Get certificates:

Log into the `certbot` container:

```console
docker com1pose run --entrypoint /bin/sh certbot
```

Get staging certificates:

```console
certbot certonly --webroot --webroot-path=/var/www/html --email your@email.com --agree-tos --no-eff-email --staging -d hash2torrent.com
```

Get production certificates:

```console
certbot certonly --webroot --webroot-path=/var/www/html --email your@email.com --agree-tos --no-eff-email --force-renewal -d hash2torrent.com
```

Check that the proxy can see the certificates:

```console
docker compose exec proxy ls -la /etc/letsencrypt/live
```

Generate your key with the openssl command:

```console
sudo openssl dhparam -out /home/torrust/github/torrust/torrust-hash2torrent/droplet/storage/dhparam/dhparam-2048.pem 2048
```

Edit the Nginx config file:

```console
vim ./storage/proxy/etc/nginx-conf/nginx.conf
```

Uncomment the lines for HTTPS servers and recreate the proxy with:

```console
docker compose up -d --force-recreate --no-deps proxy
```

Add the following cronjob with `sudo crontab -e` to auto-renew certificates:

```text
0 12 * * * /home/torrust/github/torrust/torrust-hash2torrent/droplet/bin/ssl_renew.sh >> /var/log/cron.log 2>&1
```

You can check the cronjob output with `tail -n 200  /var/log/cron.log`.

### Storage

This is how the storage folder is configured after installation (including HTTPS).

```console
$ sudo tree storage
storage
├── certbot
│   ├── etc
│   │   ├── accounts
│   │   │   ├── acme-staging-v02.api.letsencrypt.org
│   │   │   │   └── directory
│   │   │   │       └── 2cedba87be07faf5da056edf65b8939f
│   │   │   │           ├── meta.json
│   │   │   │           ├── private_key.json
│   │   │   │           └── regr.json
│   │   │   └── acme-v02.api.letsencrypt.org
│   │   │       └── directory
│   │   │           └── 08ee268c3eef6bf23301b1300bda96e5
│   │   │               ├── meta.json
│   │   │               ├── private_key.json
│   │   │               └── regr.json
│   │   ├── archive
│   │   │   └── hash2torrent.com
│   │   │       ├── cert1.pem
│   │   │       ├── cert2.pem
│   │   │       ├── chain1.pem
│   │   │       ├── chain2.pem
│   │   │       ├── fullchain1.pem
│   │   │       ├── fullchain2.pem
│   │   │       ├── privkey1.pem
│   │   │       └── privkey2.pem
│   │   ├── live
│   │   │   ├── README
│   │   │   └── hash2torrent.com
│   │   │       ├── README
│   │   │       ├── cert.pem -> ../../archive/hash2torrent.com/cert2.pem
│   │   │       ├── chain.pem -> ../../archive/hash2torrent.com/chain2.pem
│   │   │       ├── fullchain.pem -> ../../archive/hash2torrent.com/fullchain2.pem
│   │   │       └── privkey.pem -> ../../archive/hash2torrent.com/privkey2.pem
│   │   ├── renewal
│   │   │   └── hash2torrent.com.conf
│   │   └── renewal-hooks
│   │       ├── deploy
│   │       ├── post
│   │       └── pre
│   └── lib
├── dhparam
│   └── dhparam-2048.pem
├── hash2torrent
│   ├── etc
│   ├── lib
│   │   ├── session
│   │   └── torrents
│   │       └── 443c7602b4fde83d1154d6d9da48808418b181b6.torrent
│   └── log
└── proxy
    ├── etc
    │   └── nginx-conf
    │       └── nginx.conf
    └── webroot

30 directories, 24 files
```

## Usage

To start the application:

```s
docker compose up --build --detach
```

To stop the application:

```s
docker compose down
```

By default, the application will:

- Be available at <http://localhost:3000>.
- Use the `./storage` directory to store the data.

After starting the application you should see these running containers:

```s
$ docker ps
CONTAINER ID   IMAGE                       COMMAND                  CREATED       STATUS                 PORTS                                                                      NAMES
03e2f0a66512   nginx:mainline-alpine       "/docker-entrypoint.…"   3 hours ago   Up 8 minutes           0.0.0.0:80->80/tcp, :::80->80/tcp, 0.0.0.0:443->443/tcp, :::443->443/tcp   proxy
5b30ef8ddcd1   torrust/hash2torrent:main   "/usr/local/bin/entr…"   3 hours ago   Up 3 hours (healthy)   0.0.0.0:3000->3000/tcp, :::3000->3000/tcp, 51000-51010/tcp                 hash2torrent
```

Other commands are:

Restart all (reloading env vars from `.env` file by forcing recreation):

```console
docker compose up -d --force-recreate
```

Restart proxy (to reload Nginx configuration):

```console
docker compose --ansi never restart proxy
```

Update container images (to upgrade the services):

```console
docker compose down
docker compose pull
docker compose up --build --detach
```
