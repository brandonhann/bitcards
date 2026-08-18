# Changelog

All notable changes to BitCards will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project intends to follow [Semantic Versioning](https://semver.org/).

## Unreleased

### Added

- Stable Rust and Cargo project structure for the BitCards generator and art lab.
- Versioned deterministic hash stream based on SHA-256.
- Strong models for Card Types, rarity, computer-themed classes, attacks, and
  configurable generation rules.
- Deterministic generation of names, gameplay statistics, attacks, supply, and
  ASCII creature artwork.
- Robot, Glitch, Daemon, Virus, Bug, and Null creature generators.
- Canonical binary serialization and cryptographic Card Type hashes.
- Fixed-width terminal card renderer with a universal card back and ANSI color.
- Display-only rarity and card-finish previews.
- CLI commands for generating individual cards and previewing card galleries.
- Fixed determinism vectors, artwork collision checks, and renderer tests.
- `.bca` draft artwork format and validation/gallery tooling.
- Architecture, Set issuance, artwork-format, and contributor documentation.

### Changed

- Consolidated the protocol rarity model to Common, Rare, and Super Rare; removed
  the obsolete Uncommon, Legendary, and Mythic variants.
- Simplified fixed Command and Charge previews by removing their unused seed input.
- Replaced the generator's fallible positional creature list with an exhaustive
  class-to-generator mapping.
- Generator version 2 adds strongly typed attacks and passive abilities.
- Cards now deterministically receive one strong attack, an ability plus an
  attack, or two attacks while preserving a fixed-height card layout.
- Expanded every class to eight attack names and six ability names, with varied
  deterministic effects.
- Generator version 1 and its locked Card Type hash vector remain reproducible.
- Documented the early BitCard, Charge Card, and Command Card game rules.
- Added initial non-consensus Command artwork drafts.
- Added separate `card command-gallery` and `card charge-gallery` commands for
  fixed support-card previews.
- Updated support previews with fully colored borders, dark artwork fields, and
  filled information panels matching the standard BitCard visual structure.
- Simplified the CLI to render cards in color without color-selection flags.
- Changed all Command previews to category-specific grayscale palettes.
- Replaced generic Charge previews with Robot, Glitch, Daemon, Virus, Bug, and
  Null Charge cards using their matching class palettes.
- Gave each Charge type one permanent full-art design with no effect panel or
  repeated category label.
- Replaced support-card draft labels with common-rarity catalog, Set, and copy
  identity footers.
- Replaced symbolic Charge illustrations with centered oversized block renderings
  of each type symbol: `[]`, `//`, `<>`, `**`, `{}`, and `()`.
- Swapped the Glitch and Daemon palettes across standard cards, finishes, text,
  borders, artwork fields, and typed Charge cards: Glitch is purple and Daemon red.
- Standardized all Charge symbols at seven rows tall, restored clear full-width
  glyph shapes, and added visible separation between the Daemon brackets.
- Rendered every Charge symbol in a bright shade of its matching type color.
- Replaced generic Command illustrations with effect-specific terminal displays
  and simplified Command headers to show only each card's name.
- Standardized Charge and Command card frames and separators to white while
  preserving their distinct interior palettes.
- Replaced randomized Command previews with ten fixed Set 1 Command Cards, each
  with authored rules text and a matching permanent terminal display.
- Added monochrome display-only Rare CRT and Super Rare root-terminal finishes for
  Command copies without creating additional Card Types or changing gameplay effects.
- Unified every Standard Command Card under one consistent gray palette.
- Added display-only Pulse Rare and inverted Overcharged Super Rare finishes for
  all six fixed Charge types while preserving their gameplay identity.
- Replaced textual `CHG` attack costs with a class symbol followed by an explicit
  number, removed bullet separators, and removed `PASSIVE` labels.
- Changed Null attack costs to the neutral `◇` symbol, payable with any non-Null
  Charge, while retaining the experimental Null Charge Card for later design.
- Added the fixed `NETDOGE` Promo 01 draft and a dedicated `card promo-gallery`
  command, with a proposed hard supply of 1,000 copies.
- Redrew NETDOGE with a wider Shiba-style face and gave Promo cards the exclusive
  `✦` footer mark instead of a normal rarity star.
- Converted NETDOGE to a full-art Promo layout with bright gameplay text layered
  directly over the lower artwork instead of using separate information boxes.
- Centered the complete NETDOGE artwork below its header, restored the Null
  star-field background, and matched overlay cells to the artwork's exact gold.
- Restored NETDOGE's exact supplied glyphs, corrected the eye mask to the two real
  `░░░` eye groups, colored those eyes white, and kept the remaining artwork gold.
