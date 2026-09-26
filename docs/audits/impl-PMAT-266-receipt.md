# PMAT-266 — implementation receipt (paiml/depyler#266)

## Scope (issue #266 plus the operator ruling relayed by the cop, 2026-09-26)

Restore `.github/workflows/nightly.yml` on the self-hosted fleet. Each night is
published as a NEW dated prerelease rather than overwriting one `nightly` tag.
**The PR folds the dependabot action bumps #251, #252 and #253.** The runs-on
labels are the operator-approved move of depyler's nightly onto intel ("you can
move depyler to nightly", operator, relayed verbatim by the cop).

In scope, each one asked for:

1. **Runners.** Hosted runners are banned fleet-wide (paiml/infra#231).
   - check-activity, release and the x86_64 linux build run on intel clean-room.
   - aarch64 linux builds in `rust:1.93.0-bullseye` on the gx10 docker runner.
   - Both darwin targets build on the mini.
   - Windows is recorded as NotRun, since there is no self-hosted Windows runner.
2. **Action bumps folded from dependabot:**
   - #251: download-artifact → v8.0.1.
   - #252: upload-artifact → v7.0.1.
   - #253: softprops/action-gh-release → v3.0.3.
   These are the three PRs this one supersedes.
3. **Dead steps removed.** The paiml/provable-contracts checkout, the symlink and
   the `pv` codegen are gone. depyler has no provable-contracts Cargo dependency,
   and its `generated_contracts.rs` files are committed, so the steps built
   nothing. They only failed the job on a hosted-runner checkout.
4. **Tags.** Each tag is `nightly-<YYYYMMDD>-<sha7>`, with `prerelease: true` and
   `make_latest: false`. No tag is ever deleted.
5. **Skipped nights.** A night with no commits writes `SKIP` to the run summary.

## Not in scope

- The ci.yml gate job.
- Semver releases.
- crates.io.

## Proof owed before arm

A `workflow_dispatch` run of this branch on the fleet, publishing one dated
prerelease with sha256-verified assets.
