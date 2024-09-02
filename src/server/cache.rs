use std::{
    fs::File,
    io::{self, Read, Write},
};

use bytes::Bytes;
use camino::Utf8PathBuf;

use crate::info_hash::InfoHash;

pub struct Cache {
    pub cache_dir: Utf8PathBuf,
}

impl Cache {
    #[must_use]
    pub fn new(cache_dir: Utf8PathBuf) -> Self {
        Self { cache_dir }
    }

    /// Returns true if the cache contains the torrent.
    #[must_use]
    pub fn contains(&self, info_hash: &InfoHash) -> bool {
        self.path(info_hash).exists()
    }

    /// Adds a torrent to the cache.
    ///
    /// # Errors
    ///
    /// Will return an error if tt can't create or write the cache file.
    pub fn add(&self, info_hash: &InfoHash, data: &Bytes) -> io::Result<()> {
        let mut file = File::create(self.path(info_hash))?;

        file.write_all(data)?;

        Ok(())
    }

    /// Gets the torrent from the cache.
    ///
    /// # Errors
    ///
    /// Will return an error if it can't read the cache file.
    pub fn get(&self, info_hash: &InfoHash) -> io::Result<Bytes> {
        let mut file = File::open(self.path(info_hash))?;

        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        Ok(Bytes::from(buffer))
    }

    /// Returns the cache file path.
    #[must_use]
    pub fn path(&self, info_hash: &InfoHash) -> Utf8PathBuf {
        let mut cached_torrent_path = self.cache_dir.clone();
        cached_torrent_path.push(format!("{}.torrent", info_hash.to_hex_string()));
        cached_torrent_path
    }
}
