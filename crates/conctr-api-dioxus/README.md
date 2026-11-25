# Conctr Dioxus API client

This crate adapts the management API to Dioxus. Client calls reuse the response
types from `conctr-api` and convert HTTP problems to `ServerFnError`. With the
`server` feature it re-exports router setup for the fullstack host. See the
[client implementation](src/lib.rs).

## Start the app

From the repository root, run this command. It starts the UI and API together
and opens the login window in your browser.

```sh
CARGO_BUILD_JOBS=1 dx serve --package conctr-web-dioxus
```

1. On a fresh install the username is **admin**. The server prints the location
   of your private temporary password file. Open that file on the server computer
   and copy its contents into the login window.
2. Sign in with that password. Choose and confirm your own password in the login
   window. Finishing setup opens the workspace and retires the temporary password.
3. On future starts, sign in with your saved password. Existing installations
   use their current accounts. Restarting during first-run setup keeps the
   same temporary password until setup succeeds.

## Server settings

The first server start creates `.env` in the workspace root from the
[bundled defaults](../../.env.example). With a standalone server, it uses an
existing `.env` in the working directory or an ancestor, or creates one in the
working directory. Keep that directory private. Existing files keep their
contents. Missing or blank server settings use the built-in defaults, and
process environment variables take precedence over file entries. Restart after edits.

The default account file is named `accounts.json`. The terminal prints the
temporary password file's actual location, including any path overrides.

| Server OS | Default Conctr account directory |
| --- | --- |
| Linux | `$XDG_CONFIG_HOME/conctr`, normally `~/.config/conctr` |
| macOS | `~/Library/Application Support/conctr` |
| Windows | The user's roaming AppData folder under `conctr`, normally `%APPDATA%\conctr` |

Set `CONCTR_ACCOUNTS_FILE` to choose another account file.
Relative paths start at the directory containing `.env`. With a missing account
file, Conctr creates its parent directory and a private temporary password.
Saving your permanent password creates the account file at the requested path.
Existing account files are validated at startup. For a previously provisioned
installation with a damaged account file, restore its backup.

Open the exact URL printed by Dioxus. For remote access, set `CONCTR_PUBLIC_ORIGIN` to the exact external HTTPS origin
(scheme, host and optional port, with an optional trailing slash), terminate TLS at a
trusted reverse proxy, and keep the backend private. Allow only the product's
origin to call the API from browsers. Never log password request bodies, cookies
or account files. Keep account files outside public assets.

For a packaged desktop or mobile launcher, set `CONCTR_NATIVE_API_ORIGIN` in the
workspace-root `.env` or build environment to the HTTPS address of this same
fullstack server. The native build includes only this public setting from the file.
Native release launchers require this setting. Native debug builds also support
the Dioxus local development server.

## Accounts and permissions

The account file is a JSON array. To provision another account, run
`cargo run -p conctr-auth --bin conctr-account -- USERNAME member` in a private
terminal. Append the generated object to the existing array and restart. Give ordinary users the `member` role. Both roles can
open the workspace and view server details on Dashboard.

For password recovery, choose **Forgot password?** and submit the workspace
username. For a provisioned account, the server writes a private JSON handoff
file beside the account file and prints its path in the server terminal. The
administrator verifies the requester's identity through a trusted channel before
privately delivering the temporary password. It expires after 15 minutes or a
server restart. Repeat requests reuse the active password.

Sign in with the same username and temporary password, then choose a permanent
password. Completing the reset consumes the temporary password and revokes that
account's previous sessions. Other accounts keep their sessions.

To disable an account, remove its
record. Replace the file atomically and restart the server after changes. The
server loads the provisioned accounts at startup.

## How a request is checked

1. The login form posts credentials to `/api/v1/auth/login`. The server verifies
   the Argon2id hash on a blocking worker. Each process allows two concurrent
   verifications. Each rolling minute permits ten attempts per username and
   120 attempts across the installation.
2. Conctr issues a signed, 15-minute JWT in a host-only, Secure, HttpOnly,
   SameSite=Strict cookie. The session lookup returns the username and remaining lifetime. Dioxus
   attaches its custom request header, and browsers send their origin on POST.
   The server checks both for browser calls. A native HTTP client uses its cookie jar.
3. Every protected endpoint verifies the signature, algorithm, issuer,
   audience, expiry and active session ID. The local account supplies its role.
   The client calls `/api/v1/auth/session` to restore the UI after a refresh.
4. Log out calls `/api/v1/auth/logout`, revokes the session ID and clears the
   cookie. Restarting the server rotates its randomly generated signing key and
   clears every session. Expiry requires signing in again.

The Dioxus client calls the management router hosted by the product launcher.
Other frontends can use the same cookie and JSON endpoints. See the
[HTTP contract](../../docs/architecture/api-and-events.md).

For the password hash parameters and the reasons for pinning JWT algorithms,
issuer and audience, see the [OWASP password storage guidance](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
and [JWT best current practices](https://www.rfc-editor.org/rfc/rfc8725).

## Deployment scope

Local accounts, session revocation, signing keys and rate limits currently serve
a single server process. Restarting the server requires clients to sign in again.
