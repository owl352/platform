use crate::version::drive_abci_versions::drive_abci_validation_versions::{
    DriveAbciAssetLockValidationVersions, DriveAbciDocumentsStateTransitionValidationVersions,
    DriveAbciStateTransitionCommonValidationVersions, DriveAbciStateTransitionValidationVersion,
    DriveAbciStateTransitionValidationVersions, DriveAbciValidationConstants,
    DriveAbciValidationDataTriggerAndBindingVersions, DriveAbciValidationDataTriggerVersions,
    DriveAbciValidationVersions, PenaltyAmounts,
};

pub const DRIVE_ABCI_VALIDATION_VERSIONS_V1: DriveAbciValidationVersions =
    DriveAbciValidationVersions {
        state_transitions: DriveAbciStateTransitionValidationVersions {
            common_validation_methods: DriveAbciStateTransitionCommonValidationVersions {
                asset_locks: DriveAbciAssetLockValidationVersions {
                    fetch_asset_lock_transaction_output_sync: 0,
                    verify_asset_lock_is_not_spent_and_has_enough_balance: 0,
                },
                validate_identity_public_key_contract_bounds: 0,
                validate_identity_public_key_ids_dont_exist_in_state: 0,
                validate_identity_public_key_ids_exist_in_state: 0,
                validate_state_transition_identity_signed: 0,
                validate_unique_identity_public_key_hashes_in_state: 0,
                validate_master_key_uniqueness: 0,
                validate_simple_pre_check_balance: 0,
            },
            max_asset_lock_usage_attempts: 16,
            identity_create_state_transition: DriveAbciStateTransitionValidationVersion {
                basic_structure: Some(0),
                advanced_structure: Some(0),
                identity_signatures: Some(0),
                advanced_minimum_balance_pre_check: None,
                nonce: None,
                state: 0,
                transform_into_action: 0,
            },
            identity_update_state_transition: DriveAbciStateTransitionValidationVersion {
                basic_structure: Some(0),
                advanced_structure: Some(0),
                identity_signatures: Some(0),
                advanced_minimum_balance_pre_check: None,
                nonce: Some(0),
                state: 0,
                transform_into_action: 0,
            },
            identity_top_up_state_transition: DriveAbciStateTransitionValidationVersion {
                basic_structure: Some(0),
                advanced_structure: None,
                identity_signatures: None,
                advanced_minimum_balance_pre_check: None,
                nonce: None,
                state: 0,
                transform_into_action: 0,
            },
            identity_credit_withdrawal_state_transition:
                DriveAbciStateTransitionValidationVersion {
                    basic_structure: Some(0),
                    advanced_structure: None,
                    identity_signatures: None,
                    advanced_minimum_balance_pre_check: Some(0),
                    nonce: Some(0),
                    state: 0,
                    transform_into_action: 0,
                },
            identity_credit_withdrawal_state_transition_purpose_matches_requirements: 0,
            identity_credit_transfer_state_transition: DriveAbciStateTransitionValidationVersion {
                basic_structure: Some(0),
                advanced_structure: None,
                identity_signatures: None,
                advanced_minimum_balance_pre_check: Some(0),
                nonce: Some(0),
                state: 0,
                transform_into_action: 0,
            },
            masternode_vote_state_transition: DriveAbciStateTransitionValidationVersion {
                basic_structure: None,
                advanced_structure: Some(0),
                identity_signatures: None,
                advanced_minimum_balance_pre_check: Some(0),
                nonce: Some(0),
                state: 0,
                transform_into_action: 0,
            },
            contract_create_state_transition: DriveAbciStateTransitionValidationVersion {
                basic_structure: Some(0),
                advanced_structure: None,
                identity_signatures: None,
                advanced_minimum_balance_pre_check: None,
                nonce: Some(0),
                state: 0,
                transform_into_action: 0,
            },
            contract_update_state_transition: DriveAbciStateTransitionValidationVersion {
                basic_structure: None,
                advanced_structure: None,
                identity_signatures: None,
                advanced_minimum_balance_pre_check: None,
                nonce: Some(0),
                state: 0,
                transform_into_action: 0,
            },
            documents_batch_state_transition: DriveAbciDocumentsStateTransitionValidationVersions {
                balance_pre_check: 0,
                basic_structure: 0,
                advanced_structure: 0,
                state: 0,
                revision: 0,
                transform_into_action: 0,
                data_triggers: DriveAbciValidationDataTriggerAndBindingVersions {
                    bindings: 0,
                    triggers: DriveAbciValidationDataTriggerVersions {
                        create_contact_request_data_trigger: 0,
                        create_domain_data_trigger: 0,
                        create_identity_data_trigger: 0,
                        create_feature_flag_data_trigger: 0,
                        create_masternode_reward_shares_data_trigger: 0,
                        delete_withdrawal_data_trigger: 0,
                        reject_data_trigger: 0,
                    },
                },
                is_allowed: 0,
                document_create_transition_structure_validation: 0,
                document_delete_transition_structure_validation: 0,
                document_replace_transition_structure_validation: 0,
                document_transfer_transition_structure_validation: 0,
                document_purchase_transition_structure_validation: 0,
                document_update_price_transition_structure_validation: 0,
                document_create_transition_state_validation: 0,
                document_delete_transition_state_validation: 0,
                document_replace_transition_state_validation: 0,
                document_transfer_transition_state_validation: 0,
                document_purchase_transition_state_validation: 0,
                document_update_price_transition_state_validation: 0,
            },
        },
        process_state_transition: 0,
        state_transition_to_execution_event_for_check_tx: 0,
        penalties: PenaltyAmounts {
            identity_id_not_correct: 50000000,
            unique_key_already_present: 10000000,
            validation_of_added_keys_structure_failure: 10000000,
            validation_of_added_keys_proof_of_possession_failure: 50000000,
        },
        event_constants: DriveAbciValidationConstants {
            maximum_vote_polls_to_process: 2,
            maximum_contenders_to_consider: 100,
        },
    };
