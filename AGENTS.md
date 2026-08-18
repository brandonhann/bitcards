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
  supply, stats, actions, artwork, version, seeds, canonical bytes, and hash.
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

The draft game has three categories: BitCards battle, Charge Cards build a shared
reusable resource pool, and one-use Command Cards provide support effects. The
initial deck proposal is 40 cards with no more than three copies of one Card Type,
a five-card opening hand, one Active BitCard, and up to three Benched BitCards.

Players may install one Charge Card per turn, up to six. Installed Charge is shared
by every friendly BitCard, attacks exhaust it, and it refreshes at the beginning of
the owner's next turn. Charge Cards correspond to the six classes. The current
proposal requires at least one matching Charge for a specialized BitCard's attack;
remaining costs may use any Charge. Null attacks display `◇` and may be paid with
any combination of non-Null Charge. The experimental `()` Null Charge Card does not
currently pay attack costs. Players may play one Command per turn. Attacking ends
the turn. The first player to knock out three opposing BitCards wins; inability to
draw also loses.

The prototype advantage loop is Robot → Virus → Daemon → Glitch → Bug → Robot.
Advantage adds 10 damage and Null is neutral. These are draft game rules, not
consensus constants. Keep blockchain ownership independent from match state.

Each of the six Charge types has exactly one fixed full-art design: its two-character
class symbol rendered at large scale with block glyphs. Charge artwork and names must
not vary by seed. Charge cards have no effect panel and use the standard
common-rarity footer with catalog number, Set ID, and copy serial. Treat the
established symbol font as immutable; an intentional replacement requires an
explicit new support-card design version rather than silently changing it.

Charge copies may use three display-only finishes without changing their gameplay
identity: Standard, a dark alternating-band Pulse Rare treatment, and a bright
inverted Overcharged Super Rare treatment. They share the same catalog slot and
deck limit. Their symbols retain the fixed artwork and their outer borders stay white.

The Set 1 gameplay draft contains exactly ten fixed, hand-authored Command Card
types: two each for repair, draw/search, switching, defense, and Charge control.
Their names, rules text, and terminal artwork do not vary by seed. Command effects
must remain conservative under the one-Command-per-turn rule and should not create
repeatable denial or permanent free resources.

Command copies may use three display-only finishes without changing their gameplay
identity: Standard, a grayscale CRT Rare treatment, and a high-contrast monochrome
double-frame root-terminal Super Rare treatment. Finish variants share the same
catalog slot, rules, and deck limit. Their outer card borders remain white.

The launch marketing draft allows up to ten fixed, hand-authored Promo Card Types,
with a proposed hard supply of 1,000 copies each. They are playable but must not be
stronger than ordinarily obtainable cards. Promo 01 is the neutral `NETDOGE` draft.
Promo definitions never vary by seed; none are activated protocol objects yet.
Promo cards use the exclusive `✦` footer mark and never display rarity stars.

## Code boundaries

- `src/model.rs`: strong domain types and configurable rules.
- `src/hash_stream.rs` and `src/sha256.rs`: deterministic randomness and hashing.
- `src/generator.rs` and `src/creature.rs`: consensus-relevant generation.
- `src/canonical.rs`: canonical Card Type encoding and hashing.
- `src/renderer.rs`: terminal presentation only; keep it outside canonical hashes.
- `src/generator_assets/v1/`: locked version 1 names, palettes, and visual assets.
- `src/generator_assets/v2/`: version 2 action tables.
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

ANSI color and finish previews are presentation features and never affect canonical
generation. The user-facing CLI renders cards in color; plain rendering may remain
available internally for exact-width tests. Do not describe experimental Holo,
Reverse Holo, Gold, Rainbow Holo, or rarity previews as finalized protocol objects.

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
