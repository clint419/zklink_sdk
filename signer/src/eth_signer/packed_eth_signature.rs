use crate::eth_signer::error::EthSignerError;
use crate::eth_signer::{Address, H256};
use parity_crypto::{
    publickey::{public_to_address, recover, Signature as ETHSignature},
    Keccak256,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zklink_sdk_utils::serde::ZeroPrefixHexSerde;

/// Struct used for working with ethereum signatures created using eth_sign (using geth, ethers.js, etc)
/// message is serialized as 65 bytes long `0x` prefixed string.
///
/// Some notes on implementation of methods of this structure:
///
/// Ethereum signed message produced by most clients contains v where v = 27 + recovery_id(0,1,2,3),
/// but for some clients v = recovery_id(0,1,2,3).
/// Library that we use for signature verification (written for bitcoin) expects v = recovery_id
///
/// That is why:
/// 1) when we create this structure by deserialization of message produced by user
/// we subtract 27 from v in `ETHSignature` if necessary and store it in the `ETHSignature` structure this way.
/// 2) When we serialize/create this structure we add 27 to v in `ETHSignature`.
///
/// This way when we have methods that consumes &self we can be sure that ETHSignature::recover_signer works
/// And we can be sure that we are compatible with Ethereum clients.
///
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackedEthSignature(pub ETHSignature);

impl PackedEthSignature {
    pub fn serialize_packed(&self) -> [u8; 65] {
        // adds 27 to v
        self.0.clone().into_electrum()
    }

    pub fn deserialize_packed(bytes: &[u8]) -> Result<Self, EthSignerError> {
        if bytes.len() != 65 {
            return Err(EthSignerError::LengthMismatched);
        }

        let mut bytes_array = [0u8; 65];
        bytes_array.copy_from_slice(bytes);

        if bytes_array[64] >= 27 {
            bytes_array[64] -= 27;
        }

        Ok(PackedEthSignature(ETHSignature::from(bytes_array)))
    }

    pub fn from_hex(s: &str) -> Result<Self, EthSignerError> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let raw = hex::decode(s).map_err(|_e| EthSignerError::InvalidSignatureStr)?;
        Self::deserialize_packed(&raw)
    }

    pub fn as_hex(&self) -> String {
        let raw = self.serialize_packed();
        format!("0x{}", hex::encode(raw))
    }

    pub fn message_to_signed_bytes(msg: &[u8]) -> H256 {
        let prefix = format!("\x19Ethereum Signed Message:\n{}", msg.len());
        let mut bytes = Vec::with_capacity(prefix.len() + msg.len());
        bytes.extend_from_slice(prefix.as_bytes());
        bytes.extend_from_slice(msg);
        bytes.keccak256().into()
    }

    /// Checks signature and returns ethereum address of the signer.
    /// message should be the same message that was passed to `eth.sign`(or similar) method
    /// as argument. No hashing and prefixes required.
    pub fn signature_recover_signer(&self, msg: &[u8]) -> Result<Address, EthSignerError> {
        let signed_bytes = Self::message_to_signed_bytes(msg);
        let public_key = recover(&self.0, &signed_bytes)
            .map_err(|err| EthSignerError::RecoverAddress(err.to_string()))?;

        Ok(public_to_address(&public_key))
    }
}

impl Serialize for PackedEthSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let packed_signature = self.serialize_packed();
        ZeroPrefixHexSerde::serialize(packed_signature, serializer)
    }
}

impl<'de> Deserialize<'de> for PackedEthSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = ZeroPrefixHexSerde::deserialize(deserializer)?;
        Self::deserialize_packed(&bytes).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_eth_signature() {
        let h = PackedEthSignature::default();
        let s = h.as_hex();
        let h2 = PackedEthSignature::from_hex(&s).unwrap();
        println!("{s}");
        println!("{h2:?}");
    }
}