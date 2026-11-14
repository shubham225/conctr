use std::{path::Path, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=assets/styles/tailwind.input.css");
    println!("cargo:rerun-if-changed=assets/styles/theme.css");
    println!("cargo:rerun-if-changed=node_modules/@tailwindcss/cli/dist/index.mjs");

    // Optional workspace sources are absent in a standalone published package.
    for source in ["../../apps", "../conctr-app-dioxus/src"] {
        if Path::new(source).is_dir() {
            println!("cargo:rerun-if-changed={source}");
        }
    }

    // Library authors install development tools once. Consumers use the shipped
    // CSS, so Cargo builds require neither Node nor a network connection.
    let cli = Path::new("node_modules/@tailwindcss/cli/dist/index.mjs");
    if cli.is_file() {
        let status = Command::new("node")
            .arg(cli)
            .args([
                "-i",
                "assets/styles/tailwind.input.css",
                "-o",
                "assets/styles/tailwind.css",
                "--minify",
            ])
            .status()
            .expect("Run the UI library's installed Tailwind compiler");
        assert!(status.success(), "UI library Tailwind compilation failed");
    }
}
