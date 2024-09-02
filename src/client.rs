use std::sync::Arc;
use thiserror::Error;

use bytes::Bytes;
use librqbit::{
    AddTorrent, AddTorrentOptions, AddTorrentResponse, ByteBufOwned, ListOnlyResponse, Session,
    TorrentMetaV1Info,
};

#[derive(Error, Debug)]
pub enum ResolveMagnetError {
    #[error("Torrent was added to the BitTorrent client for downloading instead of only listing")]
    AddedForDownloading, // It should not be added for downloading.
    #[error("Torrent could not been added to the BitTorrent client")]
    NotAdded,
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
    session: Arc<Session>,
    magnet_link: String,
) -> Result<(TorrentMetaV1Info<ByteBufOwned>, Bytes), ResolveMagnetError> {
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
        AddTorrentResponse::Added(_, _) => return Err(ResolveMagnetError::AddedForDownloading),
    };

    Ok((info, content))
}
