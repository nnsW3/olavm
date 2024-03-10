use core::vm::hardware::ContractAddress;

use crate::{
    config::ExecuteMode,
    ola_storage::OlaCachedStorage,
    tx_exe_manager::{OlaTapeInitInfo, TxExeManager},
};

pub(crate) struct BlockExeInfo {
    pub block_number: u64,
    pub block_timestamp: u64,
    pub sequencer_address: [u64; 4],
    pub chain_id: u64,
}

pub struct BlockExeManager {
    block_info: BlockExeInfo,
    storage: OlaCachedStorage,
}

impl BlockExeManager {
    pub fn new(
        storage_db_path: String,
        chain_id: u64,
        block_number: u64,
        block_timestamp: u64,
        sequencer_address: ContractAddress,
    ) -> anyhow::Result<Self> {
        let block_info = BlockExeInfo {
            block_number,
            block_timestamp,
            sequencer_address,
            chain_id,
        };
        let storage = OlaCachedStorage::new(storage_db_path)?;
        Ok(Self {
            block_info,
            storage,
        })
    }

    pub fn invoke(&mut self, tx: OlaTapeInitInfo) -> anyhow::Result<()> {
        todo!()
    }
}