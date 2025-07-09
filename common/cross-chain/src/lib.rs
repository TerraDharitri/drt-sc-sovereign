#![no_std]

use error_messages::MAX_GAS_LIMIT_PER_TX_EXCEEDED;
use structs::configs::DcdtSafeConfig;
dharitri_sc::imports!();

pub mod deposit_common;
pub mod events;
pub mod execute_common;
pub mod storage;

pub const MAX_TRANSFERS_PER_TX: usize = 10;
pub const DEFAULT_ISSUE_COST: u64 = 50_000_000_000_000_000; // 0.05 REWA
pub const REGISTER_GAS: u64 = 60_000_000;
pub const MAX_GAS_PER_TRANSACTION: u64 = 600_000_000;

#[dharitri_sc::module]
pub trait LibCommon: crate::storage::CrossChainStorage {
    fn require_dcdt_config_valid(&self, config: &DcdtSafeConfig<Self::Api>) {
        require!(
            config.max_tx_gas_limit < MAX_GAS_PER_TRANSACTION,
            MAX_GAS_LIMIT_PER_TX_EXCEEDED
        );
    }
}
