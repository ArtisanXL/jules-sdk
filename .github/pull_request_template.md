## Summary
<!-- Describe the changes introduced by this PR. What problem does it solve? -->

## Related Issues
<!-- Link to issues fixed or addressed by this PR. Format: "Fixes #123" -->

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Code refactoring or performance optimization

## Checklist
Before submitting this Pull Request, I confirm that:
- [ ] I have read and followed [CONTRIBUTING.md](https://github.com/<organization>/jules-sdk/blob/main/CONTRIBUTING.md) and [AGENTS.md](https://github.com/<organization>/jules-sdk/blob/main/AGENTS.md).
- [ ] My branch follows the branch naming conventions in `CONTRIBUTING.md` (e.g., `feat/` or `fix/`).
- [ ] All code conforms to the [ARCHITECTURE.md](https://github.com/<organization>/jules-sdk/blob/main/ARCHITECTURE.md) design principles.
- [ ] All new and existing tests pass (`cargo nextest run` or `cargo test`).
- [ ] Formatting is clean and verified (`cargo fmt --all --check`).
- [ ] Clippy checks pass without warnings (`cargo clippy --workspace --all-features -- -D warnings`).
- [ ] Dependency audits pass (`cargo audit` and `cargo deny check` if applicable).
- [ ] Documentation is updated (Rustdocs, README, or other `.md` files as required).
