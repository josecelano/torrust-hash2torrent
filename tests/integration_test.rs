//! There must be at least one integration test otherwise `cargo next archive ...`
//! command in the Containerfile will not include the main binary.
//!
//! See: <https://github.com/nextest-rs/nextest/issues/423>
use torrust_hash2torrent::run_app_for_integration_tests;

#[test]
fn test_app() {
    assert_eq!(
        "No integration tests yet :-(",
        run_app_for_integration_tests()
    );
}
