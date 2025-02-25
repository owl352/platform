use std::convert::TryFrom;

use dpp::data_contract::created_data_contract::CreatedDataContract;

use dpp::data_contract::{DataContractFacade, DataContract, TokenConfiguration, TokenContractPosition};
use dpp::identifier::Identifier;

use crate::data_contract::state_transition::DataContractCreateTransitionWasm;
use crate::data_contract::state_transition::DataContractUpdateTransitionWasm;
use crate::data_contract::DataContractWasm;
use crate::errors::protocol_error::from_protocol_error;
use crate::utils::{
    get_bool_from_options, ToSerdeJSONExt, WithJsError, SKIP_VALIDATION_PROPERTY_NAME,
};

use dpp::ProtocolError;
use std::sync::Arc;

use dpp::prelude::IdentityNonce;
use wasm_bindgen::prelude::*;
use dpp::data_contract::accessors::v0::DataContractV0Getters;
use dpp::data_contract::accessors::v1::DataContractV1Setters;
use dpp::data_contract::associated_token::token_configuration::v0::TokenConfigurationV0;
use dpp::data_contract::associated_token::token_configuration_item::TokenConfigurationChangeItem;
use dpp::data_contract::change_control_rules::authorized_action_takers::AuthorizedActionTakers;
use dpp::data_contract::conversion::json::DataContractJsonConversionMethodsV0;
use dpp::platform_value::string_encoding::Encoding;

impl From<DataContractFacade> for DataContractFacadeWasm {
    fn from(facade: DataContractFacade) -> Self {
        Self(Arc::new(facade))
    }
}

#[wasm_bindgen(js_name=DataContractFacade)]
#[derive(Clone)]
pub struct DataContractFacadeWasm(pub(crate) Arc<DataContractFacade>);

impl DataContractFacadeWasm {
    pub fn new(protocol_version: u32) -> Self {
        let inner = DataContractFacade::new(protocol_version).expect("should create facade");

        Self(Arc::new(inner))
    }
}

#[wasm_bindgen(js_class=DataContractFacade)]
impl DataContractFacadeWasm {
    /// Create Data Contract
    #[wasm_bindgen]
    pub fn create(
        &self,
        owner_id: Vec<u8>,
        identity_nonce: IdentityNonce,
        documents: JsValue,
        definitions: Option<js_sys::Object>,
    ) -> Result<DataContractWasm, JsValue> {
        let id = Identifier::from_bytes(&owner_id)
            .map_err(ProtocolError::ValueError)
            .with_js_error()?;

        //todo: contract config
        self.0
            .create(
                id,
                identity_nonce,
                serde_wasm_bindgen::from_value(documents)?,
                None,
                definitions
                    .map(|definitions| serde_wasm_bindgen::from_value(definitions.into()))
                    .transpose()?,
            )
            .map(Into::into)
            .map_err(from_protocol_error)
    }

    /// Create Data Contract from plain object
    #[wasm_bindgen(js_name=createFromObject1)]
    pub async fn create_from_object(
        &self,
        js_raw_data_contract: JsValue,
        options: Option<js_sys::Object>,
    ) -> Result<DataContractWasm, JsValue> {

        use web_sys::console;

        console::log_1(&"starts".into());

        let skip_validation = if let Some(options) = options {
            get_bool_from_options(options.into(), SKIP_VALIDATION_PROPERTY_NAME, false)?
        } else {
            false
        };

        console::log_1(&"skip_validation here".into());

        let mut data_contract = match self.0
          .create_from_object(
              js_raw_data_contract.with_serde_to_platform_value()?,
              skip_validation,
          ){
            Ok(data_contract) => data_contract,
            Err(err) => {
                console::log_1(&err.to_string().into());

                panic!("err");
            }
        };



        console::log_1(&"data_contract here".into());

        let mut token_configuration = TokenConfigurationV0::default_most_restrictive();

        token_configuration.apply_token_configuration_item(
            TokenConfigurationChangeItem::ManualMinting(
                AuthorizedActionTakers::ContractOwner
            )
        );

        let token_configuration = TokenConfiguration::from(token_configuration);

        console::log_1(&"token created".into());

        data_contract.add_token(0u16, token_configuration);

        console::log_1(&"token here".into());

        Ok(data_contract.into())
    }

    /// Create Data Contract from buffer
    #[wasm_bindgen(js_name=createFromBuffer)]
    pub async fn create_from_buffer(
        &self,
        buffer: Vec<u8>,
        options: Option<js_sys::Object>,
    ) -> Result<DataContractWasm, JsValue> {
        let skip_validation = if let Some(options) = options {
            get_bool_from_options(options.into(), SKIP_VALIDATION_PROPERTY_NAME, false)?
        } else {
            false
        };
        self.0
            .create_from_buffer(buffer, skip_validation)
            .map(Into::into)
            .map_err(from_protocol_error)
    }

    /// Create Data Contract Create State Transition
    #[wasm_bindgen(js_name=createDataContractCreateTransition)]
    pub fn create_data_contract_create_transition(
        &self,
        data_contract: &DataContractWasm,
    ) -> Result<DataContractCreateTransitionWasm, JsValue> {
        self.0
            .create_data_contract_create_transition(
                CreatedDataContract::try_from(data_contract).with_js_error()?,
            )
            .map(DataContractCreateTransitionWasm::from)
            .with_js_error()
    }

    /// Create Data Contract Update State Transition
    #[wasm_bindgen(js_name=createDataContractUpdateTransition)]
    pub fn create_data_contract_update_transition(
        &self,
        data_contract: &DataContractWasm,
        identity_contract_nonce: IdentityNonce,
    ) -> Result<DataContractUpdateTransitionWasm, JsValue> {
        self.0
            .create_data_contract_update_transition(
                data_contract.to_owned().into(),
                identity_contract_nonce,
            )
            .map(Into::into)
            .map_err(from_protocol_error)
    }
}
