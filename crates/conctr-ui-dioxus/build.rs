use std::{path::Path, process::Command};
fn main() {
    for path in [
        "src",
        "assets/styles/theme.css",
        "assets/styles/tailwind.input.css",
        "assets/styles/utilities.css",
        "assets/styles/shell-utilities.html",
    ] {
        if Path::new(path).exists() {
            println!("cargo:rerun-if-changed={path}");
        }
    }
    let cli = "node_modules/@tailwindcss/cli/dist/index.mjs";
    if Path::new(cli).exists() {
        let result = Command::new("node")
            .args([
                cli,
                "-i",
                "assets/styles/tailwind.input.css",
                "-o",
                "assets/styles/tailwind.css",
                "--minify",
            ])
            .status();
        assert!(
            result.is_ok_and(|status| status.success()),
            "Tailwind compilation failed"
        );
    }
}
