use sp_std::prelude::*;
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct LinkChunk {
	/// Total bond amount
	pub bond: u128,
	/// Total unbond amount
    pub unbond: u128,
}

impl Default for LinkChunk {
    fn default() -> Self {
        Self {
            bond: 0,
            unbond: 0,
        }
    }
}