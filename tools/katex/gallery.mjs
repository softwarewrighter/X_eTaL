// Every expression we ship or document, through `xetal render --latex`
// and then KaTeX: a line whose LaTeX KaTeX rejects is a failure. With
// --write, the same run draws pages/latex/index.html: each line as
// typed, its LaTeX, and KaTeX's rendering, grouped by file.
//
//   node tools/katex/gallery.mjs [--write]      (scripts/latex-gallery.sh)
//
// Sources: demos/, lib/ and userlibs/ (every line of code), the xetal
// blocks of docs/literate/*.org, the session input lines (six spaces in)
// of the README's and docs/*.md's session blocks, and the SOURCE of every
// spec case (a case whose source does not lex, by design, is skipped).

import { spawnSync } from 'node:child_process';
import { cpSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import katex from 'katex';

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');
const xetal = process.env.XETAL_BIN || join(root, 'target/release/xetal');
const write = process.argv.includes('--write');

const files = (dir, ext) =>
  readdirSync(join(root, dir), { recursive: true })
    .filter((f) => f.endsWith(ext))
    .sort()
    .map((f) => join(dir, f));
const code = (line) => line.trim() !== '' && !line.trim().startsWith('#');
const read = (f) => readFileSync(join(root, f), 'utf8');

/** [file, line number, source line, may fail to lex] for every source. */
function lines() {
  const out = [];
  const add = (file, n, text, mayFail = false) => code(text) && out.push([file, n, text, mayFail]);
  for (const f of [...files('demos', '.xtl'), ...files('lib', '.xtl'), ...files('userlibs', '.xtl')]) {
    read(f).split('\n').forEach((l, i) => add(f, i + 1, l));
  }
  for (const f of files('docs/literate', '.org')) {
    let inside = false;
    read(f).split('\n').forEach((l, i) => {
      if (l.startsWith('#+begin_src xetal')) inside = true;
      else if (l.startsWith('#+end_src')) inside = false;
      else if (inside) add(f, i + 1, l);
    });
  }
  for (const f of ['README.md', ...files('docs', '.md').filter((f) => !f.includes('/', 5))]) {
    // A block is a session when its first line is input (six spaces in).
    let inside = false;
    let session = false;
    read(f).split('\n').forEach((l, i) => {
      if (l.startsWith('```')) [inside, session] = [!inside, null];
      else if (inside && session === null) session = /^ {6}\S/.test(l);
      if (inside && session && /^ {6}\S/.test(l)) add(f, i + 1, l.slice(6));
    });
  }
  for (const f of files('spec', '.case')) {
    const text = read(f);
    const m = text.match(/== SOURCE\n([\s\S]*?)(?=\n== |$)/);
    if (!m) continue;
    const at = text.slice(0, m.index).split('\n').length + 1;
    m[1].split('\n').forEach((l, i) => add(f, at + i, l, text.includes('== ERROR')));
  }
  return out;
}

/** The line's LaTeX, or null when it does not lex. */
function latex(text) {
  const r = spawnSync(xetal, ['render', '--latex', '-e', text], { encoding: 'utf8' });
  return r.status === 0 ? r.stdout.replace(/\n$/, '') : null;
}

// Strict, except that a character KaTeX has no glyph for (a string may
// hold any Unicode) is drawn from the page's font.
const strict = (code) => (code === 'unknownSymbol' ? 'ignore' : 'error');
const warn = console.warn;
console.warn = (m, ...rest) => String(m).startsWith('No character metrics') || warn(m, ...rest);
const esc = (s) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
const rows = new Map();
const failures = [];
let checked = 0;
for (const [file, n, text, mayFail] of lines()) {
  const tex = latex(text);
  if (tex === null) {
    if (!mayFail) failures.push(`${file}:${n}: xetal render --latex failed: ${text}`);
    continue;
  }
  let html;
  try {
    html = katex.renderToString(tex, { throwOnError: true, displayMode: true, strict });
  } catch (e) {
    failures.push(`${file}:${n}: KaTeX rejects ${tex}\n    (typed: ${text})\n    ${e.message}`);
    html = `<span class="bad">${esc(e.message)}</span>`;
  }
  checked += 1;
  if (!rows.has(file)) rows.set(file, []);
  rows.get(file).push(`<tr><td class="n">${n}</td><td><code>${esc(text)}</code></td>` +
    `<td><code class="tex">${esc(tex)}</code></td><td>${html}</td></tr>`);
}

if (write) {
  const out = join(root, 'pages/latex');
  mkdirSync(out, { recursive: true });
  const dist = join(root, 'tools/katex/node_modules/katex/dist');
  cpSync(join(dist, 'katex.min.css'), join(out, 'katex.min.css'));
  cpSync(join(dist, 'fonts'), join(out, 'fonts'), { recursive: true });
  const groups = [...rows].map(([f, r]) =>
    `<h2 id="${esc(f)}">${esc(f)}</h2>\n<table>\n${r.join('\n')}\n</table>`).join('\n');
  const index = [...rows.keys()].map((f) => `<a href="#${esc(f)}">${esc(f)}</a>`).join(' &middot; ');
  writeFileSync(join(out, 'index.html'), `<!doctype html>
<html lang="en"><head><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>X_eTaL in LaTeX</title>
<link rel="stylesheet" href="katex.min.css">
<link rel="stylesheet" href="../literate/style.css">
<style>
#content { max-width: 80rem; }
table { border-collapse: collapse; width: 100%; font-size: 15px; }
td { border-top: 1px solid var(--code); padding: 4px 8px; vertical-align: middle; }
td.n { color: var(--dim); text-align: right; width: 3em; }
code.tex { font-size: 12px; color: var(--dim); word-break: break-all; }
.katex-display { margin: 0; text-align: left; }
.bad { color: #ff6b6b; }
</style></head>
<body><div id="content">
<h1 class="title">X_eTaL in LaTeX</h1>
<p>Every line of code we ship or document (${checked} lines), as typed, as
<code>xetal render --latex</code> writes it, and as KaTeX draws it. The
gate checks that KaTeX accepts every one.</p>
<p>${index}</p>
${groups}
</div>
<div id="postamble"><p><a href="../literate/">Literate documents</a> &middot;
<a href="../">Live demo</a> &middot;
<a href="https://github.com/softwarewrighter/X_eTaL">Repository</a></p></div>
</body></html>
`);
}

console.log(`latex: ${checked} lines through KaTeX, ${failures.length} rejected`);
for (const f of failures) console.log(f);
process.exit(failures.length === 0 ? 0 : 1);
