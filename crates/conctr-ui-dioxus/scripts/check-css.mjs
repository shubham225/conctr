// Release/CI check: the bundled utilities must match this crate's sources.
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const directory = mkdtempSync(join(tmpdir(), "conctr-ui-css-"));
try {
  const output = join(directory, "tailwind.css");
  execFileSync(process.execPath, [
    "node_modules/@tailwindcss/cli/dist/index.mjs",
    "-i", "assets/styles/tailwind.input.css", "-o", output, "--minify",
  ], { stdio: "inherit" });
  if (!readFileSync(output).equals(readFileSync("assets/styles/tailwind.css"))) {
    throw new Error("Bundled utilities are stale; run npm run build:css in conctr-ui-dioxus.");
  }
  console.log("Bundled Tailwind CSS matches the library sources.");
} finally {
  rmSync(directory, { recursive: true, force: true });
}
