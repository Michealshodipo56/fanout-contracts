use crate::errors::ContractError;
use crate::types::{Beneficiary, BPS_TOTAL};
use soroban_sdk::{token, Address, Env, Vec};

pub fn calculate_allocations(
    e: &Env,
    amount: i128,
    beneficiaries: &Vec<Beneficiary>,
) -> Result<Vec<i128>, ContractError> {
    if amount <= 0 {
        return Err(ContractError::InvalidPaymentAmount);
    }
    let n = beneficiaries.len();
    if n == 0 {
        return Err(ContractError::EmptyBeneficiaries);
    }

    let mut result: Vec<i128> = Vec::new(e);
    let mut base_allocations: Vec<i128> = Vec::new(e);
    let mut remainders: Vec<(u32, u32)> = Vec::new(e);
    let mut sum_base: i128 = 0;
    let bps_total_128 = BPS_TOTAL as i128;

    for i in 0..n {
        let b = beneficiaries.get(i).unwrap();
        let prod = amount
            .checked_mul(b.allocation_bps as i128)
            .ok_or(ContractError::ArithmeticOverflow)?;
        let base = prod / bps_total_128;
        let rem = (prod % bps_total_128) as u32;

        base_allocations.push_back(base);
        remainders.push_back((i, rem));
        sum_base = sum_base
            .checked_add(base)
            .ok_or(ContractError::ArithmeticOverflow)?;
    }

    let left_over = (amount - sum_base) as u32;

    if left_over > 0 {
        let mut allocated_rem: Vec<bool> = Vec::new(e);
        for _ in 0..n {
            allocated_rem.push_back(false);
        }

        for _ in 0..left_over {
            let mut max_rem: u32 = 0;
            let mut best_idx: u32 = 0;
            let mut found = false;

            for j in 0..n {
                let (idx, rem) = remainders.get(j).unwrap();
                if !allocated_rem.get(idx).unwrap() && (!found || rem > max_rem) {
                    max_rem = rem;
                    best_idx = idx;
                    found = true;
                }
            }

            if found {
                let current_val = base_allocations.get(best_idx).unwrap();
                base_allocations.set(best_idx, current_val + 1);
                allocated_rem.set(best_idx, true);
            }
        }
    }

    let mut total_check: i128 = 0;
    for i in 0..n {
        let alloc = base_allocations.get(i).unwrap();
        result.push_back(alloc);
        total_check += alloc;
    }

    if total_check != amount {
        return Err(ContractError::ArithmeticOverflow);
    }

    Ok(result)
}

pub fn execute_token_transfers(
    e: &Env,
    token_address: &Address,
    payer: &Address,
    beneficiaries: &Vec<Beneficiary>,
    amounts: &Vec<i128>,
) {
    let client = token::Client::new(e, token_address);
    for i in 0..beneficiaries.len() {
        let recipient = beneficiaries.get(i).unwrap().address;
        let amt = amounts.get(i).unwrap();
        if amt > 0 {
            client.transfer(payer, &recipient, &amt);
        }
    }
}
