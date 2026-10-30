#!/usr/bin/env -S uv run --no-project --with playwright python
"""Deep UI tests of the live demos (docs/ui-nightly.md): a headless
Chromium (Playwright) opens each published site in
scripts/ui-scenarios.toml, types each scenario into the ASCII pane and
checks the Rendered, Types and output panes, exercises the chrome
(Help, Open, Save, Zoom, Tab, Lint, completion, the
Terminal page), and compares the
footer's build stamp with the repository's main. A failure leaves a
screenshot under work/ui/.

    scripts/ui-check.py                 # every site, every scenario
    scripts/ui-check.py --site X_eTaL   # sites whose name contains this
    scripts/ui-check.py --list          # the sites and scenarios, no browser
    scripts/ui-check.py --check         # the data file is well formed

The browser is Playwright's Chromium, or the one UI_BROWSER names (an
installed /usr/bin/chromium). The exit status is the number of
failures. Python 3.11+ (tomllib); uv supplies playwright.
"""

import argparse
import re
import subprocess
import sys
import time
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DATA = ROOT / "scripts" / "ui-scenarios.toml"
SHOTS = ROOT / "work" / "ui"
SOURCE = 'textarea[aria-label="source, as typed"]'
RENDERED = "section.rendered pre"
OUTPUT = "section.output pre"
TITLE = "section.output h2"
STAMP = re.compile(r"Build \((\S+) ([0-9a-f]{7,40}) (\S+)\)")
PAUSE = 0.6


def load():
    """The sites and scenarios, checked for the fields the runs need."""
    data = tomllib.loads(DATA.read_text())
    for site in data.get("site", []):
        for key in ("name", "url", "repo", "stamp"):
            assert key in site, f"site without {key}: {site}"
        assert site["stamp"] in ("main", "report", "none"), site["name"]
    for s in data.get("scenario", []):
        assert "name" in s and "source" in s, f"scenario without name or source: {s}"
        assert not s.get("output") or s.get("run"), f"{s['name']}: output needs run = true"
    return data


def main_sha(repo):
    """The commit at the head of `repo`'s main, from GitHub."""
    out = subprocess.run(
        ["git", "ls-remote", f"https://github.com/{repo}.git", "refs/heads/main"],
        capture_output=True, text=True, timeout=60, check=False,
    ).stdout
    return out.split()[0] if out.split() else ""


class Run:
    """One site's checks: the page, the failures, the screenshots."""

    def __init__(self, page, site, quiet):
        self.page, self.site, self.quiet, self.failures = page, site, quiet, 0

    def fail(self, what):
        self.failures += 1
        SHOTS.mkdir(parents=True, exist_ok=True)
        slug = re.sub(r"[^A-Za-z0-9]+", "-", f"{self.site['name']}-{what}")[:80]
        shot = SHOTS / f"{time.strftime('%Y-%m-%d')}-{slug}.png"
        try:
            self.page.screenshot(path=str(shot))
        except Exception:  # noqa: BLE001 - a page gone is the failure already reported
            shot = "(no screenshot)"
        print(f"  FAIL {self.site['name']}: {what} [{shot}]")

    def ok(self, what):
        if not self.quiet:
            print(f"  ok   {what}")

    def expect(self, what, text, parts):
        missing = [p for p in parts if p not in text]
        if missing:
            self.fail(f"{what}: missing {missing!r} in {text[:300]!r}")
        else:
            self.ok(what)

    def type_source(self, source):
        self.page.fill(SOURCE, source)
        time.sleep(PAUSE)

    def scenario(self, s):
        self.type_source(s["source"])
        if s.get("rendered"):
            self.expect(f"{s['name']}: rendered", self.page.inner_text(RENDERED), s["rendered"])
        if s.get("types"):
            self.expect(f"{s['name']}: types", self.page.inner_text(OUTPUT), s["types"])
        if s.get("run"):
            self.page.click("button:text-is('Run')")
            self.wait_run()
            self.expect(f"{s['name']}: output", self.page.inner_text(OUTPUT), s.get("output", []))

    def wait_run(self):
        """Until the run has ended: the spinner (`running`) is gone and
        the output pane is titled Output."""
        try:
            self.page.wait_for_selector("main.panes.running", timeout=3000)
        except Exception:  # noqa: BLE001 - a short run has ended already
            pass
        self.page.wait_for_selector("main.panes:not(.running)", timeout=60000)
        self.wait_title("Output")

    def wait_title(self, text, seconds=20):
        for _ in range(int(seconds / 0.2)):
            if self.page.inner_text(TITLE).strip(" \u25b6").startswith(text):
                return True
            time.sleep(0.2)
        self.fail(f"the output pane never read {text!r}: {self.page.inner_text(TITLE)!r}")
        return False

    def stamp(self):
        """The footer's build commit against the repository's main."""
        if self.site["stamp"] == "none":
            return None
        text = self.page.inner_text("footer")
        m = STAMP.search(text)
        if not m:
            return self.fail(f"no build stamp in the footer: {text!r}")
        head = main_sha(self.site["repo"])
        same = head.startswith(m.group(2))
        if self.site["stamp"] == "main" and not same:
            self.fail(f"the site is built from {m.group(2)}, main is {head[:12]}")
        else:
            self.ok(f"build stamp {m.group(2)} ({'main' if same else 'not main, as pinned'})")

    def check_help(self):
        self.page.click("button:text-is('Help')")
        dialog = self.page.locator("[role=dialog][aria-label=Help]")
        if dialog.count() == 0 or "Editing" not in dialog.inner_text():
            return self.fail("Help did not open with its Editing section")
        self.page.keyboard.press("Escape")
        time.sleep(0.3)
        self.expect("help: closed by Escape", str(dialog.count()), ["0"])

    def check_open(self):
        self.page.click("button.open")
        self.page.click(".menu .group-header:has-text('Demos')")
        item = self.page.locator(".menu .item:has-text('life.xtl')").first
        if item.count() == 0:
            return self.fail("Open lists no life.xtl under Demos")
        item.click()
        time.sleep(PAUSE)
        self.expect("open: life.xtl loaded", self.page.input_value(SOURCE), ["Life"])

    def check_save(self):
        self.type_source("saved := 42\nsaved")
        self.page.once("dialog", lambda d: d.accept("ui-test.xtl"))
        self.page.click("button:text-is('Save as')")
        time.sleep(PAUSE)
        self.page.reload()
        self.page.wait_for_selector(SOURCE)
        self.page.click("button.open")
        self.page.click(".menu .group-header:has-text('Your files')")
        item = self.page.locator(".menu .item:has-text('ui-test.xtl')").first
        if item.count() == 0:
            return self.fail("Save as: ui-test.xtl is not among Your files after a reload")
        item.click()
        time.sleep(PAUSE)
        self.expect("save: the file reloads from local storage", self.page.input_value(SOURCE), ["saved := 42"])

    def check_zoom(self):
        self.page.focus(SOURCE)
        self.page.keyboard.press("Control+.")
        time.sleep(0.3)
        zoomed = self.page.locator("main.panes.zoomed").count()
        self.page.keyboard.press("Control+.")
        self.expect("zoom: Ctrl-. zooms the pane", str(zoomed), ["1"])

    def check_focus(self):
        self.page.focus(SOURCE)
        self.page.keyboard.press("Tab")
        time.sleep(0.3)
        current = self.page.locator("section.pane.current").get_attribute("class") or ""
        self.expect("focus: Tab moves to the Rendered pane", current, ["rendered"])

    def check_lint(self):
        self.type_source("u:k_eep := { a b -> a }\n1 u:k_eep 2")
        self.page.wait_for_selector("main.panes:not(.running)")
        self.page.click("button:text-is('Lint')")
        self.wait_title("Lint")
        self.expect("lint: the finding", self.page.inner_text(OUTPUT), ["warning[unused-binding]", "1 warning(s)"])
        self.page.click("button:text-is('Fix 1')")
        time.sleep(PAUSE)
        self.expect("lint: Fix rewrites the parameter", self.page.input_value(SOURCE), ["{ a _u -> a }"])

    def check_complete(self):
        self.type_source("1 + r_a")
        self.page.keyboard.press("End")
        self.page.type(SOURCE, "n")
        time.sleep(1.0)
        rows = self.page.locator("ul.suggest li")
        if rows.count() == 0:
            return self.fail("complete: no list after typing r_an")
        self.expect("complete: r_ange offered", rows.first.inner_text(), ["r_ange"])
        self.page.keyboard.press("Tab")
        time.sleep(0.3)
        self.expect("complete: Tab writes it", self.page.input_value(SOURCE), ["1 + r_ange"])

    def check_terminal(self):
        """The Terminal button opens a shell on the file in xetal edit;
        the editor's screen after the same keys is the one `xetal edit
        --screen` prints (reg/edit-screen-run.out); the shell runs a
        command; exit brings the file back to the three panes."""
        self.type_source("")
        self.page.click("button:text-is('Terminal')")
        self.page.wait_for_selector("pre.tui")
        self.page.focus("pre.tui")
        self.page.keyboard.press("Control+q")
        self.page.keyboard.press("Control+q")
        self.page.keyboard.type("xetal edit screen-sum.xtl")
        self.page.keyboard.press("Enter")
        self.page.keyboard.type("1 + 2")
        self.page.keyboard.press("Control+r")
        time.sleep(0.5)
        self.same_screen("terminal: the editor's screen is xetal edit --screen's", "edit-screen-run")
        self.page.keyboard.press("Control+q")
        self.page.keyboard.press("Control+q")
        self.page.keyboard.type("xetal eval -e '6 * 7'")
        self.page.keyboard.press("Enter")
        time.sleep(0.3)
        self.expect("terminal: the shell runs xetal eval", self.page.inner_text("pre.tui"), ["$ xetal eval -e '6 * 7'", "42"])
        self.page.keyboard.type("exit")
        self.page.keyboard.press("Enter")
        self.page.wait_for_selector(SOURCE)
        self.expect("terminal: exit brings the file back", self.page.input_value(SOURCE), ["1 + 2"])

    def same_screen(self, what, golden):
        """The page's terminal rows against a golden's screen, line by
        line, trailing blanks ignored."""
        want = (ROOT / "reg" / f"{golden}.out").read_text().rstrip("\n").split("\n")
        rows = self.page.locator("pre.tui .row")
        got = [rows.nth(i).inner_text().rstrip() for i in range(rows.count())]
        diff = [i for i, (a, b) in enumerate(zip(got, want)) if a != b.rstrip()]
        if len(got) != len(want) or diff:
            at = diff[0] if diff else min(len(got), len(want))
            self.fail(f"{what}: row {at + 1} differs: {got[at:at + 1]!r} vs {want[at:at + 1]!r}")
        else:
            self.ok(what)

    def all(self, scenarios):
        self.page.goto(self.site["url"], wait_until="networkidle")
        self.stamp()
        if self.site.get("editor", True) is False:
            return
        self.page.wait_for_selector(SOURCE, timeout=30000)
        for s in scenarios:
            self.scenario(s)
        for name in self.site.get("checks", []):
            getattr(self, f"check_{name}")()


def run(args, data):
    from playwright.sync_api import sync_playwright

    import os
    failures = 0
    with sync_playwright() as p:
        browser_path = os.environ.get("UI_BROWSER")
        browser = p.chromium.launch(executable_path=browser_path) if browser_path else p.chromium.launch()
        for site in data["site"]:
            if args.site and args.site.lower() not in site["name"].lower():
                continue
            print(f"{site['name']} ({site['url']})")
            context = browser.new_context(viewport={"width": 1400, "height": 900})
            page = context.new_page()
            page.set_default_timeout(20000)
            r = Run(page, site, args.quiet)
            try:
                r.all(data["scenario"])
            except Exception as e:  # noqa: BLE001 - any error is this site's failure
                r.fail(f"{type(e).__name__}: {str(e)[:300]}")
            failures += r.failures
            context.close()
        browser.close()
    return failures


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--site", help="only sites whose name contains this")
    ap.add_argument("--list", action="store_true", help="print the sites and scenarios; no browser")
    ap.add_argument("--check", action="store_true", help="only check the data file")
    ap.add_argument("--quiet", action="store_true", help="only failures and the summary")
    args = ap.parse_args()
    data = load()
    if args.check:
        print(f"ui-check: {len(data['site'])} site(s), {len(data['scenario'])} scenario(s), data ok")
        return 0
    if args.list:
        for site in data["site"]:
            print(f"site: {site['name']}  {site['url']}  stamp={site['stamp']}  checks={site.get('checks', [])}")
        for s in data["scenario"]:
            print(f"scenario: {s['name']}" + ("  (run)" if s.get("run") else ""))
        return 0
    failures = run(args, data)
    print(f"ui-check: {failures} failure(s)")
    return failures


if __name__ == "__main__":
    sys.exit(main())
