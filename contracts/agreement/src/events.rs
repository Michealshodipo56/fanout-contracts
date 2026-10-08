use crate::types::{AgreementStatus, Beneficiary};
use soroban_sdk::{symbol_short, Address, Env, String, Symbol, Vec};

pub fn emit_agreement_created(
    e: &Env,
    creator: &Address,
    name: &String,
    accepted_asset: &Address,
    beneficiaries: &Vec<Beneficiary>,
) {
    e.events().publish(
        (symbol_short!("created"), creator.clone()),
        (name.clone(), accepted_asset.clone(), beneficiaries.len()),
    );
}

pub fn emit_payment_distributed(
    e: &Env,
    payer: &Address,
    amount: i128,
    payment_ref: &Symbol,
    recipients_count: u32,
) {
    e.events().publish(
        (symbol_short!("pay_dist"), payer.clone()),
        (amount, payment_ref.clone(), recipients_count),
    );
}

pub fn emit_proposal_created(e: &Env, proposer: &Address, proposal_id: u64, config_version: u32) {
    e.events().publish(
        (symbol_short!("prop_new"), proposer.clone()),
        (proposal_id, config_version),
    );
}

pub fn emit_proposal_approved(e: &Env, approver: &Address, proposal_id: u64, total_approvals: u32) {
    e.events().publish(
        (symbol_short!("prop_appr"), approver.clone()),
        (proposal_id, total_approvals),
    );
}

pub fn emit_proposal_executed(e: &Env, proposal_id: u64, new_version: u32) {
    e.events().publish(
        (symbol_short!("prop_exec"), proposal_id),
        new_version,
    );
}

pub fn emit_status_changed(e: &Env, old_status: AgreementStatus, new_status: AgreementStatus) {
    e.events().publish(
        (symbol_short!("status_ch"), old_status),
        new_status,
    );
}
