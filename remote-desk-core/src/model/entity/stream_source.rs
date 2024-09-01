use std::path::PathBuf;

#[non_exhaustive]
pub enum StreamSource {
	File { path: PathBuf },
}
