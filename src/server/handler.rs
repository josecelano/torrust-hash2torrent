use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use hyper::{header, HeaderMap, StatusCode};

use serde::Deserialize;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{debug, error, info, trace};

use crate::client::resolve_magnet;
use crate::info_hash::InfoHash;

use crate::AppState;

/// The info hash URL path parameter.
///
/// For example: ` http://127.0.0.1:3000/torrents/443c7602b4fde83d1154d6d9da48808418b181b6`.
///
/// The info hash represents the value collected from the URL path parameter.
/// It does not include validation as this is done by the API endpoint handler,
/// in order to provide a more specific error message.
#[derive(Deserialize)]
pub struct InfoHashParam(pub String);

impl InfoHashParam {
    fn lowercase(&self) -> String {
        self.0.to_lowercase()
    }
}

pub async fn get_metainfo(
    State(app_state): State<Arc<AppState>>,
    Path(info_hash): Path<InfoHashParam>,
) -> Response {
    let Ok(info_hash) = InfoHash::from_str(&info_hash.lowercase()) else {
        return (StatusCode::BAD_REQUEST, "Invalid info hash").into_response();
    };

    info!("req: {}", info_hash.to_hex_string());

    if app_state.cache.contains(&info_hash) {
        if let Ok(bytes) = app_state.cache.get(&info_hash) {
            debug!("cached torrent: {}", app_state.cache.path(&info_hash));

            return torrent_file_response(
                bytes,
                &format!("{}.torrent", info_hash.to_hex_string()),
                &info_hash.to_hex_string(),
            );
        }
    }

    let magnet_link = format!("magnet:?xt=urn:btih:{}", info_hash.to_hex_string());

    let Ok((_info, bytes)) = resolve_magnet(app_state.session.clone(), magnet_link).await else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "BitTorrent client error").into_response();
    };

    match app_state.cache.add(&info_hash, &bytes) {
        Ok(()) => {
            trace!("added torrent to cache: {}", info_hash.to_hex_string());
        }
        Err(err) => {
            error!("error adding torrent to cache: {}", err);
        }
    };

    torrent_file_response(
        bytes,
        &format!("{}.torrent", info_hash.to_hex_string()),
        &info_hash.to_hex_string(),
    )
}

/// Builds the binary response for a torrent file.
///
/// # Panics
///
/// Panics if the filename is not a valid header value for the `content-disposition`
/// header.
#[must_use]
pub fn torrent_file_response(bytes: Bytes, filename: &str, info_hash: &str) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/x-bittorrent"
            .parse()
            .expect("HTTP content type header should be valid"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename={filename}").parse().expect(
            "Torrent filename should be a valid header value for the content disposition header",
        ),
    );
    headers.insert(
        "x-torrust-torrent-infohash",
        info_hash.parse().expect(
            "Torrent infohash should be a valid header value for the content disposition header",
        ),
    );

    (StatusCode::OK, headers, bytes).into_response()
}
