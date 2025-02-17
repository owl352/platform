use super::broadcast_request::BroadcastRequestForStateTransition;
use crate::platform::block_info_from_metadata::block_info_from_metadata;
use crate::{Error, Sdk};
use dapi_grpc::platform::VersionedGrpcResponse;
use dpp::state_transition::proof_result::StateTransitionProofResult;
use dpp::state_transition::StateTransition;
use drive::drive::Drive;
use drive_proof_verifier::error::ContextProviderError;
use drive_proof_verifier::DataContractProvider;
use rs_dapi_client::{DapiRequest, IntoInner, RequestSettings};

#[async_trait::async_trait]
pub trait BroadcastStateTransition {
    async fn broadcast(&self, sdk: &Sdk) -> Result<(), Error>;
    async fn broadcast_and_wait(
        &self,
        sdk: &Sdk,
        time_out_ms: Option<u64>,
    ) -> Result<StateTransitionProofResult, Error>;
}

#[async_trait::async_trait]
impl BroadcastStateTransition for StateTransition {
    async fn broadcast(&self, sdk: &Sdk) -> Result<(), Error> {
        let request = self.broadcast_request_for_state_transition()?;

        request
            .execute(sdk, RequestSettings::default())
            .await // TODO: We need better way to handle execution errors
            .into_inner()?;

        // response is empty for a broadcast, result comes from the stream wait for state transition result

        Ok(())
    }

    async fn broadcast_and_wait(
        &self,
        sdk: &Sdk,
        _time_out_ms: Option<u64>,
    ) -> Result<StateTransitionProofResult, Error> {
        let request = self.broadcast_request_for_state_transition()?;
        // TODO: Implement retry logic
        request
            .clone()
            .execute(sdk, RequestSettings::default())
            .await
            .into_inner()?;

        let request = self.wait_for_state_transition_result_request()?;

        let response = request
            .execute(sdk, RequestSettings::default())
            .await
            .into_inner()?;

        let block_info = block_info_from_metadata(response.metadata()?)?;
        let proof = response.proof_owned()?;
        let context_provider =
            sdk.context_provider()
                .ok_or(Error::from(ContextProviderError::Config(
                    "Context provider not initialized".to_string(),
                )))?;

        let (_, result) = Drive::verify_state_transition_was_executed_with_proof(
            self,
            &block_info,
            proof.grovedb_proof.as_slice(),
            &context_provider.as_contract_lookup_fn(),
            sdk.version(),
        )?;

        Ok(result)
    }
}
