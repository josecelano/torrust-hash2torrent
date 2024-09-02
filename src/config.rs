use camino::Utf8PathBuf;

pub struct Config {
    pub session_output_dir: Utf8PathBuf,
    pub cache_dir: Utf8PathBuf,
}
