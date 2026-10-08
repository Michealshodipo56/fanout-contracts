use soroban_sdk::{contracttype, Address, String, Symbol, Vec};

pub const BPS_TOTAL: u32 = 10_000;
pub const MAX_BENEFICIARIES: u32 = 20;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AgreementStatus {
    Active = 1,
    Suspended = 2,
    Closed = 3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalStatus {
    Pending = 1,
    Approved = 2,
    Executed = 3,
    Rejected = 4,
    Expired = 5,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Beneficiary {
    pub address: Address,
    pub allocation_bps: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgreementConfig {
    pub name: String,
    pub creator: Address,
    pub accepted_asset: Address,
    pub beneficiaries: Vec<Beneficiary>,
    pub status: AgreementStatus,
    pub version: u32,
    pub required_approvals: u32,
    pub total_distributed: i128,
    pub transaction_count: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceProposal {
    pub id: u64,
    pub proposer: Address,
    pub new_beneficiaries: Vec<Beneficiary>,
    pub approvals: Vec<Address>,
    pub status: ProposalStatus,
    pub created_at: u64,
    pub config_version: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Config,
    Proposal(u64),
    ProposalCount,
    PaymentRef(Symbol),
}
