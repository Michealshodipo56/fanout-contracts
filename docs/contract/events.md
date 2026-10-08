# Events

The contract publishes agreement creation, payment distribution, proposal creation, proposal approval, proposal execution, and status changes. Consumers must identify events by contract ID and ledger position, persist a checkpoint, and deduplicate by transaction hash plus event index.
