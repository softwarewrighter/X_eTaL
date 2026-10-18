# Cloud sessions: setting up a fresh container

A cloud session (claude.ai/code) runs in a container that starts from a
fresh clone of this repository and is reclaimed when the session ends
or sits idle. Nothing installed in one session survives into the next.
The full process (the agentrail saga, `just gate`, reg-rs goldens,
sw-checklist, `just pages`) needs tools that are not in the base image.
`scripts/cloud-setup.sh` installs them all in one idempotent run.

## How to use it

Either of these:

1. As the cloud environment's setup script, so every new session starts
   ready. In the session's title bar open the environment menu, choose
   Edit, and set Setup script to:

   ```
   bash /home/user/X_eTaL/scripts/cloud-setup.sh
   ```

2. By hand, first thing in a session:

   ```
   bash scripts/cloud-setup.sh
   ```

It prints a summary of each tool's path and ends with
`cloud setup complete`, or lists what failed and exits 1. Measured in
a session on 2026-10-18: a first run builds four tools from source and
takes several minutes; a second run, with everything present, takes
about 4 seconds (tools on PATH are skipped, cloned repositories are
fast-forwarded).

## What it installs, and why

| What | From | Used by |
| ---- | ---- | ------- |
| `agentrail` | sw-vibe-coding/agentrail-rs, `crates/agentrail-cli` | the saga protocol (CLAUDE.md: next, begin, complete) |
| `reg-rs` | sw-cli-tools/reg-rs, `components/app/crates/bin` | the CLI goldens (`scripts/reg.sh`, the gate) |
| `sw-checklist` | sw-vibe-coding/sw-checklist (a fork, newer than softwarewrighter's), `components/checklist-cli/crates/sw-checklist` | the structure gate (functions, modules, crates) |
| `sw-markdown-checker` | sw-vibe-coding/sw-markdown-checker (a fork, newer than softwarewrighter/markdown-checker) | the ASCII-only markdown gate |
| `sw-cli` | sw-cli-tools/sw-cli, cloned only | a path dependency of sw-markdown-checker (`../../sw-cli-tools/sw-cli`) |
| `just` | crates.io, prebuilt through cargo-binstall | every recipe in the justfile |
| `trunk` | crates.io, prebuilt through cargo-binstall | `just pages`, the live demo and the Rosetta page |
| `cargo-binstall` | its GitHub release | fetching just and trunk prebuilt instead of compiling them |
| wasm32-unknown-unknown | rustup | the browser builds |
| Emacs (emacs-nox) | apt | the Emacs mode's ERT tests |
| cairosvg, pillow | pip | pictures (frames, PNG conversion) |
| X_eTaL-libraries, X_eTaL-ML | softwarewrighter, cloned beside X_eTaL | the sibling repositories' work and their asks |

The user's tools are built with `cargo install --locked --path` from
shallow clones, never with `sw-install` (CLAUDE.md: never run it unless
asked). Their build directory is `/home/user/.tools-target`, apart from
X_eTaL's `target/`.

## Where things go

```
/home/user/X_eTaL                       this repository
/home/user/x_etal-libraries             the sibling repositories, as cloned
/home/user/x_etal-ml
/home/user/X_eTaL-libraries -> x_etal-libraries   links: the repositories
/home/user/X_eTaL-ML -> x_etal-ml                 name each other ../X_eTaL-...
/home/user/sw-vibe-coding/agentrail-rs  the tools' sources
/home/user/sw-vibe-coding/sw-checklist
/home/user/sw-vibe-coding/sw-markdown-checker
/home/user/sw-cli-tools/reg-rs
/home/user/sw-cli-tools/sw-cli
/root/.cargo/bin                        the installed binaries
```

`HOME_DIR` overrides `/home/user`.

## Known differences from a local machine

- The golden cli-too-deep fails on Linux: it passes a 200 KB expression
  as one command-line argument, and Linux limits a single argument to
  128 KB (MAX_ARG_STRLEN), so the shell cannot start `xetal` ("Argument
  list too long"). macOS has no per-argument limit. Fixing it means
  passing the input in a file; until then the sample gate reports this
  one failure in the cloud.
- The gate's Python checks run in parallel through
  `uv run --no-project`, and on a checkout with no `.venv` they race to
  create it and fail ("A virtual environment already exists"). The
  script creates it once first; a second gate run also passes.
- The sibling repositories are shallow clones (one commit). Fetch
  history when a task needs it: `git fetch --depth=1000 origin main`.
- Pushing to the sibling repositories needs them attached to the
  session with push access (the session's add-repository tool); a
  clone through the anonymous proxy is read-only.

## When it fails

The summary lists each failed step (`clone ...`, `install ...`). A
clone failure is usually the network policy or a transient proxy error;
run the script again. A build failure of one of the user's tools is
usually a moved path dependency (as sw-cli was for the markdown
checker): read the error, clone the missing repository beside it, and
add it to the script.
