# ADR 0013: Immutable Rust UI source and Conctr facade

Status: Accepted by explicit user direction, 2026-11-15.
Supersedes component-source/layer specifics in ADR 0002 and CSS-module specifics
in ADR 0012. Platform ownership and library-owned compiled CSS remain applicable.

## Context

The maintainer requested replacement from scratch after backup, preserving the
canonical brand kit and keeping Rust UI upgradeable without local source edits.

## Decision

Vendor commit `4f5a516e76d75a15a587ab2169352cf34fd318d0` under `src/rust-ui/`,
retain MIT notice and SHA-256 inventory, and keep files unchanged. Expose `ui/`
facades and put custom combinations in `composed/`. Map semantic Tailwind roles to
Conctr tokens and assets. Keep the shipped CSS and development tooling in the UI
crate. Recreate the gallery; migrate the shared app to the facade.

## Alternatives

Continuing editable DioxusLabs components would not meet the requested replacement.
Customizing Rust UI source directly would obstruct clean upstream updates.
A second runtime UI dependency or a Leptos frontend is unnecessary.

## Consequences and verification

The old public API is removed; archived source provides recovery. Upstream is
experimental and includes incomplete Dioxus ports (AutoForm). Workflow examples
are presentation demonstrations only. Verify unchanged upstream hashes, facade
compilation, brand colors/fonts, browser interaction and consumer style delivery
on every update. No production or native-readiness claim follows from vendoring.
