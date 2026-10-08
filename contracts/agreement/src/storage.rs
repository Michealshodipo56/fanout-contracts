use crate::errors::ContractError;
use crate::types::{AgreementConfig, DataKey, GovernanceProposal};
use soroban_sdk::{Env, Symbol};

pub const INSTANCE_BUMP_AMOUNT: u32 = 300_000;
pub const INSTANCE_LIFETIME_THRESHOLD: u32 = 100_000;

pub fn bump_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn has_config(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Config)
}

pub fn set_config(e: &Env, config: &AgreementConfig) {
    e.storage().instance().set(&DataKey::Config, config);
    bump_instance(e);
}

pub fn get_config(e: &Env) -> Result<AgreementConfig, ContractError> {
    if !has_config(e) {
        return Err(ContractError::NotInitialized);
    }
    bump_instance(e);
    Ok(e.storage().instance().get(&DataKey::Config).unwrap())
}

pub fn get_proposal_count(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get(&DataKey::ProposalCount)
        .unwrap_or(0)
}

pub fn increment_proposal_count(e: &Env) -> u64 {
    let next_id = get_proposal_count(e) + 1;
    e.storage()
        .instance()
        .set(&DataKey::ProposalCount, &next_id);
    bump_instance(e);
    next_id
}

pub fn set_proposal(e: &Env, proposal: &GovernanceProposal) {
    e.storage()
        .instance()
        .set(&DataKey::Proposal(proposal.id), proposal);
    bump_instance(e);
}

pub fn get_proposal(e: &Env, proposal_id: u64) -> Result<GovernanceProposal, ContractError> {
    let key = DataKey::Proposal(proposal_id);
    if !e.storage().instance().has(&key) {
        return Err(ContractError::ProposalNotFound);
    }
    bump_instance(e);
    Ok(e.storage().instance().get(&key).unwrap())
}

pub fn has_payment_ref(e: &Env, payment_ref: &Symbol) -> bool {
    e.storage()
        .instance()
        .has(&DataKey::PaymentRef(payment_ref.clone()))
}

pub fn set_payment_ref(e: &Env, payment_ref: &Symbol) {
    e.storage()
        .instance()
        .set(&DataKey::PaymentRef(payment_ref.clone()), &true);
    bump_instance(e);
}
