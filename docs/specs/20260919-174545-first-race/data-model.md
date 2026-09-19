# Data Model: First Race

This model is deliberately limited to the existing point-to-point prototype course. It distinguishes a simulated **racer** from the **controller** that supplies the racer’s requested internal-mass position.

## Racer

| Field | Description | Validation |
|---|---|---|
| stable ID | Deterministic participant identity and final ordering tie-break | Unique within the four-racer field |
| controller kind | Human or AI | Exactly one controller kind per racer |
| visual identity | Prototype material/colour and human marker where useful | Human is distinct; AI colours differ from each other |
| start state | Assigned position and orientation on the staged start | Starts do not overlap and are physically fair |
| physical state | Existing independent transform, linear velocity, angular velocity, collider, and centre of mass | Equivalent baseline characteristics for all racers |
| internal-mass state | Requested and current stable world-horizontal offset | Clamped to the existing permitted displacement disk |
| progression | Next required gate and completed gate count | Increments only for the exact next gate; never decrements |
| finish record | Optional immutable finish place and time | Set once, only after valid progress |

Relationships: each Racer has exactly one Controller and one race Progress record. All racers participate in one Race.

## Controller

| Kind | Responsibilities | Prohibited behaviour |
|---|---|---|
| Human | Converts W/A/S/D input into the human racer’s requested world-horizontal mass offset while racing | Direct movement, rotation, force, torque, propulsion, or hidden traction |
| AI | Selects the next route target and requests a world-horizontal mass offset through the shared mass model while racing | Direct movement, heading control, teleport recovery, artificial steering, rubber-banding |

Both controllers are inactive before GO and after the player result. Both use the same shared clamp, smoothing, centre-of-mass conversion, and wake-on-non-neutral-offset behaviour.

## Race lifecycle

| Phase | Entry condition | Behaviour | Exit |
|---|---|---|---|
| Preparing | Initial launch or rematch reset | All racers restored to assigned held starts; no timer or controller activity | Starts countdown immediately |
| Countdown | Fresh staged field | Visible 3–2–1 countdown; all mass state neutral; timer and AI inactive | GO releases every racer together |
| Racing | Countdown expires | Timer, human control, AI control, progression, positions, and valid finishes active | Player records a valid finish |
| Player finished | Human’s valid finish is recorded | Player result is visible and fixed; remaining racer ordering may settle if inexpensive | Rematch |

The countdown-to-racing transition happens exactly once. Rematch clears all mutable race state and starts a new countdown.

## Progress gate

| Field | Description | Validation |
|---|---|---|
| sequence | Required position in the current route | Strictly increasing, beginning at zero |
| centre | World location of the broad gate region | Matches the visible connected course |
| horizontal forward | Direction of travel for the route leg | Non-zero and normalised in the world-horizontal plane |
| lateral/vertical/depth extents | Local tolerance around the course | Broad enough for valid bank lines, narrow enough to reject obvious shortcuts |
| route distance | Logical route advance used to compare racers in the same leg | Monotonically increases through the route |

Progress tests use the racer’s location in gate-local coordinates. Entering a later gate early is ignored; revisiting an earlier gate has no effect.

## Position record

1. Finished racers sort before unfinished racers by immutable finish place.
2. Unfinished racers sort by completed gates descending.
3. Racers tied on gates sort by clamped advance toward the next route target descending.
4. Remaining ties sort by stable ID ascending.

The displayed player place is the one-based index in this order. This is intentionally a current-course rule, not an alternate-route solution.

## Race result

| Field | Description | Validation |
|---|---|---|
| finishing place | One-based immutable order in which an eligible racer finished | Unique; never changes after assignment |
| elapsed time | Race timer at valid finish | Non-negative; timing only advances in Racing |
| player won | Whether the player’s place is first | Derived, not independently mutable |
