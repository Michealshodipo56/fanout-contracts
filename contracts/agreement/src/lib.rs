#![no_std]

mod errors;
mod events;
mod governance;
mod payment;
mod storage;
mod types;

#[cfg(test)]
mod test;

pub use errors::ContractError;
pub use types::{
    AgreementConfig, AgreementStatus, Beneficiary, DataKey, GovernanceProposal, ProposalStatus,
};

use events::{
    emit_agreement_created, emit_payment_distributed, emit_proposal_approved,
    emit_proposal_created, emit_proposal_executed, emit_status_changed,
};
use governance::{is_authorized_participant, validate_beneficiaries};
use payment::{calculate_allocations, execute_token_transfers};
use storage::{
    get_config, get_proposal, has_config, has_payment_ref,
    increment_proposal_count, set_config, set_payment_ref, set_proposal,
};

use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol, Vec};

#[contract]
pub struct FanoutAgreementContract;

#[contractimpl]
impl FanoutAgreementContract {
    pub fn initialize(
        e: Env,
        creator: Address,
        name: String,
        accepted_asset: Address,
        beneficiaries: Vec<Beneficiary>,
        required_approvals: u32,
    ) -> Result<(), ContractError> {
        if has_config(&e) {
            return Err(ContractError::AlreadyInitialized);
        }

        creator.require_auth();

        validate_beneficiaries(&beneficiaries)?;

        let req_appr = if required_approvals == 0 {
            beneficiaries.len()
        } else {
            required_approvals
        };

        if req_appr > beneficiaries.len() {
            return Err(ContractError::InvalidQuorum);
        }

        let config = AgreementConfig {
            name: name.clone(),
            creator: creator.clone(),
            accepted_asset: accepted_asset.clone(),
            beneficiaries: beneficiaries.clone(),
            status: AgreementStatus::Active,
            version: 1,
            required_approvals: req_appr,
            total_distributed: 0,
            transaction_count: 0,
        };

        set_config(&e, &config);
        emit_agreement_created(&e, &creator, &name, &accepted_asset, &beneficiaries);

        Ok(())
    }

    pub fn distribute(
        e: Env,
        payer: Address,
        amount: i128,
        payment_ref: Symbol,
    ) -> Result<Vec<i128>, ContractError> {
        let mut config = get_config(&e)?;
        if config.status != AgreementStatus::Active {
            return Err(ContractError::AgreementNotActive);
        }

        payer.require_auth();

        if has_payment_ref(&e, &payment_ref) {
            return Err(ContractError::DuplicatePaymentRef);
        }

        let amounts = calculate_allocations(&e, amount, &config.beneficiaries)?;

        execute_token_transfers(&e, &config.accepted_asset, &payer, &config.beneficiaries, &amounts);

        set_payment_ref(&e, &payment_ref);

        config.total_distributed = config
            .total_distributed
            .checked_add(amount)
            .ok_or(ContractError::ArithmeticOverflow)?;
        config.transaction_count += 1;

        set_config(&e, &config);

        emit_payment_distributed(
            &e,
            &payer,
            amount,
            &payment_ref,
            config.beneficiaries.len(),
        );

        Ok(amounts)
    }

    pub fn propose_update(
        e: Env,
        proposer: Address,
        new_beneficiaries: Vec<Beneficiary>,
    ) -> Result<u64, ContractError> {
        let config = get_config(&e)?;
        if config.status != AgreementStatus::Active {
            return Err(ContractError::AgreementNotActive);
        }

        proposer.require_auth();

        if !is_authorized_participant(&config, &proposer) {
            return Err(ContractError::Unauthorized);
        }

        validate_beneficiaries(&new_beneficiaries)?;

        let proposal_id = increment_proposal_count(&e);

        let mut approvals = Vec::new(&e);
        approvals.push_back(proposer.clone());

        let status = if approvals.len() >= config.required_approvals {
            ProposalStatus::Approved
        } else {
            ProposalStatus::Pending
        };

        let proposal = GovernanceProposal {
            id: proposal_id,
            proposer: proposer.clone(),
            new_beneficiaries,
            approvals,
            status,
            created_at: e.ledger().timestamp(),
            config_version: config.version,
        };

        set_proposal(&e, &proposal);
        emit_proposal_created(&e, &proposer, proposal_id, config.version);

        Ok(proposal_id)
    }

    pub fn approve_proposal(
        e: Env,
        approver: Address,
        proposal_id: u64,
    ) -> Result<(), ContractError> {
        let config = get_config(&e)?;
        let mut proposal = get_proposal(&e, proposal_id)?;

        if proposal.status != ProposalStatus::Pending {
            return Err(ContractError::InvalidProposalState);
        }

        if proposal.config_version != config.version {
            return Err(ContractError::ProposalExpired);
        }

        approver.require_auth();

        if !is_authorized_participant(&config, &approver) {
            return Err(ContractError::Unauthorized);
        }

        for app in proposal.approvals.iter() {
            if app == approver {
                return Err(ContractError::ProposalAlreadyApproved);
            }
        }

        proposal.approvals.push_back(approver.clone());

        if proposal.approvals.len() >= config.required_approvals {
            proposal.status = ProposalStatus::Approved;
        }

        set_proposal(&e, &proposal);
        emit_proposal_approved(&e, &approver, proposal_id, proposal.approvals.len());

        Ok(())
    }

    pub fn execute_proposal(e: Env, proposal_id: u64) -> Result<(), ContractError> {
        let mut config = get_config(&e)?;
        let mut proposal = get_proposal(&e, proposal_id)?;

        if proposal.status != ProposalStatus::Approved {
            return Err(ContractError::ProposalNotApproved);
        }

        if proposal.config_version != config.version {
            return Err(ContractError::ProposalExpired);
        }

        config.beneficiaries = proposal.new_beneficiaries.clone();
        config.version += 1;
        proposal.status = ProposalStatus::Executed;

        set_config(&e, &config);
        set_proposal(&e, &proposal);

        emit_proposal_executed(&e, proposal_id, config.version);

        Ok(())
    }

    pub fn set_status(
        e: Env,
        caller: Address,
        new_status: AgreementStatus,
    ) -> Result<(), ContractError> {
        let mut config = get_config(&e)?;
        caller.require_auth();

        if caller != config.creator {
            return Err(ContractError::Unauthorized);
        }

        let old_status = config.status.clone();
        config.status = new_status.clone();

        set_config(&e, &config);
        emit_status_changed(&e, old_status, new_status);

        Ok(())
    }

    pub fn get_config(e: Env) -> Result<AgreementConfig, ContractError> {
        get_config(&e)
    }

    pub fn get_proposal(e: Env, proposal_id: u64) -> Result<GovernanceProposal, ContractError> {
        get_proposal(&e, proposal_id)
    }
}
