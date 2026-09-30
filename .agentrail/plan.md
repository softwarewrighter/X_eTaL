# web-playground

Saga 10 of X_eTaL (docs/plan.md), milestone M9, moved first at the
user's request to show the work in progress: a live demo of the
editor in the browser, Rust + Yew compiled to WASM, built locally into
./pages and deployed by a GitHub Actions workflow that uploads the
committed pages/ (nothing is built on GitHub), linked from the README.

Model: ../../sw-embed/web-sw-tos (its .github/workflows/pages.yml,
scripts/build-pages.sh, build.rs capturing BUILD_SHA / BUILD_HOST /
BUILD_TIMESTAMP, src/chrome.rs footer) and sw-mlpl's live demo.

Decided with the user:
- It is the terminal editor, not a new design: the ASCII pane, the
  decorated pane, the types/output pane, Tab between panes, Ctrl-T
  zoom, Ctrl-R run, the same colors; plus a drop-down of the canned
  .xtl files (demos and the tour), buttons (Run, Help), a Help dialog
  (how the editor works: cursor, Tab, Ctrl-T, Ctrl-R) closed by its
  corner X, a background click or Escape.
- The footer is like the other live demos: copyright, MIT License, a
  link to the repository, a link to the literate HTML documents'
  index page, and build info (build host, short commit SHA, build
  timestamp).
- Load and save in the browser's local storage (like the sw-apl live
  demo's workspaces); []R_EAD reads from a prompt and files live in
  local storage, so TTTML can train and play in the browser.
- The standard libraries are bundled (the tour uses Combinators).
- The literate documents (tour, birds, tttml) exported to HTML under
  pages/literate/ with an index page, the footer's Literate link.

Rules: strict TDD, tests are the spec, no panics, stricter gates and
design for them up front, docs ASCII-only, gate before commit, commit
.agentrail with the work, push.

## Steps

1. wasm-core -- the pipeline for wasm32: a single-threaded evaluator
   path (no worker thread), libraries in memory only, a small API
   (decorate, check, run) tested natively.
2. yew-app -- components/web: the three panes, drop-down, Run, Tab,
   Ctrl-T, Ctrl-R, the TUI colors; trunk serve locally.
3. pages -- scripts/build-pages.sh, .github/workflows/pages.yml, the
   Help dialog, the footer (build.rs provenance), README link: the
   first live demo.
4. browser-io -- local-storage workspaces, []R_EAD via a prompt,
   files in local storage (TTTML trains and plays in the browser).
5. literate-html -- Org HTML exports of the literate documents with an
   index page, linked from the footer and the README.
6. web-release -- docs (architecture, README, a tour page), a
   screenshot of the live demo, retrospective.
