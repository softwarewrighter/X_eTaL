#!/usr/bin/env python3
"""Are the live demos live? Every github.io link in the READMEs of this
repository and its siblings is fetched and must answer 200 with a real
page, so a broken Pages deploy or a renamed demo is caught before a
reader finds it.

    scripts/check-live.py                 # the six repositories, from GitHub
    scripts/check-live.py DIR...          # READMEs in these checkouts instead
    scripts/check-live.py --quiet         # only the failures and the summary

Each README is read from GitHub's raw site (or from a checkout given on
the command line), its github.io links are collected, de-duplicated and
fetched with a short timeout, following redirects. A link passes when
the final status is 200 and the body looks like a page (an <html> or a
<title>, not GitHub's "There isn't a GitHub Pages site here"). The exit
status is the number of failing links, so CI can run it. Only the
standard library is used: no package to install.
"""

import re
import sys
import urllib.error
import urllib.request
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

OWNER = "softwarewrighter"
REPOS = ["X_eTaL", "X_eTaL-demos", "X_eTaL-ML", "X_eTaL-games", "X_eTaL-libraries", "X_eTaL-extensions"]
RAW = "https://raw.githubusercontent.com/{owner}/{repo}/main/README.md"
LINK = re.compile(r"https?://[A-Za-z0-9.-]+\.github\.io/[^\s)>\"'`\]]*")
TIMEOUT = 20
AGENT = {"User-Agent": "xetal-check-live/1 (+https://github.com/softwarewrighter/X_eTaL)"}
NOT_A_SITE = "There isn't a GitHub Pages site here"


def fetch(url):
    """(status, body or error text) for `url`, redirects followed."""
    try:
        with urllib.request.urlopen(urllib.request.Request(url, headers=AGENT), timeout=TIMEOUT) as r:
            return r.status, r.read(200_000).decode("utf-8", "replace")
    except urllib.error.HTTPError as e:
        return e.code, ""
    except (urllib.error.URLError, TimeoutError, OSError) as e:
        return 0, str(e)


def readmes(args):
    """(name, text) for each README: checkouts given, else GitHub's."""
    if args:
        for d in args:
            path = (Path(d) / "README.md").resolve()
            yield path.parent.name, path.read_text()
        return
    for repo in REPOS:
        status, text = fetch(RAW.format(owner=OWNER, repo=repo))
        if status != 200:
            sys.exit(f"check-live: cannot read {repo}'s README ({status}): {text}")
        yield repo, text


def links_in(text):
    """The github.io links of a README, trailing punctuation dropped."""
    return {m.rstrip(".,;:") for m in LINK.findall(text)}


def check(url):
    """(url, ok, note) for one link."""
    status, body = fetch(url)
    if status != 200:
        return url, False, f"status {status}" if status else f"unreachable: {body[:80]}"
    if NOT_A_SITE in body:
        return url, False, "no GitHub Pages site here (404 page)"
    if "<html" not in body.lower() and "<title" not in body.lower():
        return url, False, "answered, but not an HTML page"
    return url, True, "ok"


def main():
    quiet = "--quiet" in sys.argv[1:]
    args = [a for a in sys.argv[1:] if a != "--quiet"]
    found = {}
    for name, text in readmes(args):
        for url in links_in(text):
            found.setdefault(url, set()).add(name)
    urls = sorted(found)
    with ThreadPoolExecutor(max_workers=8) as pool:
        results = list(pool.map(check, urls))
    failed = 0
    for url, ok, note in results:
        failed += not ok
        if not ok or not quiet:
            mark = "ok  " if ok else "FAIL"
            print(f"{mark} {url}  ({note}; in {', '.join(sorted(found[url]))})")
    print(f"check-live: {len(urls) - failed} of {len(urls)} links serve a page")
    sys.exit(failed)


if __name__ == "__main__":
    main()
