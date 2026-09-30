# The name: spelling it and saying it

The language has one name and several spellings, each for a place
where the others do not fit. All of them stand for the eXperimental
eXtensible Typed Array Language.

![The name every way: said Ecks-e-tal; the logo; the favicon (an italic
underlined X with a raised ellipsis); XeTaL in prose; X_eTaL and X_ e:T
a:L typed and drawn; LaTeX; the binary xetal; the file type .xtl; the
repository](../images/name-forms.png)

The picture is generated: `scripts/name-image.sh` draws it, the drawn
and LaTeX forms by `xetal render` itself.

## Saying it

**Ecks-e-tal** (preferred): the X as the letter, "ecks", then "e",
then "tal", as in "total" without the "to". It follows the file type:
`.xtl` is said eks-tee-ell, and XeTaL is the same letters with vowels
between them.

People meeting the name cold may try "zee-tal" (X as in xylophone) or
"shee-tal" (X as in Spanish or Chinese). Neither is wrong English, but
neither is the name: say Ecks-e-tal.

## Spelling it

| Spelling | Where | What it is |
| -------- | ----- | ---------- |
| the logo | the README, the live demo, `images/modern-xetal-logo.jpg` | X underlined, then a raised e, T, a raised a, L |
| the favicon | the browser tab of the live demo (`favicon.ico`) | an italic underlined X with a raised ellipsis, black on yellow: the logo's X, the rest implied |
| `X_ e:T a:L` | typed as X_eTaL source | the logo as XeTaL itself writes it (below) |
| `X_eTaL` | the display name, headings, the repository | the logo in plain ASCII: the `_` marks the underline |
| XeTaL | prose | the name in a sentence, with no marks at all |
| `X_eTaL` | `github.com/softwarewrighter/X_eTaL`, the live demo's URL | the repository and GitHub Pages site |
| `xetal` | the binary, the crates (`xetal-*`), the code | the slug: lowercase, no marks, safe everywhere |
| `.xtl` | file names (`demos/life.xtl`, `lib/Stats.xtl`) | the file type: the consonants of the slug |

Inside the implementation the display name is written once, as
`xetal_base::LANG_NAME` (`"X_eTaL"`), so renaming it is a one-line
change; everything else uses the slug.

## The logo is XeTaL

The logo is not only a picture. Written as XeTaL source, an underscore
after a letter underlines it, and a name with a one-letter prefix and a
colon shows the prefix raised:

```
X_eTaL        drawn as an underlined X, then eTaL
X_ e:T a:L    drawn as the logo: an underlined X, then T with a raised
              e, then L with a raised a
```

(`xetal render -e 'X_ e:T a:L'` draws it; the second form is three
names: X underlined, and the names T and L in the namespaces e: and
a:.) So the variations in the logo are the language's own marks:

- **X underlined**: an underlined letter makes a name a function; X_ is
  the name's one function.
- **e and a raised**: a raised letter is a namespace prefix, as `u:`
  for your own names is drawn as a raised u.
- **Plain or marked**: `X_eTaL` keeps only the underline, as typed in
  ASCII; XeTaL drops every mark, for prose; both are the same name.

The ASCII spelling and the drawn one are related the way all XeTaL is:
the ASCII is what you type and what the tools read, and the typography
is how it is shown. That is also the name's joke: LaTeX, reversed.
