use std::path::PathBuf;

#[non_exhaustive]
#[derive(Debug)]
pub enum StreamType {
	File { path: PathBuf },
}
