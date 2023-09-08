pub mod eip1271_signature;
pub mod eip712;
pub mod error;
pub mod eth_signature;
pub mod json_rpc_signer;
pub mod packed_eth_signature;
pub mod pk_signer;
pub mod raw_tx;

use async_trait::async_trait;
use error::EthSignerError;
use eth_signature::TxEthSignature;
pub use primitive_types::{H160, H256};
use raw_tx::RawTransaction;
pub type Address = H160;

#[derive(Debug, Clone)]
pub struct EthTypedData {
    pub raw_data: String,
    pub data_hash: H256,
}

#[async_trait]
pub trait EthereumSigner: Send + Sync + Clone {
    async fn sign_message(&self, message: &[u8]) -> Result<TxEthSignature, EthSignerError>;
    async fn sign_transaction(&self, raw_tx: RawTransaction) -> Result<Vec<u8>, EthSignerError>;
    async fn get_address(&self) -> Result<Address, EthSignerError>;
    async fn sign_typed_data(&self, data: &EthTypedData) -> Result<TxEthSignature, EthSignerError>;
}