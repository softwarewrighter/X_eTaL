#!/usr/bin/env bash
# Print the literate documents' index page: each document's title and
# subtitle, linked, in reading order.
#   scripts/literate-index.sh > pages/literate/index.html
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
field() { sed -n "s/^#+$1: *//p" "$2" | head -1; }
cat <<'HEAD'
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8"/>
<meta name="viewport" content="width=device-width, initial-scale=1"/>
<title>X_eTaL literate documents</title>
<link rel="stylesheet" href="style.css"/>
</head>
<body>
<div id="content">
<h1 class="title">Literate documents</h1>
<p>Worked examples written in Org mode: prose and X_eTaL blocks, each
block run by <code>xetal</code> through <code>ob-xetal</code> in Emacs
and its result recorded under it.</p>
<ul class="index">
HEAD
for name in tour hello life libraries birds tttml classics hanoi duck; do
    doc="$root/docs/literate/$name.org"
    echo "<li><a href=\"$name.html\">$(field TITLE "$doc")</a><br/>$(field SUBTITLE "$doc")</li>"
done
cat <<'TAIL'
</ul>
<p><a href="../latex/">XeTaL in LaTeX</a>: every line of code we ship or
document, as xetal render --latex writes it and KaTeX draws it.</p>
<p>The Org sources are in the repository's docs/literate/, with the
Emacs mode and ob-xetal in docs/emacs/.</p>
</div>
<div id="postamble"><p><a href="../">Live demo</a> &middot;
<a href="https://github.com/softwarewrighter/X_eTaL">Repository</a></p></div>
</body>
</html>
TAIL
