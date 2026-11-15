# Rust UI provenance

Repository: https://github.com/rust-ui/ui
Revision: `4f5a516e76d75a15a587ab2169352cf34fd318d0`
Source: `app_crates/registry/src/{ui,hooks,demos,workflows}`
License: MIT, copyright 2026 Max Wells; full notice at [src/rust-ui/LICENSE](src/rust-ui/LICENSE).

All Rust files in these four directories are byte-for-byte copies. The integration
`src/rust-ui/mod.rs` and hash manifest are Conctr-owned. `ui` and `hooks` absolute
imports resolve through crate-root facades without editing upstream imports.
Demos and workflows are exposed for the showcase; applications use `ui` and `composed`.

Run `python3 scripts/check-upstream.py` after every change. For an update, download
an immutable upstream commit, inspect its license and dependency changes, replace
the four source directories, regenerate the manifest, and review wrapper/API
compatibility. Rebuild CSS and run Rust/browser checks. Do not run a CLI force-update
over the facade or reformat vendored files. Upstream files are exempt from local
format/lint corrections; isolate allowances at the module boundary where necessary.

Brand fonts include their supplied SIL OFL notices in `assets/fonts/`. Keep these
and the Rust UI MIT notice in source and distributed artifacts. Dependency licenses
remain separate; the workspace's MIT/Apache declaration does not relicense Rust UI.

The Conctr-owned `src/hooks/use_click_outside.rs` overrides the root hook seam.
It owns and removes its document listener on component disposal instead of
forgetting the JS closure. The unchanged upstream data-grid hook resolves
`crate::hooks::use_click_outside` through this adapter. Styling compatibility
corrections for border defaults, command selection and table/grid roles live
in `assets/styles/main.css`, outside the source snapshot.
