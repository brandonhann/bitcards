# Phase 1 architecture

The `bitcards` library contains consensus-oriented models, generation, canonical
encoding, and hashing. The binary is only a CLI adapter. Rendering is a separate
projection which consumes a generated model and cannot alter it.
Generator data is stored under versioned `src/generator_assets/` modules. Version
1 holds palettes, names, and ASCII templates; version 2 adds action tables while
reusing the locked version 1 visual assets. Larger templates are plain text embedded
with `include_str!`. Generator assets are never loaded as mutable runtime
configuration.

## Seed derivation

The generator receives three explicit inputs: a positive 32-bit generator version,
an arbitrary non-empty Set seed, and an arbitrary non-empty Card Type seed. Version
1 initializes its stream key as:

```text
SHA256("BitCards/CardGenerator\0" || uint32-be(version)
       || uint32-be(setSeed.length) || setSeed
       || uint32-be(typeSeed.length) || typeSeed)
```

Stream block `n` is `SHA256("BitCards/HashStream/v1\0" || key || uint64-be(n))`,
starting at zero. Integers are read big-endian. A value in `[0, bound)` is selected
by rejecting 32-bit words at or above `floor(2^32 / bound) * bound`.

Every incompatible generator or canonical-format change requires a new generator
version. Existing versions must remain available forever. Future Set commitments
will lock a generator version and rules before their unpredictable activation seed
exists; that lifecycle is intentionally outside Phase 1.

## Canonical Card Type encoding

Each binary encoding begins with a versioned domain: `BITCARDS:CARDTYPE:V1` or
`BITCARDS:CARDTYPE:V2`. Unsigned integers are big-endian. Enum values are one byte.
Strings are UTF-8 prefixed by a four-byte length. Version 1 generation emits only
protocol-fixed ASCII strings and ASCII artwork, avoiding Unicode normalization
ambiguity; the Unicode card border and rarity stars are display-only. Lists use a
four-byte count followed by ordered elements. The order is:

1. generator version, Set seed, Card Type seed
2. name, class, rarity, maximum supply
3. HP and deploy cost
4. ordered actions
5. ordered artwork rows

Version 1 actions are always attacks encoded as name, damage, cost, and effect.
Version 2 prefixes every action with a one-byte kind. Attack kind `1` is followed
by name, damage, cost, and effect. Ability kind `2` is followed by name and effect.
This explicit distinction avoids magic damage values and keeps abilities strongly
typed.

The hash is SHA-256 over exactly this encoding. Display-only serial numbers belong
to future Card Copies and are not part of a Card Type. Discovery block height is
also copy-specific and therefore excluded from Card Type generation and hashing.

Card Types use exactly three protocol rarity values: Common, Rare, and Super Rare.
Their default integer selection weights total 10,000, and each higher tier has a
lower hard maximum supply. Holo, Reverse Holo, Gold, and Rainbow Holo are visual
copy finishes under evaluation; they are not additional rarity values.

The distinction between Blocks, Sets, Card Types, and Card Copies—and the current
finite-issuance proposal—is documented in
[`sets-and-issuance.md`](sets-and-issuance.md).

## Prototype class and Charge rules

The sole public classification is Robot, Glitch, Daemon, Virus, Bug, or Null;
internal drawing algorithms are not a second card type. Robot beats Virus, Virus
beats Daemon, Daemon beats Glitch, Glitch beats Bug, and Bug beats Robot. An
advantaged attack adds 10 damage. Null is neutral in both directions.

Attacks spend universal Charge. The prototype grants 1 Charge per turn and caps
stored Charge at 6. Abilities are passive and do not have damage or a Charge cost.
Version 2 deterministically gives a Card Type one strong attack (20%), one ability
plus one attack (35%), or two attacks (45%). The renderer reserves the same seven
action rows for every layout. These integer constants are explicit and testable;
no floating-point modifiers are used.

## Artwork diversity

Each public class maps directly to its own drawing algorithm. Robot uses embedded
structural templates with procedural extensions; Glitch uses asymmetric corrupted
bands; Daemon uses spectral bodies and randomized fringes; Virus uses irregular
radial growth; Bug uses segmented insect geometry; and Null uses symmetric sparse
bit patterns. A deterministic audit currently checks 512 seeds per class (3,072
artworks total), requires zero exact collisions, and requires broad silhouette
diversity. Future Set construction must additionally reject close matches across
the complete proposed Set before its definitions are locked.
