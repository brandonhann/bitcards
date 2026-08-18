# BitCards game rules — early draft

These rules exist so generated attacks and abilities share a coherent vocabulary.
They are design proposals, not consensus rules, and the playable game remains
outside the current repository scope.

## Cards and decks

- A deck contains 40 cards.
- A deck may contain no more than three copies of one Card Type.
- **BitCards** battle and have HP, class, actions, and a Charge requirement.
- **Charge Cards** build the player's shared reusable Charge pool. Charge comes in
  Robot, Glitch, Daemon, Virus, Bug, and neutral Null types.
- **Command Cards** are one-use support effects such as healing, drawing,
  switching, or protection.

The Set 1 draft has ten fixed Command Card types rather than procedural Commands:
Quick Patch, Purge, Download, Index Search, Hot Swap, Force Route, Firewall, Safe
Mode, Reboot Charge, and Burst Charge. There are two conservative effects in each
of five roles: repair, draw/search, switching, defense, and Charge control.

## Setup and field

- Each player draws five cards.
- Each player begins with one Active BitCard when possible.
- Each player may have up to three Benched BitCards.
- Determine the first player using a mutually agreed random method.

## Turn

1. Refresh all installed Charge.
2. Draw one card.
3. Deploy BitCards to open Active or Bench positions.
4. Install at most one Charge Card, to a maximum of six installed Charge.
5. Play at most one Command Card.
6. Switch or retreat when an effect permits it.
7. Attack with the Active BitCard. Attacking ends the turn.

Installed Charge belongs to the player rather than an individual BitCard. An
attack exhausts its listed Charge cost; exhausted Charge refreshes at the beginning
of that player's next turn. Charge remains installed after a knockout.

To attack, a specialized BitCard must have at least one installed Charge matching
its class; the rest of the attack's cost may use any installed Charge. Null
BitCards show `◇` for their attack cost and may pay it with any combination of
non-Null Charge. The experimental `()` Null Charge Card remains in the visual
prototype but does not currently pay attack costs. This typed-payment rule is an
early proposal and needs playtesting.

Each Charge type uses one permanent oversized rendering of its two-character class
symbol rather than procedural art variants. Charge cards are common and receive normal
Set-local catalog numbers and copy serials.

## Damage and victory

- Robot → Virus → Daemon → Glitch → Bug → Robot.
- Class advantage adds 10 damage. Null has no advantage or weakness.
- Damage remains on a BitCard until healed or it is knocked out.
- A player wins after knocking out three opposing BitCards.
- A player also wins if the opponent cannot draw at the start of their turn.

The deck size, limits, turn sequence, and victory threshold must be playtested
before they are treated as final game rules.
