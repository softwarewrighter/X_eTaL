#!/usr/bin/env bash
# Set up a fresh cloud session (claude.ai/code) to run X_eTaL's full
# process: the agentrail saga, the gates and the pages. Documented in
# docs/cloud-setup.md (what it installs, where, and why).
#
# Use it as the cloud environment's setup script (environment menu,
# Edit, Setup script):
#
#   bash /home/user/X_eTaL/scripts/cloud-setup.sh
#
# or run it by hand at the start of a session. It is idempotent: a tool
# already on PATH is skipped, a repository already cloned is fetched,
# so a second run takes seconds.
#
# Installs:
#   the user's tools, built from their repositories (never sw-install):
#     agentrail (sw-vibe-coding/agentrail-rs), reg-rs (sw-cli-tools),
#     sw-checklist and sw-markdown-checker (sw-vibe-coding forks; the
#     checker needs sw-cli-tools/sw-cli beside it)
#   just and trunk (prebuilt, through cargo-binstall; cargo install as
#     the fallback), the wasm32 target, Emacs (ERT tests), cairosvg and
#     pillow (pictures)
#   the sibling repositories X_eTaL-libraries and X_eTaL-ML beside
#     X_eTaL, with the mixed-case names the repositories use for each
#     other (../X_eTaL-libraries) linked to the cloned paths
# Environment: HOME_DIR (default /home/user) is where repositories go.
set -uo pipefail

home="${HOME_DIR:-/home/user}"
export PATH="$HOME/.cargo/bin:$PATH"
failed=()

step() { printf '\n==> %s\n' "$*"; }
have() { command -v "$1" >/dev/null 2>&1; }
note_failure() { failed+=("$1"); echo "FAILED: $1" >&2; }

# Clone (shallow) or fast-forward a public repository to $home/$2.
repo() {
    local slug="$1" dir="$home/$2"
    if git -C "$dir" rev-parse HEAD >/dev/null 2>&1; then
        git -C "$dir" pull -q --ff-only 2>/dev/null || echo "kept $dir as it is"
    else
        GIT_LFS_SKIP_SMUDGE=1 git clone -q --depth 1 "https://github.com/$slug" "$dir" \
            || note_failure "clone $slug"
    fi
}

# Build and install a binary from a crate path unless it is on PATH.
tool() {
    local bin="$1" path="$home/$2"
    if have "$bin"; then echo "$bin: $("$bin" --version 2>&1 | head -1)"; return; fi
    (cd "$path" && CARGO_TARGET_DIR="$home/.tools-target" cargo install -q --locked --path .) \
        || note_failure "install $bin"
}

# A crates.io binary: prebuilt through cargo-binstall, else built.
binary() {
    local bin="$1" crate="$2"
    if have "$bin"; then echo "$bin: $("$bin" --version 2>&1 | head -1)"; return; fi
    if have cargo-binstall; then
        cargo binstall -q -y --locked "$crate" && return
    fi
    CARGO_TARGET_DIR="$home/.tools-target" cargo install -q --locked "$crate" \
        || note_failure "install $bin"
}

step "system packages: emacs (ERT tests)"
if ! have emacs; then
    (apt-get update -qq && apt-get install -y -qq emacs-nox) >/dev/null 2>&1 \
        || note_failure "apt-get install emacs-nox"
fi

step "Rust: the wasm32 target (live demo)"
rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || note_failure "wasm32 target"

step "Python: cairosvg and pillow (pictures)"
python3 -c "import cairosvg, PIL" 2>/dev/null \
    || pip install -q cairosvg pillow >/dev/null 2>&1 || note_failure "pip cairosvg pillow"

step "cargo-binstall (prebuilt just and trunk)"
if ! have cargo-binstall; then
    url=https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
    curl -fsSL "$url" | tar -xz -C "$HOME/.cargo/bin" 2>/dev/null \
        || echo "no cargo-binstall: just and trunk will be built (slower)"
fi

step "the user's tools, from source"
repo sw-vibe-coding/agentrail-rs sw-vibe-coding/agentrail-rs
repo sw-cli-tools/reg-rs sw-cli-tools/reg-rs
repo sw-vibe-coding/sw-checklist sw-vibe-coding/sw-checklist
repo sw-vibe-coding/sw-markdown-checker sw-vibe-coding/sw-markdown-checker
repo sw-cli-tools/sw-cli sw-cli-tools/sw-cli
tool agentrail sw-vibe-coding/agentrail-rs/crates/agentrail-cli
tool reg-rs sw-cli-tools/reg-rs/components/app/crates/bin
tool sw-checklist sw-vibe-coding/sw-checklist/components/checklist-cli/crates/sw-checklist
tool sw-markdown-checker sw-vibe-coding/sw-markdown-checker

step "just and trunk"
binary just just
binary trunk trunk

step "sibling repositories"
repo softwarewrighter/X_eTaL-libraries x_etal-libraries
repo softwarewrighter/X_eTaL-ML x_etal-ml
[ -e "$home/X_eTaL-libraries" ] || ln -s x_etal-libraries "$home/X_eTaL-libraries"
[ -e "$home/X_eTaL-ML" ] || ln -s x_etal-ml "$home/X_eTaL-ML"

step "X_eTaL's Python environment (created once, so the gate's parallel checks do not race)"
if [ -d "$home/X_eTaL/scripts" ] && have uv; then
    (cd "$home/X_eTaL" && uv run --quiet --no-project python -c "pass") >/dev/null 2>&1 || true
fi

step "summary"
for b in agentrail reg-rs sw-checklist sw-markdown-checker just trunk emacs; do
    printf '  %-20s %s\n' "$b" "$(command -v "$b" || echo MISSING)"
done
if [ "${#failed[@]}" -gt 0 ]; then
    printf 'failed: %s\n' "${failed[@]}" >&2
    exit 1
fi
echo "cloud setup complete"
