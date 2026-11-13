#!/usr/bin/env python3
"""Check Conctr documentation links, inventory and provisional example artifacts.

Uses the standard library. If jsonschema is installed, also validate the example
against the complete Draft 2020-12 schema. This is not a runtime flow compiler.
"""
import json
from html.parser import HTMLParser
from pathlib import Path
import re
import sys
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


def markdown_links(path):
    fenced = False
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
            check_link(path, target.split(' "', 1)[0])
    if fenced:
        errors.append(f"{path.relative_to(ROOT)}: unclosed code fence")


class HtmlLinks(HTMLParser):
    def __init__(self, source):
        super().__init__()
        self.source = source

    def handle_starttag(self, tag, attrs):
        for name, value in attrs:
            if name in ("href", "src") and value:
                check_link(self.source, value)


def validate_inventory():
    project = json.loads((DOCS / "ai/project.json").read_text())
    members = tomllib.loads((ROOT / "Cargo.toml").read_text())["workspace"]["members"]
    actual = {}
    for member in members:
        for folder in ROOT.glob(member):
            package = tomllib.loads((folder / "Cargo.toml").read_text())["package"]
            actual[package["name"]] = folder.relative_to(ROOT).as_posix()
    documented = {p["name"]: p["path"] for p in project["existing_packages"]}
    if actual != documented:
        errors.append("docs/ai/project.json: existing package inventory differs from Cargo workspace")
    for path in project["references"].values():
        if not (ROOT / path).is_file():
            errors.append(f"docs/ai/project.json: missing reference {path}")
    return len(actual)


def validate_examples():
    schema = json.loads((DOCS / "architecture/schemas/flow.schema.json").read_text())
    flow = json.loads((DOCS / "architecture/examples/http-flow.json").read_text())
    layout = json.loads((DOCS / "architecture/examples/http-flow.layout.json").read_text())
    ids = [node["id"] for node in flow["nodes"]]
    if len(ids) != len(set(ids)) or set(layout["nodes"]) != set(ids):
        errors.append("Example flow node IDs must be unique and agree with layout")
    for edge in flow["edges"]:
        if edge["from"]["node"] not in ids or edge["to"]["node"] not in ids:
            errors.append("Example flow has a dangling edge")
    for node in flow["nodes"]:
        binding = node["config"].get("connection")
        if binding and binding not in flow["bindings"]:
            errors.append(f"Example node {node['id']} references an unknown binding")

    def refs(value):
        if isinstance(value, dict):
            if "$ref" in value:
                reference = value["$ref"]
                if not reference.startswith("#/"):
                    errors.append(f"Provisional schema must be self-contained: {reference}")
                else:
                    try:
                        current = schema
                        for part in reference[2:].split("/"):
                            current = current[part.replace("~1", "/").replace("~0", "~")]
                    except KeyError:
                        errors.append(f"Broken local schema reference: {reference}")
            for child in value.values():
                refs(child)
        elif isinstance(value, list):
            for child in value:
                refs(child)
    refs(schema)
    try:
        from jsonschema import Draft202012Validator
    except ImportError:
        print("JSON Schema library unavailable; checked JSON syntax, local refs and example graph links only.")
    else:
        Draft202012Validator.check_schema(schema)
        Draft202012Validator(schema).validate(flow)
        print("Provisional example validates against Draft 2020-12 schema.")


def main():
    project = json.loads((DOCS / "ai/project.json").read_text())
    markdown = set(DOCS.rglob("*.md"))
    for entry in project["instruction_entrypoints"]:
        path = ROOT / entry
        if not path.is_file():
            errors.append(f"docs/ai/project.json: missing instruction entry {entry}")
        else:
            markdown.add(path)
    for base in (ROOT / "apps", ROOT / "crates", DOCS):
        markdown.update(base.rglob("AGENTS.md"))
    markdown.add(ROOT / "AGENTS.md")
    for path in sorted(markdown):
        markdown_links(path)
    for path in DOCS.rglob("*.html"):
        HtmlLinks(path).feed(path.read_text())
    for path in DOCS.rglob("*.json"):
        json.loads(path.read_text())
    count = validate_inventory()
    validate_examples()
    if errors:
        raise SystemExit("\n".join(errors))
    print(f"Passed: {len(markdown)} Markdown files, {links} local links, {count} workspace packages and documentation examples.")


if __name__ == "__main__":
    main()
