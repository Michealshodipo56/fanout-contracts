use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InvalidAllocationTotal = 3,
    EmptyBeneficiaries = 4,
    DuplicateBeneficiary = 5,
    BeneficiaryLimitExceeded = 6,
    InvalidPaymentAmount = 7,
    AgreementNotActive = 8,
    Unauthorized = 9,
    ProposalNotFound = 10,
    ProposalAlreadyApproved = 11,
    ProposalExpired = 12,
    ProposalNotApproved = 13,
    ProposalAlreadyExecuted = 14,
    DuplicatePaymentRef = 15,
    InvalidProposalState = 16,
    ArithmeticOverflow = 17,
    MismatchLength = 18,
    InvalidQuorum = 19,
    InvalidStatusTransition = 20,
}
