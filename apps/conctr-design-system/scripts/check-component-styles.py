#!/usr/bin/env python3
"""Verify rendered component classes match the CSS served by a running gallery.

Usage: python3 check-component-styles.py http://127.0.0.1:8080
Uses only the Python standard library. Run against a fullstack gallery so that
component markup is present in each response before client hydration.
"""
from html.parser import HTMLParser
from pathlib import Path
import re
import sys
from urllib.parse import urljoin
from urllib.request import urlopen


class Page(HTMLParser):
    def __init__(self, html):
        super().__init__()
        self.stylesheets = []
        self.classes = set()
        self.stack = []
        self.feed(html)

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag == "link" and attrs.get("rel") == "stylesheet":
            self.stylesheets.append(attrs["href"])
        classes = attrs.get("class", "").split()
        inside = "cn-body" in classes or any(scoped for _, scoped in self.stack)
        if inside:
            self.classes.update(classes)
        if tag not in {"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"}:
            self.stack.append((tag, inside))

    def handle_endtag(self, tag):
        for index in range(len(self.stack) - 1, -1, -1):
            if self.stack[index][0] == tag:
                del self.stack[index:]
                break



def check(base_url):
    catalog = Path(__file__).resolve().parents[1] / "src/catalog.rs"
    slugs = re.findall(r'slug: "([\w-]+)"', catalog.read_text())
    cache = {}
    failures = []
    checked = set()
    utilities = set()
    library = catalog.parents[3] / "crates/conctr-ui-dioxus"
    css_sources = [library / "assets/styles/main.css", *library.glob("src/components/**/style.css")]
    markers = set()
    for source in css_sources:
        markers.update(re.findall(r"\.((?:cn|dx)-[\w-]+)", source.read_text()))
    for slug in slugs:
        url = urljoin(base_url.rstrip("/") + "/", f"components/{slug}")
        with urlopen(url, timeout=30) as response:
            page = Page(response.read().decode())
        if not page.classes:
            failures.append(f"{slug}: no server-rendered component classes found")
            continue
        sheets = []
        for href in page.stylesheets:
            stylesheet_url = urljoin(url, href)
            if stylesheet_url not in cache:
                with urlopen(stylesheet_url, timeout=30) as response:
                    cache[stylesheet_url] = response.read().decode()
            sheets.append(cache[stylesheet_url])
        css = "\n".join(sheets)
        if not any(re.search(r"/tailwind(?:-dxh[0-9a-f]+)?\.css$", href) for href in page.stylesheets):
            failures.append(f"{slug}: missing library Tailwind stylesheet")
        if "tailwindcss v4.1." not in css:
            failures.append(f"{slug}: missing compiled Tailwind 4.1 output")
        for directive in ("@apply ", "@reference ", "@theme "):
            if directive in css:
                failures.append(f"{slug}: uncompiled {directive.strip()} in served CSS")
        for cls in page.classes:
            if cls.startswith(("cn-", "dx-", "ds-")):
                if re.search(r"-[0-9a-f]{8}$", cls):
                    checked.add(cls)  # Manganis-scoped component selector.
                elif cls in markers:
                    # Unscoped hooks accompany hashed module classes. Only
                    # intentionally global selectors must match literally.
                    if not re.search(r"\." + re.escape(cls) + r"(?![\w-])", css):
                        continue
                    checked.add(cls)
                else:
                    continue
            else:
                utilities.add(cls)
            escaped = "".join(char if char.isalnum() or char in "_-" else "\\" + char for char in cls)
            if not re.search(r"\." + re.escape(escaped) + r"(?![\w-])", css):
                failures.append(f"{slug}: no served selector for .{cls}")
    if failures:
        raise SystemExit("\n".join(failures))
    print(f"Passed: {len(slugs)} pages, {len(checked)} component classes, "
          f"{len(utilities)} utility classes, {len(cache)} stylesheets fetched and matched.")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit(__doc__)
    check(sys.argv[1])
