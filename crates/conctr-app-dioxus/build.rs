//! Native packages need their public API address before installation. Read only
//! that setting into the binary. The server loads its own .env at runtime.
#[cfg(feature = "native-client")]
fn main() {
    const KEY: &str = "CONCTR_NATIVE_API_ORIGIN";
    const DOTENV: &str = "../../.env";
    println!("cargo:rerun-if-env-changed={KEY}");
    println!("cargo:rerun-if-changed={DOTENV}");
    if std::env::var_os(KEY).is_some() {
        return;
    }
    let entries = match dotenvy::from_path_iter(DOTENV) {
        Ok(entries) => entries,
        Err(dotenvy::Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => return,
        Err(_) => panic!("Cannot read the workspace .env file."),
    };
    for entry in entries {
        let (key, value) = entry.unwrap_or_else(|_| panic!("Invalid entry in the workspace .env."));
        if key == KEY {
            assert!(
                value.starts_with("https://") && !value.chars().any(char::is_control),
                "CONCTR_NATIVE_API_ORIGIN must be an HTTPS address."
            );
            println!("cargo:rustc-env={KEY}={value}");
            break;
        }
    }
}

#[cfg(not(feature = "native-client"))]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
}
