# cardano Secret Handling Policy

Secret-bearing data must be isolated before local signing or wallet-adjacent
features are admitted.

Secret-bearing data includes:

- private keys, seed phrases, and derivation material;
- signing requests that expose raw transaction bodies before user approval;
- raw signed transactions before submission policy has been applied;
- bearer tokens, node credentials, and endpoint secrets.

Rules:

- Local key storage is not a default feature.
- External-signer-first APIs are preferred.
- Error messages and logs must not include secret-bearing bytes.
- Query and submit adapters must redact endpoint credentials and raw signed
  transactions by default.
- Any future sanitization support stays outside the default dependency graph.

Public values such as addresses, transaction ids, block hashes, and policy ids
are not secret-bearing by themselves, but they may still be sensitive in
application logs.
