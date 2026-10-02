#!/bin/bash
cd ../..
./scripts/syntax-poster.sh

# internally, for each snippet:
xetal render --html -e 'u:s_quare := { _r * _r }'

# insert that HTML verbatim into the appropriate poster slot

# finally:
chrome --headless ... screenshot poster.html
