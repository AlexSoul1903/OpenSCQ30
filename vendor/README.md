# Dependency security override

`cryoglyph` is copied from the exact revision already used by libcosmic:
https://github.com/iced-rs/cryoglyph/tree/e429a025df36ab8145708acb309080ae3deec17a

Its source is unchanged. The only manifest change upgrades `lru` from 0.16 to
0.18.2, the fixed release for RUSTSEC-2026-0253. Original licenses are included.
Remove this override when the upstream renderer requires a fixed LRU release.

`xkbcommon` is the crates.io 0.7.0 source, with only its optional `memmap2`
dependency raised from 0.8.0 to 0.9.11 (RUSTSEC-2026-0186). This protects the
Linux preview without changing iced's keyboard API. Its MIT license is included.
Remove this override when iced upgrades its xkbcommon dependency.
