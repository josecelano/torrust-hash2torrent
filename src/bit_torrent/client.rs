use anyhow::Context;
use camino::Utf8PathBuf;
use std::sync::Arc;
use thiserror::Error;

use bytes::Bytes;
use librqbit::{
    AddTorrent, AddTorrentOptions, AddTorrentResponse, ByteBufOwned, ListOnlyResponse, Session,
    TorrentMetaV1Info,
};

#[derive(Error, Debug)]
pub enum ResolveMagnetError {
    #[error("BitTorrent client session not started")]
    NoSession,
    #[error("Torrent was added to the BitTorrent client for downloading instead of only listing")]
    AddedForDownloading,
    #[error("Torrent could not been added to the BitTorrent client")]
    NotAdded,
}

pub struct Client {
    pub opt_session: Option<Arc<Session>>,
    pub output_dir: Utf8PathBuf,
}

impl Client {
    #[must_use]
    pub fn new(output_dir: Utf8PathBuf) -> Self {
        Self {
            opt_session: None,
            output_dir,
        }
    }

    /// # Errors
    ///
    /// Will return an error if the session can't be created.
    pub async fn start_session(&mut self) -> Result<(), anyhow::Error> {
        self.opt_session = Some(
            Session::new(self.output_dir.clone().into())
                .await
                .context("error creating session")?,
        );

        Ok(())
    }

    /// Return the torrent info and metainfo (torrent binary data) from the magnet link.
    ///
    /// # Errors
    ///
    /// Will return an error if the torrent:
    ///
    /// - Can't be added in list-only mode to the `BitTorrent` client.
    /// - Was added for downloading. It shouldn't, it should be added in list-only mode.
    pub async fn resolve_magnet(
        &self,
        magnet_link: String,
    ) -> Result<(TorrentMetaV1Info<ByteBufOwned>, Bytes), ResolveMagnetError> {
        match &self.opt_session {
            Some(session) => {
                let added = match session
                    .add_torrent(
                        AddTorrent::from_url(&magnet_link),
                        Some(AddTorrentOptions {
                            list_only: true,
                            ..Default::default()
                        }),
                    )
                    .await
                {
                    Ok(add_torrent_response) => add_torrent_response,
                    Err(_err) => return Err(ResolveMagnetError::NotAdded),
                };

                let (info, content) = match added {
                    AddTorrentResponse::AlreadyManaged(_, handle) => (
                        handle.shared().info.clone(),
                        handle.shared().torrent_bytes.clone(),
                    ),
                    AddTorrentResponse::ListOnly(ListOnlyResponse {
                        info,
                        torrent_bytes,
                        ..
                    }) => (info, torrent_bytes),
                    AddTorrentResponse::Added(_, _) => {
                        return Err(ResolveMagnetError::AddedForDownloading)
                    }
                };

                Ok((info, content))
            }
            None => Err(ResolveMagnetError::NoSession),
        }
    }
}
