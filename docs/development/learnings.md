# Engineering learnings

## Format Cargo manifests consistently

Align assignment operators within each table. Put one space after the operator.
Use inline tables with interior spaces for inherited properties. Keep short
arrays on one line. For longer arrays, indent each item four spaces and include
a trailing comma. The [API client manifest](../../crates/conctr-api-dioxus/Cargo.toml)
is a working example.

```toml
[package]
name    = "conctr-api-dioxus"
version = { workspace = true }

[dependencies]
dioxus     = { workspace = true, features = ["fullstack"] }
conctr-api = { workspace = true }
```

## Keep local verification within available RAM

Concurrent DX servers and browser instances can exhaust this 14 GiB workstation.
Check for a healthy existing server before starting one. Set
`CARGO_BUILD_JOBS=1` for Cargo and DX, run heavy checks sequentially, and use
one server and one verification browser at a time. Stop processes you started
when switching apps or finishing verification. Preserve user-owned sessions.

If the desktop stalls or swap grows as available memory falls, stop your build
and inspect `free -h` and `/proc/pressure/memory`. After a restart, inspect the
previous boot journal before retrying. Report any renderer check deferred due
to memory pressure.

## Keep experiments in the workspace scratch directory

Use `temp/` for investigation scripts, sample inputs, screenshots and downloaded
references. Keep credentials out of samples. Put durable guidance in `docs/`
and keep required build inputs with their owning package so a clean checkout
can build without scratch files.

## Preserve HTTP semantics at the API boundary

Reuse Conctr's success and problem types when adding management operations.
A client first checks HTTP status, skips parsing for 204, reads Problem Details
on failure, and unwraps `data` for a successful JSON body. Keep request IDs
in headers and test status, media type and body together. See the
[implemented HTTP reference](../architecture/api-and-events.md).

For a future asynchronous operation, 202 can acknowledge admission and
`Location` can identify the run. A later successful GET of that run can return
200 even if execution failed. Put per-record failures or partial success in
the execution resource. Keep third-party pass-through, binary and streaming
payloads in their contracted formats. These conventions guide future endpoints.
