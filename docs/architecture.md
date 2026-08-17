# Phase 1 architecture

The `bitcards` library contains consensus-oriented models, generation, canonical
encoding, and hashing. The binary is only a CLI adapter. Rendering is a separate
projection which consumes a generated model and cannot alter it.
Generator data is stored under `src/generator_assets/v1`: Rust constants hold
small palettes and name tables, while larger ASCII templates are plain text
embedded with `include_str!`. Generator assets are never loaded as mutable runtime
configuration.

## Version 1 seed derivation

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

The binary encoding begins with ASCII `BITCARDS:CARDTYPE:V1` and encodes fields in a
fixed documented order. Unsigned integers are big-endian. Enum values are one byte.
Strings are UTF-8 prefixed by a four-byte length. Version 1 generation emits only
protocol-fixed ASCII strings and ASCII artwork, avoiding Unicode normalization
ambiguity; the Unicode card border and rarity stars are display-only. Lists use a
four-byte count followed by ordered elements. The order is:

1. generator version, Set seed, Card Type seed
2. name, class, rarity, maximum supply
3. HP and deploy cost
4. ordered attacks (name, damage, cost, effect)
5. ordered artwork rows

The hash is SHA-256 over exactly this encoding. Display-only serial numbers belong
to future Card Copies and are not part of a Card Type. Discovery block height is
also copy-specific and therefore excluded from Card Type generation and hashing.

The distinction between Blocks, Sets, Card Types, and Card Copies—and the current
finite-issuance proposal—is documented in
[`sets-and-issuance.md`](sets-and-issuance.md).

## Prototype class and Charge rules

The sole public classification is Robot, Glitch, Daemon, Virus, Bug, or Null;
internal drawing algorithms are not a second card type. Robot beats Virus, Virus
beats Daemon, Daemon beats Glitch, Glitch beats Bug, and Bug beats Robot. An
advantaged attack adds 10 damage. Null is neutral in both directions.

Attacks spend universal Charge. The prototype grants 1 Charge per turn and caps
stored Charge at 6. These integer constants are explicit and testable; no
floating-point modifiers are used.

## Artwork diversity

Each public class maps directly to its own drawing algorithm. Robot uses embedded
structural templates with procedural extensions; Glitch uses asymmetric corrupted
bands; Daemon uses spectral bodies and randomized fringes; Virus uses irregular
radial growth; Bug uses segmented insect geometry; and Null uses symmetric sparse
bit patterns. A deterministic audit currently checks 512 seeds per class (3,072
artworks total), requires zero exact collisions, and requires broad silhouette
diversity. Future Set construction must additionally reject close matches across
the complete proposed Set before its definitions are locked.
