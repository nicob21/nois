use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const NOIS_PROXY: Item<Addr> = Item::new("nois_proxy");
// mapping job_id => random_int
pub const RANDOM_INT_OUTCOME: Map<&str, u8> = Map::new("random_int_outcome");
pub const COUNTER: Item<u64> = Item::new("games_count");
