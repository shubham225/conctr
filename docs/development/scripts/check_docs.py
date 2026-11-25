#!/usr/bin/env python3
"""Check local documentation links, AI entry points and workspace coverage."""
from pathlib import Path
import re
import tomllib
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[3]
DOCS = ROOT / "docs"
errors = []
links = 0


def check_link(source, target):
    global links
    target = target.strip().strip("<>")
    parsed = urlsplit(target)
    if parsed.scheme or parsed.netloc or not parsed.path:
        return
    links += 1
    path = (source.parent / unquote(parsed.path)).resolve()
    if not path.exists():
        errors.append(f"{source.relative_to(ROOT)}: missing local link {target}")
    return path


def markdown_links(path):
    fenced = False
    targets = set()
    for line in path.read_text().splitlines():
        if re.match(r"^\s*(```|~~~)", line):
            fenced = not fenced
            continue
        if fenced:
            continue
        # Claude-style imports are not Markdown links but must also resolve.
        if imported := re.fullmatch(r"@([^\s]+)", line.strip()):
            check_link(path, imported.group(1))
        for target in re.findall(r"\[[^\]]*\]\(([^)]+)\)", line):
            resolved = check_link(path, target.split(' "', 1)[0])
            if resolved is not None:
                targets.add(resolved)
    if fenced:
        errors.append(f"{path.relative_to(ROOT)}: unclosed code fence")
    return targets


def main():
    entries = ["README.md", "AGENTS.md", "CLAUDE.md", "GEMINI.md",
               ".github/copilot-instructions.md"]
    markdown = {ROOT / name for name in entries}
    markdown.update(DOCS.rglob("*.md"))
    for base in (ROOT / "apps", ROOT / "crates"):
        markdown.update(base.glob("*/README.md"))
        markdown.update(base.rglob("AGENTS.md"))
    targets = {path: markdown_links(path) for path in sorted(markdown)}

    manifest = tomllib.loads((ROOT / "Cargo.toml").read_text())
    mapped = targets[DOCS / "architecture/crate-map.md"]
    for pattern in manifest["workspace"]["members"]:
        for member in ROOT.glob(pattern):
            target = member.relative_to(ROOT).as_posix()
            if member not in mapped:
                errors.append(f"Crate map lacks a link to {target}")
    if errors:
        raise SystemExit("\n".join(errors))
    print(f"Passed: {len(markdown)} Markdown files, {links} local links and workspace crate map.")


if __name__ == "__main__":
    main()
