use std::path::PathBuf;

#[non_exhaustive]
#[derive(Debug)]
pub enum StreamSource {
	File { path: PathBuf },
}
