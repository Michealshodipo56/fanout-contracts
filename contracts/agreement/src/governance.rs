use crate::errors::ContractError;
use crate::types::{
    AgreementConfig, Beneficiary, BPS_TOTAL, MAX_BENEFICIARIES,
};
use soroban_sdk::{Address, Vec};

pub fn validate_beneficiaries(
    beneficiaries: &Vec<Beneficiary>,
) -> Result<(), ContractError> {
    let len = beneficiaries.len();
    if len == 0 {
        return Err(ContractError::EmptyBeneficiaries);
    }
    if len > MAX_BENEFICIARIES {
        return Err(ContractError::BeneficiaryLimitExceeded);
    }

    let mut sum_bps: u32 = 0;
    for i in 0..len {
        let b1 = beneficiaries.get(i).unwrap();
        if b1.allocation_bps == 0 {
            return Err(ContractError::InvalidAllocationTotal);
        }
        sum_bps = sum_bps
            .checked_add(b1.allocation_bps)
            .ok_or(ContractError::ArithmeticOverflow)?;

        for j in (i + 1)..len {
            let b2 = beneficiaries.get(j).unwrap();
            if b1.address == b2.address {
                return Err(ContractError::DuplicateBeneficiary);
            }
        }
    }

    if sum_bps != BPS_TOTAL {
        return Err(ContractError::InvalidAllocationTotal);
    }

    Ok(())
}

pub fn is_authorized_participant(
    config: &AgreementConfig,
    address: &Address,
) -> bool {
    if config.creator == *address {
        return true;
    }
    for b in config.beneficiaries.iter() {
        if b.address == *address {
            return true;
        }
    }
    false
}
