# Working with Rust UI

Conctr vendors a pinned copy of Rust UI for its Dioxus base components.
Use the [component catalog](https://rust-ui.com/docs/components) to find a family,
then check the [local library](../../crates/conctr-ui-dioxus/README.md) for
its available wrapper and usage. Apply Conctr defaults in the wrapper.

The vendor directory, license and hashes belong together. Follow the
[upstream record](../../crates/conctr-ui-dioxus/UPSTREAM.md) when updating the
snapshot. To verify the local copy, run from the repository root:

```sh
(cd crates/conctr-ui-dioxus && python3 scripts/check-upstream.py)
```

Use the [gallery](../../apps/conctr-design-kit/README.md) to inspect a control's
rendered behavior and the [theming guide](theming.md) to check its styles.
