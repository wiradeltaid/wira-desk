## Summary of changes

<!-- Provide a concise explanation of what this pull request changes and why. -->

## Checklist before requesting review

- [ ] Formatted with `cargo fmt --all`
- [ ] Clippy checks pass cleanly: `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] Test suite passes cleanly: `$env:WIRADESK_SKIP_MANIFEST = '1'; cargo test --workspace`
- [ ] Any new public facts/numbers match `docs/public-facts.yaml` and pass `.\scripts\verify-public-facts.ps1`
- [ ] Publication hygiene gate passes: `.\scripts\verify-public-export.ps1 -Path . -SkipHistory`
- [ ] Any new `unsafe` block includes an explicit, truthful `SAFETY:` documentation comment
