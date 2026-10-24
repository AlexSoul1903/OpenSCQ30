"""Regenerate the bundled metric face: python -m pip install fonttools first."""
from pathlib import Path
from fontTools.ttLib import TTFont
from fontTools.varLib.instancer import instantiateVariableFont

root = Path(__file__).parent
font = instantiateVariableFont(TTFont(root / "Sora.ttf"), {"wght": 600}, inplace=True)
substitutions = {}
for record in font["GSUB"].table.FeatureList.FeatureRecord:
    if record.FeatureTag == "tnum":
        for index in record.Feature.LookupListIndex:
            for subtable in font["GSUB"].table.LookupList.Lookup[index].SubTable:
                substitutions.update(subtable.mapping)
for cmap in font["cmap"].tables:
    if cmap.isUnicode():
        for codepoint, glyph in list(cmap.cmap.items()):
            cmap.cmap[codepoint] = substitutions.get(glyph, glyph)
for record in font["name"].names:
    if record.nameID in {1, 2, 3, 4, 6, 16, 17}:
        name = {1: "OpenSCQ30 Metrics", 2: "SemiBold", 3: "OpenSCQ30Metrics-SemiBold",
                4: "OpenSCQ30 Metrics SemiBold", 6: "OpenSCQ30Metrics-SemiBold",
                16: "OpenSCQ30 Metrics", 17: "SemiBold"}[record.nameID]
        record.string = name.encode(record.getEncoding())
font.save(root / "OpenSCQ30Metrics.ttf")
# Make tabular numerals a build-time assertion, not an assumption about the font.
cmap = font.getBestCmap()
assert len({font["hmtx"][cmap[ord(d)]][0] for d in "0123456789"}) == 1
