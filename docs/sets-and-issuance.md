# Sets, blocks, and card issuance

This document records the current design direction. Its numbers are deliberately
not consensus constants yet; they must be tested before a future protocol version
locks them.

## Vocabulary

- A **Block** is one ordered step in the blockchain. Blocks continue for as long
  as the network operates.
- A **Set** is a finite collection period spanning many blocks.
- A **Card Type** is one reproducible design in a Set.
- A **Card Copy** is an owned instance of a Card Type. Its serial number and
  discovery block never change.

For example, Set 1 can cover blocks 0 through 131,399. Block 101 is simply another
block within Set 1; it does not start a new Set.

## Ten-year issuance draft

The current planning target is:

- one target block every 2 minutes;
- 131,400 blocks per Set, approximately 6 months;
- 20 Sets over approximately 10 years;
- 100 Card Types per Set;
- final card-issuance height 2,628,000;
- at most one network-wide discovery opportunity per block.

This produces 2,000 Card Types and no more than 2,628,000 discovery opportunities.
Network popularity must not shorten the schedule. After card issuance ends, blocks
must continue so existing cards can still be validated and transferred. How the
network remains secure after issuance is intentionally unresolved and belongs to a
later consensus design phase.

## Working Set names

Set names are human-readable labels attached to numbered Sets; Set IDs remain the
stable protocol identity. These are working names, not locked generator data:

| Set | Working name | Set | Working name |
|---:|---|---:|---|
| 1 | Genesis | 11 | Root Access |
| 2 | Boot Sequence | 12 | Forked Reality |
| 3 | Kernel Dawn | 13 | Cache Eclipse |
| 4 | Neon Circuit | 14 | Zero Day |
| 5 | Packet Storm | 15 | Black Terminal |
| 6 | Glitch Protocol | 16 | Deadlock |
| 7 | Daemon Rising | 17 | Entropy Engine |
| 8 | Viral Load | 18 | Final Compile |
| 9 | Null Sector | 19 | End of Line |
| 10 | Machine Age | 20 | Last Signal |

Before any Set activates, its generator version, seed derivation, type count,
rarity rules, supply limits, and name must be committed. Once activated, those
definitions cannot change.

## Card-face metadata

The fixed card face is 34 columns by 24 rows, including its border, with a 7-row
artwork region. In a typical monospaced font this approximates the proportions of
a 2.5-by-3.5-inch physical trading card. Its footer has no horizontal divider:
the three-digit Set-local Card Type catalog number such as `025` appears immediately
before the rarity stars on the left, the Set ID stays centered, and the type-local
copy serial appears on the right. Catalog position and copy
serial are different identities: every copy of Type 25 shares `025`, while
each copy has its own permanent `#000081`-style serial. One empty row separates
the attacks from the footer. The Phase 1 CLI uses `001`, `SET 01`, and
`#000001` as its default complete-card preview identity; these display values are
not a real on-chain mint. Catalog positions must eventually be assigned by the
deterministic Set construction process, never inferred by hashing one isolated
card modulo the Set size. Discovery block
remains verifiable chain metadata but is not displayed on the card face.
