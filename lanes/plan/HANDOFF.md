# plan lane: handoff for the local agent

The plan lane (branch `feat/integrals-plan-oo9lu7`, a cloud sandbox)
takes the small independent steps of the saga: 114 integrals-plan,
119 pages-gh-pages, 120 doc-comment-glyphs. The sandbox has no
agentrail, just, reg-rs, sw-checklist, sw-markdown-checker or trunk,
so it keeps no `lanes/plan/.agentrail`; record the steps locally after
merging, as the lint lane did.

## Step 114 integrals-plan

Documents only: docs/plan.md gains Saga 42 (integrals), Saga 41 points
at it, CHANGES.md has the entry. docs/convolutions/ was already
committed as given. No file under components/ changed for the step.
The sandbox ran `scripts/check-spelling.py` (ok) and checked plan.md
is ASCII by grep; not run: `sw-markdown-checker`.

To do locally:

1. agentrail: `agentrail begin` and `agentrail complete` for step 114
   with this branch's commit.
2. `just pages` if a part is stale (plan.md is not a page input, so
   probably none).

## Step 119 pages-gh-pages (D162)

pages/ is gitignored and removed from the index (`git rm -r --cached`,
history untouched); scripts/publish-pages.sh (`just publish`),
scripts/seed-pages.sh, check-pages.sh's warning when origin/gh-pages is
behind, the Pages workflow deleted; CLAUDE.md (AGENTS.md is a link to
it), docs/vendoring.md, docs/local-asks.md, README, docs/plan.md, the
register (D162; renumber if the local step 110 took D162 first).

The sandbox could not publish or change GitHub settings. Merging is
safe on its own: with the workflow gone, nothing redeploys, and the
site last deployed stays live until the switch. Pulling the merge
deletes the tracked pages/ files from a checkout; the next
`just pages` seeds them back from the parent of the commit that
removed them (no gh-pages branch exists yet), so only stale parts
rebuild.

To do locally, in order, after merging into main and pushing:

1. `just pages` (seeds pages/, rebuilds what is stale), `just gate`.
2. `just publish --dry-run`, look at the commit, then `just publish`.
3. Switch Pages to the branch:
   `gh api -X PUT repos/softwarewrighter/X_eTaL/pages -f build_type=legacy -f 'source[branch]=gh-pages' -f 'source[path]=/'`
4. Check https://softwarewrighter.github.io/X_eTaL/ and its /doc/.
5. Update the memory notes that say pages/ is committed.
6. agentrail: record step 119 with the merge's commit.

Tested in the sandbox: the orphan tree publish-pages.sh builds from
pages/ equals the tree main tracked (same hash); seed-pages.sh restores
pages/ byte for byte from the removal commit's parent; bash -n on both
scripts. Not run: a real publish, trunk, the gate.
