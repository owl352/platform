//! Definitions of errors
use dpp::consensus::ConsensusError;
use dpp::serialization::PlatformDeserializable;
use dpp::version::PlatformVersionError;
use dpp::ProtocolError;
pub use drive_proof_verifier::error::ContextProviderError;
use rs_dapi_client::transport::TransportError;
use rs_dapi_client::{CanRetry, DapiClientError, ExecutionError};
use std::fmt::Debug;
use std::time::Duration;

/// Error type for the SDK
// TODO: Propagate server address and retry information so that the user can retrieve it
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// SDK is not configured properly
    #[error("SDK misconfigured: {0}")]
    Config(String),
    /// Drive error
    #[error("Drive error: {0}")]
    Drive(#[from] drive::error::Error),
    /// DPP error
    #[error("Protocol error: {0}")]
    Protocol(#[from] ProtocolError),
    /// Proof verification error
    #[error("Proof verification error: {0}")]
    Proof(#[from] drive_proof_verifier::Error),
    /// Invalid Proved Response error
    #[error("Invalid Proved Response error: {0}")]
    InvalidProvedResponse(String),
    /// DAPI client error, for example, connection error
    #[error("Dapi client error: {0}")]
    DapiClientError(String),
    #[cfg(feature = "mocks")]
    /// DAPI mocks error
    #[error("Dapi mocks error: {0}")]
    DapiMocksError(#[from] rs_dapi_client::mock::MockError),
    /// Dash core error
    #[error("Dash core error: {0}")]
    CoreError(#[from] dpp::dashcore::Error),
    /// MerkleBlockError
    #[error("Dash core error: {0}")]
    MerkleBlockError(#[from] dpp::dashcore::merkle_tree::MerkleBlockError),
    /// Core client error, for example, connection error
    #[error("Core client error: {0}")]
    CoreClientError(#[from] dashcore_rpc::Error),
    /// Dependency not found, for example data contract for a document not found
    #[error("Required {0} not found: {1}")]
    MissingDependency(String, String),
    /// Total credits in Platform are not found; we must always have credits in Platform
    #[error("Total credits in Platform are not found; it should never happen")]
    TotalCreditsNotFound,
    /// Epoch not found; we must have at least one epoch
    #[error("No epoch found on Platform; it should never happen")]
    EpochNotFound,
    /// SDK operation timeout reached error
    #[error("SDK operation timeout {} secs reached: {1}", .0.as_secs())]
    TimeoutReached(Duration, String),
    /// Generic error
    // TODO: Use domain specific errors instead of generic ones
    #[error("SDK error: {0}")]
    Generic(String),

    /// Context provider error
    #[error("Context provider error: {0}")]
    ContextProviderError(#[from] ContextProviderError),

    /// Operation cancelled - cancel token was triggered, timeout, etc.
    #[error("Operation cancelled: {0}")]
    Cancelled(String),

    /// Remote node is stale; try another server
    #[error(transparent)]
    StaleNode(#[from] StaleNodeError),
}

// TODO: Decompose DapiClientError to more specific errors like connection, node error instead of DAPI client error
impl From<DapiClientError> for Error {
    fn from(value: DapiClientError) -> Self {
        if let DapiClientError::Transport(TransportError::Grpc(status)) = &value {
            if let Some(consensus_error_value) = status
                .metadata()
                .get_bin("dash-serialized-consensus-error-bin")
            {
                return ConsensusError::deserialize_from_bytes(
                    consensus_error_value.as_encoded_bytes(),
                )
                .map(|consensus_error| {
                    Self::Protocol(ProtocolError::ConsensusError(Box::new(consensus_error)))
                })
                .unwrap_or_else(Self::Protocol);
            }
        }

        Self::DapiClientError(format!("{:?}", value))
    }
}

impl From<PlatformVersionError> for Error {
    fn from(value: PlatformVersionError) -> Self {
        Self::Protocol(value.into())
    }
}

impl<T> From<ExecutionError<T>> for Error
where
    ExecutionError<T>: ToString,
{
    fn from(value: ExecutionError<T>) -> Self {
        // TODO: Improve error handling
        Self::DapiClientError(value.to_string())
    }
}

impl CanRetry for Error {
    fn can_retry(&self) -> bool {
        matches!(self, Error::StaleNode(..) | Error::TimeoutReached(_, _))
    }
}

/// Server returned stale metadata
#[derive(Debug, thiserror::Error)]
pub enum StaleNodeError {
    /// Server returned metadata with outdated height
    #[error("received height is outdated: expected {expected_height}, received {received_height}, tolerance {tolerance_blocks}; try another server")]
    Height {
        /// Expected height - last block height seen by the Sdk
        expected_height: u64,
        /// Block height received from the server
        received_height: u64,
        /// Tolerance - how many blocks can be behind the expected height
        tolerance_blocks: u64,
    },
    /// Server returned metadata with time outside of the tolerance
    #[error(
        "received invalid time: expected {expected_timestamp_ms}ms, received {received_timestamp_ms} ms, tolerance {tolerance_ms} ms; try another server"
    )]
    Time {
        /// Expected time in milliseconds - is local time when the message was received
        expected_timestamp_ms: u64,
        /// Time received from the server in the message, in milliseconds
        received_timestamp_ms: u64,
        /// Tolerance in milliseconds
        tolerance_ms: u64,
    },
}
