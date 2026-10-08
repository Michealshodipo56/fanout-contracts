# Contract Functions

- `initialize` creates the agreement once and requires creator authorization.
- `distribute` requires payer authorization, a positive amount, an active agreement, and a unique payment reference.
- `propose_update` starts a version-bound beneficiary change.
- `approve_proposal` records one approval per authorized participant.
- `execute_proposal` applies an approved, current-version proposal.
- `set_status` allows the creator to change lifecycle status.
- `get_config` and `get_proposal` expose contract state.
