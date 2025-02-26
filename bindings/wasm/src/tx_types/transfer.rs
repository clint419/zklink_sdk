use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use zklink_sdk_types::basic_types::{BigUint, ZkLinkAddress};
use zklink_sdk_types::error::TypeError::InvalidBigIntStr;
use zklink_sdk_types::tx_builder::TransferBuilder as TxTransferBuilder;
use zklink_sdk_types::tx_type::transfer::Transfer as TransferTx;

#[wasm_bindgen]
pub struct Transfer {
    inner: TransferTx,
}

#[wasm_bindgen]
impl Transfer {
    pub fn json_value(&self) -> Result<JsValue, JsValue> {
        Ok(serde_wasm_bindgen::to_value(&self.inner)?)
    }
}

#[wasm_bindgen]
pub struct TransferBuilder {
    inner: TxTransferBuilder,
}

#[wasm_bindgen]
impl TransferBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(
        account_id: u32,
        to_address: String,
        from_sub_account_id: u8,
        to_sub_account_id: u8,
        token: u32,
        fee: String,
        amount: String,
        nonce: u32,
        ts: Option<u32>,
    ) -> Result<TransferBuilder, JsValue> {
        let ts = if let Some(time_stamp) = ts {
            time_stamp
        } else {
            std::time::UNIX_EPOCH.elapsed().unwrap().as_millis() as u32
        };
        let inner = TxTransferBuilder {
            account_id: account_id.into(),
            to_address: ZkLinkAddress::from_hex(&to_address)?,
            from_sub_account_id: from_sub_account_id.into(),
            to_sub_account_id: to_sub_account_id.into(),
            token: token.into(),
            fee: BigUint::from_str(&fee).map_err(|e| InvalidBigIntStr(e.to_string()))?,
            nonce: nonce.into(),
            timestamp: ts.into(),
            amount: BigUint::from_str(&amount).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        };
        Ok(TransferBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn build(self) -> Transfer {
        Transfer {
            inner: self.inner.build(),
        }
    }
}

#[wasm_bindgen(js_name=newTransfer)]
pub fn new_transfer(builder: TransferBuilder) -> Transfer {
    builder.build()
}
