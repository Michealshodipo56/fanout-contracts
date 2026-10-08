#![cfg(test)]

use crate::errors::ContractError;
use crate::types::{AgreementStatus, Beneficiary, ProposalStatus};
use crate::FanoutAgreementContract;
use crate::FanoutAgreementContractClient;

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, Address, Env, String, Symbol, Vec,
};

fn create_token_contract<'a>(
    e: &Env,
    admin: &Address,
) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let contract_id = e
        .register_stellar_asset_contract_v2(admin.clone())
        .address();
    (
        token::Client::new(e, &contract_id),
        token::StellarAssetClient::new(e, &contract_id),
    )
}

fn setup_test<'a>(
    e: &'a Env,
) -> (
    FanoutAgreementContractClient<'a>,
    Address,
    Address,
    Address,
    Address,
    token::Client<'a>,
    token::StellarAssetClient<'a>,
) {
    e.mock_all_auths();
    e.ledger().with_mut(|li| {
        li.timestamp = 1000;
    });

    let contract_id = e.register(FanoutAgreementContract, ());
    let client = FanoutAgreementContractClient::new(e, &contract_id);

    let creator = Address::generate(e);
    let dev1 = Address::generate(e);
    let dev2 = Address::generate(e);
    let dev3 = Address::generate(e);

    let token_admin = Address::generate(e);
    let (token_client, token_admin_client) = create_token_contract(e, &token_admin);

    (
        client,
        creator,
        dev1,
        dev2,
        dev3,
        token_client,
        token_admin_client,
    )
}

#[test]
fn test_initialize_and_get_config() {
    let e = Env::default();
    let (client, creator, dev1, dev2, dev3, token_client, _) = setup_test(&e);

    let mut beneficiaries = Vec::new(&e);
    beneficiaries.push_back(Beneficiary {
        address: dev1.clone(),
        allocation_bps: 5000, // 50%
    });
    beneficiaries.push_back(Beneficiary {
        address: dev2.clone(),
        allocation_bps: 3000, // 30%
    });
    beneficiaries.push_back(Beneficiary {
        address: dev3.clone(),
        allocation_bps: 2000, // 20%
    });

    let name = String::from_str(&e, "Dev Team Revenue Share");
    client.initialize(
        &creator,
        &name,
        &token_client.address,
        &beneficiaries,
        &2, // 2 required approvals
    );

    let config = client.get_config();
    assert_eq!(config.name, name);
    assert_eq!(config.creator, creator);
    assert_eq!(config.accepted_asset, token_client.address);
    assert_eq!(config.version, 1);
    assert_eq!(config.status, AgreementStatus::Active);
    assert_eq!(config.required_approvals, 2);
    assert_eq!(config.beneficiaries.len(), 3);
}

#[test]
fn test_invalid_allocation_total_rejection() {
    let e = Env::default();
    let (client, creator, dev1, dev2, _, token_client, _) = setup_test(&e);

    let mut beneficiaries = Vec::new(&e);
    beneficiaries.push_back(Beneficiary {
        address: dev1,
        allocation_bps: 5000,
    });
    beneficiaries.push_back(Beneficiary {
        address: dev2,
        allocation_bps: 4000, // Total 9000 (invalid)
    });

    let name = String::from_str(&e, "Invalid Share");
    let res = client.try_initialize(&creator, &name, &token_client.address, &beneficiaries, &1);

    assert_eq!(res, Err(Ok(ContractError::InvalidAllocationTotal)));
}

#[test]
fn test_deterministic_payment_distribution_and_rounding() {
    let e = Env::default();
    let (client, creator, dev1, dev2, dev3, token_client, token_admin_client) = setup_test(&e);

    let mut beneficiaries = Vec::new(&e);
    beneficiaries.push_back(Beneficiary {
        address: dev1.clone(),
        allocation_bps: 5000, // 50%
    });
    beneficiaries.push_back(Beneficiary {
        address: dev2.clone(),
        allocation_bps: 3000, // 30%
    });
    beneficiaries.push_back(Beneficiary {
        address: dev3.clone(),
        allocation_bps: 2000, // 20%
    });

    let name = String::from_str(&e, "App Rev Share");
    client.initialize(&creator, &name, &token_client.address, &beneficiaries, &2);

    let payer = Address::generate(&e);
    token_admin_client.mint(&payer, &100_000_000); // 100 tokens

    // Payment of 100 tokens (100_000_000 base units)
    let payment_ref1 = Symbol::new(&e, "PAY001");
    let amounts1 = client.distribute(&payer, &100_000_000, &payment_ref1);

    assert_eq!(amounts1.get(0).unwrap(), 50_000_000);
    assert_eq!(amounts1.get(1).unwrap(), 30_000_000);
    assert_eq!(amounts1.get(2).unwrap(), 20_000_000);

    assert_eq!(token_client.balance(&dev1), 50_000_000);
    assert_eq!(token_client.balance(&dev2), 30_000_000);
    assert_eq!(token_client.balance(&dev3), 20_000_000);

    // Mint 101 base units for second payment
    token_admin_client.mint(&payer, &101);

    // Test non-divisible payment amount of 101 base units
    // 50% of 101 = 50.5 -> base 50, rem 5000
    // 30% of 101 = 30.3 -> base 30, rem 3000
    // 20% of 101 = 20.2 -> base 20, rem 2000
    // Total base = 100, leftover = 1 base unit
    // Dev1 has largest remainder (5000) -> gets +1 -> 51 total!
    let payment_ref2 = Symbol::new(&e, "PAY002");
    let amounts2 = client.distribute(&payer, &101, &payment_ref2);

    assert_eq!(amounts2.get(0).unwrap(), 51);
    assert_eq!(amounts2.get(1).unwrap(), 30);
    assert_eq!(amounts2.get(2).unwrap(), 20);
    assert_eq!(
        amounts2.get(0).unwrap() + amounts2.get(1).unwrap() + amounts2.get(2).unwrap(),
        101
    );
}

#[test]
fn test_replay_protection_duplicate_ref() {
    let e = Env::default();
    let (client, creator, dev1, dev2, _, token_client, token_admin_client) = setup_test(&e);

    let mut beneficiaries = Vec::new(&e);
    beneficiaries.push_back(Beneficiary {
        address: dev1,
        allocation_bps: 6000,
    });
    beneficiaries.push_back(Beneficiary {
        address: dev2,
        allocation_bps: 4000,
    });

    let name = String::from_str(&e, "Replay Test");
    client.initialize(&creator, &name, &token_client.address, &beneficiaries, &1);

    let payer = Address::generate(&e);
    token_admin_client.mint(&payer, &500);

    let pay_ref = Symbol::new(&e, "TX_UNIQUE_1");
    client.distribute(&payer, &100, &pay_ref);

    // Second distribution with same pay_ref must fail
    let res = client.try_distribute(&payer, &100, &pay_ref);
    assert_eq!(res, Err(Ok(ContractError::DuplicatePaymentRef)));
}

#[test]
fn test_governance_proposal_lifecycle() {
    let e = Env::default();
    let (client, creator, dev1, dev2, dev3, token_client, _) = setup_test(&e);

    let mut beneficiaries = Vec::new(&e);
    beneficiaries.push_back(Beneficiary {
        address: dev1.clone(),
        allocation_bps: 5000,
    });
    beneficiaries.push_back(Beneficiary {
        address: dev2.clone(),
        allocation_bps: 5000,
    });

    let name = String::from_str(&e, "Gov Test");
    client.initialize(
        &creator,
        &name,
        &token_client.address,
        &beneficiaries,
        &2, // 2 approvals required
    );

    // Dev1 proposes adding Dev3 with new 40/30/30 split
    let mut new_beneficiaries = Vec::new(&e);
    new_beneficiaries.push_back(Beneficiary {
        address: dev1.clone(),
        allocation_bps: 4000,
    });
    new_beneficiaries.push_back(Beneficiary {
        address: dev2.clone(),
        allocation_bps: 3000,
    });
    new_beneficiaries.push_back(Beneficiary {
        address: dev3.clone(),
        allocation_bps: 3000,
    });

    let prop_id = client.propose_update(&dev1, &new_beneficiaries);
    assert_eq!(prop_id, 1);

    let prop1 = client.get_proposal(&prop_id);
    assert_eq!(prop1.status, ProposalStatus::Pending);
    assert_eq!(prop1.approvals.len(), 1); // Dev1 auto-approved on creation

    // Attempt to execute while pending should fail
    let exec_err = client.try_execute_proposal(&prop_id);
    assert_eq!(exec_err, Err(Ok(ContractError::ProposalNotApproved)));

    // Dev2 approves proposal -> reaches quorum of 2
    client.approve_proposal(&dev2, &prop_id);

    let prop2 = client.get_proposal(&prop_id);
    assert_eq!(prop2.status, ProposalStatus::Approved);
    assert_eq!(prop2.approvals.len(), 2);

    // Execute proposal -> updates beneficiaries and increments version to 2
    client.execute_proposal(&prop_id);

    let config = client.get_config();
    assert_eq!(config.version, 2);
    assert_eq!(config.beneficiaries.len(), 3);
    assert_eq!(config.beneficiaries.get(2).unwrap().address, dev3);
}

#[test]
fn test_status_suspension_and_closure() {
    let e = Env::default();
    let (client, creator, dev1, dev2, _, token_client, token_admin_client) = setup_test(&e);

    let mut beneficiaries = Vec::new(&e);
    beneficiaries.push_back(Beneficiary {
        address: dev1,
        allocation_bps: 5000,
    });
    beneficiaries.push_back(Beneficiary {
        address: dev2,
        allocation_bps: 5000,
    });

    let name = String::from_str(&e, "Status Test");
    client.initialize(&creator, &name, &token_client.address, &beneficiaries, &1);

    // Suspend agreement
    client.set_status(&creator, &AgreementStatus::Suspended);
    let config = client.get_config();
    assert_eq!(config.status, AgreementStatus::Suspended);

    // Distribution while suspended must fail
    let payer = Address::generate(&e);
    token_admin_client.mint(&payer, &1000);
    let pay_ref = Symbol::new(&e, "PAY_SUSP");

    let dist_res = client.try_distribute(&payer, &100, &pay_ref);
    assert_eq!(dist_res, Err(Ok(ContractError::AgreementNotActive)));

    // Closure is terminal and cannot be silently reopened by the creator.
    client.set_status(&creator, &AgreementStatus::Closed);
    let reopen = client.try_set_status(&creator, &AgreementStatus::Active);
    assert_eq!(reopen, Err(Ok(ContractError::InvalidStatusTransition)));
}
