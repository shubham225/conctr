# Conctr mobile launcher

Mobile is an existing workspace target scheduled for future removal. Current
product development targets web and desktop. The [mobile entry point](src/main.rs)
configures the native API client and launches the same
[product app](../../crates/conctr-app-dioxus/README.md).

For maintenance on an Android development machine with its SDK configured, run
from the repository root.

```sh
CARGO_BUILD_JOBS=1 dx serve --package conctr-mobile-dioxus --platform android
```

Packaged clients use the HTTPS API origin described in the
[native client settings](../../crates/conctr-api-dioxus/README.md).
