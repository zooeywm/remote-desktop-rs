use std::collections::HashMap;

use rdrs_domain_player::{error::PlayerError, repository::PlayerRepository, Player};
use rdrs_tools::error::Result;

#[derive(dep_inj::DepInj, Default)]
#[target(PlayerMemoryRepository)]
pub struct PlayerMemoryRepositoryState {
	map:   HashMap<u8, Player>,
	count: u8,
}

impl PlayerMemoryRepositoryState {
	pub fn new() -> Self { Self { map: HashMap::new(), count: 0 } }
}

impl<Deps> PlayerRepository for PlayerMemoryRepository<Deps>
where
	Deps: AsRef<PlayerMemoryRepositoryState> + AsMut<PlayerMemoryRepositoryState>,
{
	fn create(&mut self, player: Player) -> Result<u8> {
		let count = &mut self.count;
		*count += 1;
		let count = *count;
		self.map.insert(count, player);
		Ok(count)
	}

	fn get_all(&self) -> Vec<&Player> { self.map.values().collect() }

	fn get_mut_by_id(&mut self, id: u8) -> Result<&mut Player> {
		Ok(self.map.get_mut(&id).ok_or(PlayerError::NoSuchPlayer { id })?)
	}

	fn get_by_id(&self, id: u8) -> Result<&Player> {
		Ok(self.map.get(&id).ok_or(PlayerError::NoSuchPlayer { id })?)
	}
}
