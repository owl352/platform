use crate::buffer::Buffer;
use dpp::consensus::basic::identity::IdentityAssetLockTransactionOutputNotFoundError;
use dpp::consensus::codes::ErrorWithCode;
use dpp::consensus::ConsensusError;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=IdentityAssetLockTransactionOutputNotFoundError)]
pub struct IdentityAssetLockTransactionOutputNotFoundErrorWasm {
    inner: IdentityAssetLockTransactionOutputNotFoundError,
}

impl From<&IdentityAssetLockTransactionOutputNotFoundError>
    for IdentityAssetLockTransactionOutputNotFoundErrorWasm
{
    fn from(e: &IdentityAssetLockTransactionOutputNotFoundError) -> Self {
        Self { inner: e.clone() }
    }
}

#[wasm_bindgen(js_class=IdentityAssetLockTransactionOutputNotFoundError)]
impl IdentityAssetLockTransactionOutputNotFoundErrorWasm {
    #[wasm_bindgen(js_name=getOutputIndex)]
    pub fn output_index(&self) -> usize {
        self.inner.output_index()
    }

    #[wasm_bindgen(js_name=getCode)]
    pub fn get_code(&self) -> u32 {
        ConsensusError::from(self.inner.clone()).code()
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.inner.to_string()
    }

    #[wasm_bindgen(js_name=serialize)]
    pub fn serialize(&self) -> Result<Buffer, JsError> {
        let bytes = ConsensusError::from(self.inner.clone())
            .serialize()
            .map_err(|e| JsError::from(e))?;

        Ok(Buffer::from_bytes(bytes.as_slice()))
    }
}
