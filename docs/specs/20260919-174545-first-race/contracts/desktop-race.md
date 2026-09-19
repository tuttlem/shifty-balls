# Desktop Race Contract

This contract defines the observable behaviour of the first local race vertical slice. It is a gameplay/UI contract, not a network or public API.

## Race start

1. Launching the application presents four distinct balls at the prototype-course start.
2. The race display presents a 3–2–1–GO countdown.
3. Before GO, race time does not advance and mass shifting cannot create an early launch.
4. At GO, all racers are released together and the timer starts.

## Player controls

| Input | Contract |
|---|---|
| W/A/S/D | While racing, request the human racer’s existing internal-mass displacement in the documented stable world frame |
| R | At any race phase, reset every racer and race record, then begin a fresh countdown |
| F3 | Toggle development-only human mass/velocity information without affecting race state or simulation |

## Race display

During countdown and racing, the player can read:

- the countdown or GO state;
- player position as place / 4;
- elapsed race time once GO has occurred.

After the player finishes validly, the display shows:

- fixed finishing place;
- elapsed race time;
- win or loss;
- rematch instruction.

## Validity and ordering

- A racer must enter each current-course progression region in order before the finish can count.
- The finish result cannot be revoked by later contact or motion.
- Current positions use finish status, ordered progress, in-leg advance, and stable racer identity in that order.
- A physical collision may change racer order, but never bypasses the progression rule.

## Explicit exclusions

There are no selectable modes, laps, alternate routes, local/network multiplayer, persistent records, collision attacks, AI difficulty settings, production HUD, or general checkpoint system.
