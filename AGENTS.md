# BitCards Contributor Guide

This file defines the working rules for humans and coding agents contributing to
this repository. Follow it for every change unless a more specific `AGENTS.md` in
a subdirectory overrides it.

## Mission and current scope

BitCards is its own proposed collectible-card network. Cards—not a conventional
coin—are the primary scarce native objects. This is not an NFT layer for an
existing blockchain.

The repository is currently in **Phase 1**: deterministic Card Type generation.
Keep changes focused on the generator, models, canonical encoding, hashing,
rendering, artwork authoring tools, documentation, and tests.

Do not add networking, consensus, chain persistence, wallets, marketplaces, the
playable card game, discovery capsules, staking, mining rewards, or a currency.
Those systems require explicit later design. Do not use an LLM or external service
to generate protocol cards.

## Protocol invariants

Consensus-facing generation must be reproducible byte for byte on every supported
machine.

- Use stable Rust and deterministic integer arithmetic. Do not use floating point
  in generator or canonical protocol logic.
- Derive every random choice from the versioned cryptographic hash stream. Never
  use system randomness, clocks, process state, environment data, network calls,
  filesystem order, or locale in generation.
- Never rely on unordered map/set iteration for canonical results. Sort explicitly
  or use types with a defined order.
- Treat byte order, lengths, string encoding, list order, and domain separators as
  protocol rules. Canonical integers are fixed-width and big-endian; strings are
  deliberate UTF-8 with explicit lengths.
- Every incompatible generator change requires a new generator version. Once a
  version is released, its outputs and test vectors are immutable.
- Display metadata and ANSI escape sequences must never enter canonical Card Type
  bytes or hashes. Renderer changes should not require a generator-version bump
  unless they change canonical artwork.
- Keep dependencies minimal and auditable. Do not introduce a framework, async
  runtime, serialization format, or RNG crate without a concrete need.

The current stream derivation and canonical format are documented in
`docs/architecture.md`. Preserve their domain separation and rejection sampling.

## Domain model

- A **Set** has an ID, lifecycle, future-derived seed, generator version, Card
  Types, rarity rules, and hard supply limits. Activated Sets must be immutable;
  after retirement no more copies from that Set may be minted.
- A **Card Type** is the deterministic shared identity: name, class, rarity,
  supply, stats, attacks, artwork, version, seeds, canonical bytes, and hash.
- A **Card Copy** is a future owned instance with a permanent serial within its
  Card Type. Serial 1 is the unique Origin copy without a special mint path.
- The three-digit catalog number identifies a Card Type's position within its Set.
  It is distinct from a Card Copy serial.

Current proposals such as 100 Card Types per Set, 20 Sets, two-minute blocks, and
roughly ten years of issuance are drafts. Do not silently turn them into fixed
consensus constants.

## Gameplay vocabulary

The six current computer-themed classes and symbols are:

- `[]` Robot
- `//` Glitch
- `<>` Daemon
- `**` Virus
- `{}` Bug
- `()` Null

The prototype advantage loop is Robot → Virus → Daemon → Glitch → Bug → Robot.
Advantage adds 10 damage. Null is neutral. The prototype shared resource is Charge:
players gain one per turn and may store up to six. These rules remain configurable
while Phase 1 is experimental.

## Code boundaries

- `src/model.rs`: strong domain types and configurable rules.
- `src/hash_stream.rs` and `src/sha256.rs`: deterministic randomness and hashing.
- `src/generator.rs` and `src/creature.rs`: consensus-relevant generation.
- `src/canonical.rs`: canonical Card Type encoding and hashing.
- `src/renderer.rs`: terminal presentation only; keep it outside canonical hashes.
- `src/generator_assets/v1/`: embedded, versioned generator assets.
- `generator-assets/drafts/`: editable artwork drafts, not active consensus assets.
- `tools/art-lab/`: non-consensus authoring and validation utilities.
- `tests/determinism_vectors.rs`: locked end-to-end vectors.

Keep the library usable independently of the CLI. Avoid leaking terminal and
preview concerns into domain types.

## Artwork and rendering

Canonical artwork is Unicode text encoded as UTF-8. The current card face is fixed
at 34 columns by 24 rows with a seven-row art panel. Preserve fixed cell width and
test exact output; do not assume all Unicode glyphs have the same terminal width.

Creature families/classes need genuinely different silhouette grammars, not a
single template with swapped decoration. Artwork itself must not contain letters
or numbers. Keep components recognizable and aligned rather than maximizing random
noise.

Use the documented `.bca` format for draft art components. Validate drafts with
the art lab before promoting them. Released versioned assets are immutable; add a
new generator version for incompatible changes.

ANSI color and finish previews are presentation features. They must degrade to a
readable no-color card, respect `NO_COLOR`, and never affect canonical generation.
Do not describe experimental Holo, Reverse Holo, Gold, Rainbow Holo, or rarity
previews as finalized protocol objects.

## Required verification

Complete small changes one at a time and keep intermediate states compiling. Before
handoff or commit, run:

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Add focused unit tests and fixed vectors for consensus-relevant behavior. Never
rewrite an existing determinism vector merely to make an accidental output change
pass. If an intentional incompatible design requires new output, introduce a new
generator version and retain tests for the old one.

The artwork-diversity audit should continue checking many seeds for exact
collisions and silhouette diversity. Visual review still matters: collision-free
noise is not good creature art.

## Repository hygiene

- Commit `Cargo.lock`; this workspace ships binaries.
- Do not commit `target/`, local node data, editor state, logs, `.env` files,
  credentials, private keys, seed phrases, or personal configuration.
- Do not commit generated gallery output unless it is an intentional small test
  fixture.
- Preserve unrelated local changes. Avoid destructive Git commands.
- Keep source and asset files on LF line endings; `.gitattributes` enforces this.
- Update README or the relevant document when behavior, commands, formats, or
  protocol assumptions change.

Use clear commits that separate protocol changes from renderer-only experiments
whenever practical.
