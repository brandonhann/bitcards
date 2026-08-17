# BitCards Art (`.bca`) format, version 1

`.bca` is a deliberately small development format for fixed-grid ASCII components.
It is consumed by `tools/art-lab` and is not read from disk by consensus code.

```text
BITCARDS-ART 1
id bug.body.wide-01
size 21 7
anchor head 10 1
---
<exactly seven rows of exactly 21 cells>
```

Rules:

- Files are UTF-8 with LF line endings and must contain ASCII artwork only.
- Backtick (`` ` ``) is a transparent cell and renders as a space.
- Letters and digits are forbidden inside the artwork grid.
- IDs and anchor names use lowercase ASCII, digits, `.` and `-`.
- Coordinates are zero-based and must fall inside the declared grid.
- Tabs, invisible spaces, duplicate anchors, and incorrect dimensions are rejected.
- Approved assets will be embedded and hashed before use by a generator version.
