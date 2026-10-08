# Data Model

`AgreementConfig` stores the name, creator, accepted asset, beneficiaries, status, version, approval threshold, lifetime distributed amount, and transaction count. A beneficiary combines a Stellar address and allocation in basis points. Proposals retain the proposed list, approvals, status, creation time, and originating configuration version.

Instance TTL is extended whenever durable agreement state is read or written.
