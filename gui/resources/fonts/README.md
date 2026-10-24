# Bundled Sora fonts

`Sora.ttf` is the unmodified Sora variable font from
<https://github.com/google/fonts/tree/main/ofl/sora>.
Copyright and SIL Open Font License terms are preserved in `OFL.txt`.

`OpenSCQ30Metrics.ttf` is a renamed derivative, instantiated at weight 600 with
tabular digit glyphs mapped by default. It uses the same license. The reproducible
transformation is `build_metrics.py`; fonttools is only needed to regenerate this
asset, not to build or run the Rust app.
