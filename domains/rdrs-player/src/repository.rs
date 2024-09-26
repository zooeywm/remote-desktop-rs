use rdrs_tools::error::Result;

use crate::entity::Player;

pub trait PlayerRepository {
	fn create(&mut self, player: Player) -> Result<u8>;

	fn get_all(&self) -> Vec<&Player>;

	fn get_mut_by_id(&mut self, id: u8) -> Result<&mut Player>;

	fn get_by_id(&self, id: u8) -> Result<&Player>;
}
