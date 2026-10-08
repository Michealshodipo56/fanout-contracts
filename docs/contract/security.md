# Security Invariants

- allocations total exactly 10,000 basis points;
- at least one and no more than 20 beneficiaries are configured;
- payment amounts are positive and payout base units sum exactly to the input;
- a payment reference can be accepted only once per agreement;
- token transfers and state changes are atomic;
- participant approvals cannot be duplicated;
- proposals from an old configuration version cannot execute;
- only the creator changes agreement status.
- a closed agreement cannot be reopened.

Mainnet use requires an independent security review. Report vulnerabilities with a private GitHub security advisory.
