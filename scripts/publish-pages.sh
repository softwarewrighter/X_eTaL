#!/usr/bin/env bash
# Publish the site: pages/ (built locally, untracked) becomes one orphan
# commit force-pushed to the gh-pages branch, which GitHub Pages serves.
# Only main is published, so the site always names a main commit:
# refuses a dirty tree and a HEAD that is not on origin/main. Stale
# parts are rebuilt first (scripts/build-pages.sh), so the published
# INPUTS stamps match the commit.
#   scripts/publish-pages.sh            # build what is stale, then publish
#   scripts/publish-pages.sh --dry-run  # build and make the commit, do not push
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
push=yes
[ "${1:-}" = "--dry-run" ] && push=no

if [ -n "$(git status --porcelain)" ]; then
    echo "publish-pages: the tree is dirty; commit or stash first" >&2
    exit 1
fi
git fetch -q origin main
if ! git merge-base --is-ancestor HEAD origin/main; then
    echo "publish-pages: HEAD is not on origin/main; publish from a pushed main" >&2
    exit 1
fi
git fetch -q origin gh-pages 2> /dev/null || true
scripts/seed-pages.sh
scripts/build-pages.sh
scripts/check-pages.sh

main="$(git rev-parse --short HEAD)"
index="$(mktemp)"
trap 'rm -f "$index"' EXIT
rm -f "$index"
# A tree of pages/ alone, from a throwaway index: the working tree and
# the real index are untouched. -f: the repository's ignore rules are
# about the source tree, not the site.
gitdir="$(git rev-parse --absolute-git-dir)"
tree="$(cd pages && export GIT_DIR="$gitdir" GIT_WORK_TREE=. GIT_INDEX_FILE="$index" && git add -A -f . && git write-tree)"
commit="$(git commit-tree "$tree" -m "pages: the site built from main $main")"
if [ "$push" = no ]; then
    echo "publish-pages: built $commit (main $main); not pushed (--dry-run)"
    exit 0
fi
git push -f origin "$commit:refs/heads/gh-pages"
echo "publish-pages: gh-pages is the site of main $main ($commit)"
