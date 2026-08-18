# BitCards

BitCards is an experimental network built around collectible ASCII trading cards.
Cards are intended to be native protocol objects—not NFTs on another chain—and the
network's primary scarce assets. Card generation is deterministic, versioned, open
source, and independently reproducible from its seeds.

BitCards is written in stable Rust. Rust gives the project strong types, explicit
integer and byte semantics, memory safety, and efficient native binaries: useful
properties for deterministic protocol code and, eventually, a long-running node.

> **Current status:** this repository implements only the Phase 1 deterministic
> card-generation foundation. It is not yet a blockchain, node, wallet, or game.
> Networking, consensus, transfers, discovery, and any anti-spam mechanism remain
> future design work. BitCards has no cryptocurrency or token.

## A BitCard

Markdown preserves terminal artwork in fenced text blocks:

```text
╔════════════════════════════════╗
║ CODEBEETLE              HP: 90 ║
║ {} BUG                          ║
╟────────────────────────────────╢
║                                ║
║          .-""""-.              ║
║       __/  ◉  ◉  \__           ║
║      /  \   ▴   /  \           ║
║      \__/|=====|\__/           ║
║         /_/   \_\              ║
║                                ║
╟────────────────────────────────╢
║ Packet Pincer              20  ║
║ Add 10 when Bug has advantage. ║
║                                ║
║ Kernel Shell               30  ║
║ Reduce the next hit by 10.     ║
║                                ║
║ 001 ★★★       SET 01    #000001║
╚════════════════════════════════╝
```

The CLI renders a generated front and the universal card back side by side using
ANSI terminal color.

## Try it

Requirements are stable Rust 1.85 or newer, Cargo, and a native C linker (`cc`,
normally supplied by GCC or Clang).

```sh
cargo test --workspace
cargo run -- card generate --seed 01
cargo run -- card gallery
cargo run -- card command-gallery
cargo run -- card charge-gallery
cargo run -- card promo-gallery
```

`card generate` always reproduces the same Card Type for the same generator version
and seed. `card gallery` intentionally chooses a new preview seed each run and
prints that seed; use `cargo run -- card gallery --seed <printed-seed>` to reproduce
the gallery exactly. This preview convenience does not affect generator
determinism.

`card command-gallery` prints all ten fixed Set 1 Command Card drafts, while
`card charge-gallery` prints the six fixed typed Charge Card drafts. Their names,
effects, and artwork do not vary by seed. These remain game-design material rather
than canonical protocol objects.

`card promo-gallery` prints the fixed, hand-authored launch Promo Card drafts.
Promo 01 is `NETDOGE`, with a proposed hard supply of 1,000 copies. Promo previews
are not procedural and are not activated protocol cards.

For all CLI options:

```sh
cargo run -- card generate --help
cargo run -- card gallery --help
```

## What exists today

- A domain-separated SHA-256 hash stream with unbiased integer selection.
- Versioned deterministic generation of names, classes, rarity, supply, gameplay
  stats, varied attack/ability layouts, and ASCII artwork.
- Exactly three gameplay rarity tiers: Common (`★`), Rare (`★★`), and Super Rare
  (`★★★`).
- Six computer-themed classes: Robot, Glitch, Daemon, Virus, Bug, and neutral Null.
- Canonical binary serialization and cryptographic Card Type hashes.
- A fixed-width terminal renderer with front, back, color, and experimental finish
  previews kept outside consensus-critical data.
- Fixed test vectors, determinism tests, and automated artwork-diversity checks.
- An art lab for validating and previewing draft `.bca` artwork components.

## Core concepts

- A **Set** is a time-limited collection whose generator rules are committed before
  its future seed is known. Activated definitions and supply limits must be
  immutable.
- A **Card Type** is the shared identity, art, stats, and rules generated from a
  Set seed and Card Type seed.
- A **Card Copy** is an individually owned instance of a Card Type. Its permanent
  serial number distinguishes it from other copies; serial `#000001` naturally
  identifies the future Origin copy.

Set cadence, Set size, supply values, and issuance duration are still proposals,
not frozen protocol constants.

## Repository layout

```text
src/                    generator, models, canonical encoding, hashing, renderer
tests/                  fixed cross-run determinism vectors
generator-assets/       draft procedural-art components
tools/art-lab/          non-consensus artwork authoring and validation tool
docs/                   architecture and format notes
```

Read [the architecture contract](docs/architecture.md), [Set and issuance notes](docs/sets-and-issuance.md),
[the draft game rules](docs/game-rules.md), and [the `.bca` artwork format](docs/bca-format.md)
for more detail. Contributor and automation rules live in [AGENTS.md](AGENTS.md).
Release history is recorded in [CHANGELOG.md](CHANGELOG.md).

BitCards is licensed under the [MIT License](LICENSE).
