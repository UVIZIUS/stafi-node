use parity_codec::{Encode, Decode};
use rstd::prelude::*;
#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
use crate::chain::StakeTokenType;

#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum XtzStakeStage {
	// Init - Transfer token to multi sig address
	Init,
	// Successful transfer
	Completed,
}

#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq)]
pub struct XtzStakeData<AccountId, Hash, Balance> {
	// stake identifier id
	pub id: Hash,
	// creator of stake
	pub initiator: AccountId,
	// Stage of stake
	pub stage: XtzStakeStage,
	// multi sig address
	pub multi_sig_address: Vec<u8>,
	// Token data of stake
	pub stake_amount: Balance,
	// transaction hash
	pub tx_hash: Vec<u8>,
	// block hash
	pub block_hash: Vec<u8>,
	// xtz account
	pub stake_account: Vec<u8>,
	// xtz sig
	pub sig: Vec<u8>,
}

#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq)]
pub enum AtomStakeStage {
	// Init
	Init,
	// Successful transfer
	TransferSuccess,
	// Active staking stage
	Staking,
	// Completed staking stage
	Completed,
}

#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq)]
pub struct AtomStakeData<AccountId, Hash, Balance> {
	// stake identifier id
	pub id: Hash,
	// creator of stake
	pub initiator: AccountId,
	// Stage of stake
	pub stage: AtomStakeStage,
	// multi sig address
	pub multi_sig_address: Vec<u8>,
	// Token data of stake
	pub stake_amount: Balance,
	// transaction hash
	pub tx_hash: Vec<u8>,
	// block hash
	pub block_hash: Vec<u8>,
	// atom account
	pub stake_account: Vec<u8>,
	// atom sig
	pub sig: Vec<u8>,
}


#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Clone, PartialEq)]
pub struct StakeDropAct<BlockNumber, Balance> {
	pub begin: BlockNumber,
	pub end: BlockNumber,
	pub current_cycle: u32,
	pub token_type: StakeTokenType,
	pub issue_amount: Balance,
}