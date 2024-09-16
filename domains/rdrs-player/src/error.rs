#[derive(thiserror::Error, Debug)]
pub enum PlayerError {
	#[error("No such player with id: {id}")]
	NoSuchPlayer { id: u8 },
}
