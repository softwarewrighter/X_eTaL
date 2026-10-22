# Nightly UI tests of the live demos

Every night an always-on Linux machine opens the published live demos
in a headless browser, types programs into the editor, and checks what
the panes show, the chrome and the build stamp. It is the test of
record for the sites themselves (the Mac's nightly gate tests the
code; this tests what GitHub Pages serves): a broken publish, a stale
build, a regression in the browser editor or a service worker holding
an old build is found before a reader does.

## What runs

`scripts/ui-check.py` (Python through uv, Playwright with a headless
Chromium) reads `scripts/ui-scenarios.toml` and, for each site:

- compares the footer's build stamp (`Build (host sha time)`) with the
  repository's `main` on GitHub: the X_eTaL editor must be built from
  `main`; a sibling site's stamp is only recorded, since the siblings
  pin an older build on purpose;
- types each scenario into the ASCII pane and checks the Rendered
  pane's glyphs (the arrow for `:=`, the superscript, alpha, omega, the
  low block for `_u`, the underline), the Types pane, and after Run the
  output pane;
- exercises the chrome: Help opens and Escape closes it; Open loads a
  demo; Save as keeps a file in local storage across a reload; Ctrl-.
  zooms; Tab moves the focus; Lint reports a finding and `Fix 1`
  rewrites the source; a pause after typing opens the completion list
  and Tab takes its first row; Terminal shows the editor as one
  screen, typing draws in its cells, and Ctrl-Q brings the text back.

A failure prints `FAIL`, leaves a screenshot under `work/ui/`, and the
exit status is the number of failures. The scenarios are data: add a
`[[scenario]]` table (source, the substrings expected in each pane,
`run = true` for output) or a `[[site]]` table.

```bash
scripts/ui-check.py --check        # the data file is well formed
scripts/ui-check.py --list         # what would run
scripts/ui-check.py --site X_eTaL  # one site, every scenario
just ui-check                      # every site
```

`UI_BROWSER=/usr/bin/chromium` uses an installed Chromium instead of
Playwright's download (on the Mac, Google Chrome's binary works too).

## The nightly

`scripts/ui-nightly.sh` pulls `main`, runs the check with `--quiet`,
writes `work/ui/DATE.log`, and opens a GitHub issue titled "nightly UI
tests failed" (commenting on it while it stays open, closing it when
the tests pass again), as `scripts/nightly.sh` does for the full gate.

## Installing on Arch

Arch has no cron by default; the schedule is a systemd user timer.

```bash
sudo pacman -S --needed git github-cli uv chromium
git clone https://github.com/softwarewrighter/X_eTaL ~/github/softwarewrighter/X_eTaL
cd ~/github/softwarewrighter/X_eTaL
gh auth login                      # once, for the issue
UI_BROWSER=/usr/bin/chromium scripts/ui-check.py --site X_eTaL   # by hand first
mkdir -p ~/.config/systemd/user
cp scripts/ui-nightly.service scripts/ui-nightly.timer ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now ui-nightly.timer
loginctl enable-linger "$USER"     # the timer runs without a login session
systemctl --user list-timers       # ui-nightly.timer, next run at 03:30
```

Without a system Chromium, `uv run --no-project --with playwright
playwright install chromium` downloads Playwright's own and the
`Environment=UI_BROWSER` line in the service is removed. The log of
each run is `work/ui/DATE.log`; `journalctl --user -u ui-nightly`
shows the service's own output. Run it once by hand with
`just ui-nightly` (it opens an issue on failure, so run `just ui-check`
to try the tests without one).
