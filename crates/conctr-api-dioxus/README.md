# API

This crate contains all shared fullstack server functions. This is a great place to place any server-only logic you would like to expose in multiple platforms like a method that accesses your database or a method that sends an email.

This crate will be built twice:
1. Once for the server build with the `dioxus/server` feature enabled
2. Once for the client build with the client feature disabled

During the server build, the server functions will be collected and hosted on a public API for the client to call. During the client build, the server functions will be compiled into the client build.

## Dependencies

Most server dependencies (like sqlx and tokio) will not compile on client platforms like WASM. To avoid building server dependencies on the client, you should add platform specific dependencies under the `server` feature in the [Cargo.toml](../Cargo.toml) file. More details about managing server only dependencies can be found in the [Dioxus guide](https://dioxuslabs.com/learn/0.7/guides/fullstack/managing_dependencies#adding-server-only-dependencies).

### Credential reference directory

The fullstack web launcher serves the directory at `/credentials`. Configure
`CONCTR_CREDENTIAL_DIRECTORY` as a private persistent local directory per
workspace and `CONCTR_CREDENTIAL_ADMIN_TOKEN` as a high-entropy server secret.
Place the launcher behind an authenticating TLS reverse proxy. On the four
credential POST endpoints the proxy must authorize workspace credential
administrators, enforce same-origin mutation requests, strip incoming
Authorization headers and inject the configured Bearer token. Keep the backend
private. Missing authorization fails closed. Never ship the token to web clients.

The list, create, update and delete functions under `/api/credentials/` return
metadata only. Updates and deletes require the revision originally read. Local
file locking serializes transactions and atomic replacement commits them.
Deploy one workspace per configured directory. This implementation manages
reference metadata. Secret provisioning and rotation belong to the external vault.

For local development, run the fullstack web launcher and open
`http://localhost:8080/credentials` (use the port printed by dx). With both
credential configuration variables unset, a debug build offers local sign-in.
Read the private temporary file named on the page and paste its code into
the sign-in field. A browser-session HttpOnly cookie authorizes CRUD. Restarting
the server invalidates the code and cookie. Metadata persists under
`$XDG_STATE_HOME/conctr/local-workspace`, defaulting to
`~/.local/state/conctr/local-workspace`. Release builds and configured
deployments require proxy authorization.
