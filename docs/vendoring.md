# Using X_eTaL from another repository

How a downstream repository (X_eTaL-libraries, X_eTaL-demos,
X_eTaL-games, X_eTaL-ML, X_eTaL-extensions, or one of your own) gets
the `xetal` binary and crates it builds on.

A downstream repository does not track a copy of X_eTaL's source. It
tracks one line, the known-good commit, and keeps everything else out
of git: it clones this repository into a gitignored work directory,
checks that commit out, builds it, and reaches the fresh binary through
a symlink.

## Layout

```
your-repo/
  XETAL_COMMIT        tracked: the known-good X_eTaL commit (a full SHA)
  scripts/xetal.sh    tracked: the script below
  .gitignore          lists work/ and bin/xetal
  work/xetal/         ignored: a clone of X_eTaL at that commit
  bin/xetal           ignored: a symlink to the built binary
```

`.gitignore`:

```
work/
bin/xetal
```

`XETAL_COMMIT` holds one line, the commit to build:

```
c74ea8d922995fb20f66fa9ef7b39ebe54ce3bc9
```

## The script

`scripts/xetal.sh` clones (once), checks out the known-good commit,
builds the release binary and points `bin/xetal` at it. It is safe to
run again at any time: with nothing to do it only confirms the build.

```bash
#!/usr/bin/env bash
# Get xetal: clone X_eTaL into work/xetal (gitignored), check out the
# known-good commit in XETAL_COMMIT, build the release binary, and
# symlink bin/xetal to it.
#   scripts/xetal.sh
#   XETAL_SOURCE=../X_eTaL scripts/xetal.sh   # clone from a local checkout
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
commit="$(tr -d '[:space:]' < "$root/XETAL_COMMIT")"
source="${XETAL_SOURCE:-https://github.com/softwarewrighter/X_eTaL.git}"
clone="$root/work/xetal"

if [ ! -d "$clone/.git" ]; then
    mkdir -p "$root/work"
    git clone --quiet "$source" "$clone"
fi
if ! git -C "$clone" cat-file -e "$commit^{commit}" 2>/dev/null; then
    git -C "$clone" fetch --quiet origin
fi
git -C "$clone" checkout --quiet --detach "$commit"

(cd "$clone/components/cli" && cargo build --quiet --release -p xetal-cli)

mkdir -p "$root/bin"
ln -sfn "../work/xetal/target/release/xetal" "$root/bin/xetal"
"$root/bin/xetal" --version | grep Commit
```

The clone is a real git repository, so the binary records its own
commit: `bin/xetal --version` prints the first seven characters of
`XETAL_COMMIT` on its `Commit:` line. (No `XETAL_BUILD_SHA` is needed;
that variable is only for building from a copy that is not a clone.)

The first build compiles everything and takes a few minutes; later
runs are incremental. The clone keeps its own `target/` directory.
`XETAL_SOURCE` may name a local checkout (`../X_eTaL`) to clone
without the network; the commit must exist there.

## Recipes

In the downstream `justfile`:

```
# Get and build xetal at the known-good commit (work/xetal, bin/xetal)
xetal:
    scripts/xetal.sh

# Run a program with the known-good xetal
run FILE: xetal
    bin/xetal run {{FILE}}
```

Every other recipe and script calls `bin/xetal`, never a `xetal` found
on `PATH`, so the repository's results do not depend on what happens to
be installed.

## Libraries and crates

The standard libraries (`lib/*.xtl`, `lib/*.xtlm`, `System.xtlm`) are
built into the binary: nothing else is needed at run time. A
repository's own libraries are found beside the importing file, in
`userlibs/`, or through `XETAL_PATH`.

A web demo that builds on X_eTaL's crates names them by path inside
the clone:

```toml
xetal-play = { path = "../../work/xetal/components/web/crates/xetal-play" }
```

Because `work/` is ignored, run `just xetal` before the first build of
such a crate (make the demo's build recipe depend on it).

## Moving to a newer X_eTaL

1. Put the new commit's full SHA in `XETAL_COMMIT`.
2. Run `just xetal`.
3. Run the repository's own gate (tests, goldens, pages).
4. Commit `XETAL_COMMIT` with whatever the new version changed.

The tracked SHA is the record of which X_eTaL the repository is known
to work with: anyone who checks the repository out and runs
`just xetal` builds exactly that version. To go back, restore the old
SHA and run `just xetal` again.

## Published pages

Pages are built locally and committed (`pages/`), so the hosting
workflow uploads them without building: it needs neither the clone nor
the binary.

## Replacing a tracked vendor/xetal

A repository that tracks a copy of X_eTaL under `vendor/xetal`:

1. Write the commit from `vendor/xetal/VENDORED` (or a newer
   known-good one) into `XETAL_COMMIT`.
2. Add `scripts/xetal.sh`, the recipes and the `.gitignore` lines.
3. Change paths: `vendor/xetal/...` becomes `work/xetal/...`, and the
   built binary is `bin/xetal`.
4. `git rm -r vendor/xetal`, run `just xetal`, then the gate.
