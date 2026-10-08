# Contract Architecture

Each deployment represents one agreement. Instance storage contains its configuration, proposal counter, proposals, and processed payment references. Contract functions validate state and authorization before calling the accepted token contract. Events expose lifecycle changes to off-chain indexers.
