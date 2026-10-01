#!/usr/bin/env python3
"""Turn an animated picture (an SVG drawn by []G_RID or []P_ATH, its
frames shown in turn) into an animated WebP, for places that do not
play SVG animation: some markdown renderers, slides, chat.

    scripts/frames-to-webp.py PICTURE.svg [OUT.webp] [--width PX]

Each frame is drawn on its own (the others hidden), rasterized with
cairosvg and joined with Pillow, 0.4 s a frame as in the SVG, looping.
A still picture becomes a one-frame WebP. Needs Python's cairosvg and
Pillow (pip install cairosvg pillow; cairosvg needs the cairo library).
"""

import re
import sys
from io import BytesIO
from pathlib import Path

FRAME = re.compile(r'<g opacity="[01]"><animate [^>]*/>\n(.*?)</g>\n', re.DOTALL)
FRAME_MS = 400


def frames(svg):
    """The picture once per frame, that frame alone and not animated."""
    bodies = FRAME.findall(svg)
    if not bodies:
        return [svg]
    start, end = FRAME.search(svg).start(), svg.rfind("</g>\n") + len("</g>\n")
    return [svg[:start] + body + svg[end:] for body in bodies]


def raster(svg, width):
    import cairosvg
    from PIL import Image
    png = cairosvg.svg2png(bytestring=svg.encode(), output_width=width)
    return Image.open(BytesIO(png)).convert("RGB")


def main(args):
    width = None
    if "--width" in args:
        i = args.index("--width")
        width = int(args[i + 1])
        del args[i:i + 2]
    if not args or len(args) > 2:
        sys.exit(__doc__.strip().splitlines()[3].strip())
    source = Path(args[0])
    out = Path(args[1]) if len(args) == 2 else source.with_suffix(".webp")
    images = [raster(f, width) for f in frames(source.read_text())]
    images[0].save(out, save_all=True, append_images=images[1:],
                   duration=FRAME_MS, loop=0, lossless=True)
    print(f"{out}: {len(images)} frame(s)")


if __name__ == "__main__":
    main(sys.argv[1:])
