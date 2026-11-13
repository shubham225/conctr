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
        self.feed(html)

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag == "link" and attrs.get("rel") == "stylesheet":
            self.stylesheets.append(attrs["href"])
        self.classes.update(
            cls for cls in attrs.get("class", "").split()
            if re.fullmatch(r"dx-[\w-]+-[a-f0-9]{8}", cls)
        )


def check(base_url):
    catalog = Path(__file__).resolve().parents[1] / "src/catalog.rs"
    slugs = re.findall(r'slug: "([\w-]+)"', catalog.read_text())
    cache = {}
    failures = []
    checked = set()
    for slug in slugs:
        url = urljoin(base_url.rstrip("/") + "/", f"components/{slug}")
        with urlopen(url, timeout=30) as response:
            page = Page(response.read().decode())
        if not page.classes:
            failures.append(f"{slug}: no server-rendered CSS module classes found")
            continue
        sheets = []
        for href in page.stylesheets:
            stylesheet_url = urljoin(url, href)
            if stylesheet_url not in cache:
                with urlopen(stylesheet_url, timeout=30) as response:
                    cache[stylesheet_url] = response.read().decode()
            sheets.append(cache[stylesheet_url])
        css = "\n".join(sheets)
        for cls in page.classes:
            checked.add(cls)
            if not re.search(r"\." + re.escape(cls) + r"(?![\w-])", css):
                failures.append(f"{slug}: no served selector for .{cls}")
    if failures:
        raise SystemExit("\n".join(failures))
    print(f"Passed: {len(slugs)} pages, {len(checked)} component classes, "
          f"{len(cache)} stylesheets fetched and matched.")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit(__doc__)
    check(sys.argv[1])
