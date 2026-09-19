# First Race Vertical Slice

The first race turns the existing single-ball course into one compact,
point-to-point physical contest: one human racer and three AI racers share the
same baseline ball physics, track, and internal-mass control model.

This is deliberately not a general racing system. There are no laps, alternate
routes, selectable tracks, multiplayer, ball classes, advanced AI, persistent
records, or production presentation.

## Racer and controller

A **racer** is the simulated physical ball. It owns its transform, motion,
rotation, internal-mass state, course progress, and finish record.

A **controller** supplies a requested internal-mass position:

- The human controller maps W/A/S/D to the documented stable world-horizontal
  frame.
- The AI reads the next broad route target and requests a world-horizontal
  internal-mass position through the same clamp, smoothing, centre-of-mass, and
  wake path.

Neither controller sets velocity, turns a ball, applies a steering force, or
teleports a ball. Gravity, contact, and ordinary collisions decide the result.
During the post-First-Race control comparison, this race path deliberately
remains pinned to SHIFT for both controllers; TORQUE/FORCE AI is deferred until
human playtest evidence chooses a preferred model.

## Start and race lifecycle

Four visually distinct balls begin in a spaced two-by-two arrangement on the
level start deck. Racers are physically held during the visible 3–2–1
countdown, with every internal mass neutral. At GO, every racer is released
together into ordinary gravity-driven simulation and the race timer begins.

| Phase | Behaviour |
|---|---|
| Countdown | No active timer, human control, or AI control; no early launch |
| Racing | Timer, internal-mass controllers, progress, position, and finishes are active |
| Player finished | Player place and time are fixed; R starts a new countdown |

## Progress, position, and finish

The hand-authored course has four broad ordered regions aligned to its local
travel direction. A racer can advance only through its next required region.
Short backwards movement does not revoke progress, and entering a later region
early does not count.

The finish is valid only after all four regions. Finished racers are ordered by
their immutable finish place. Other racers are ordered by completed regions,
then forward progress toward the next region, then stable racer identity. This
is sufficient for this single connected route, not a universal track graph.

## Controls and display

| Input | Behaviour |
|---|---|
| W/A/S/D | Shift the human racer’s internal mass while racing |
| R | Reset every racer and race record, then begin a fresh countdown |
| F3 | Toggle human-only mass and velocity development visibility |

`4` now explicitly enters/resets this accepted race path from the human-only
control comparison. `1`/`2`/`3` are comparison controls, not race controls.

The race display shows countdown/GO, player position, and race time. Once the
player finishes validly, it shows fixed place, time, and a rematch hint.

## Known limitations and first-race record

- AI is intentionally course-aware but simple. It targets broad route regions;
  it has no avoidance, overtaking logic, strategic lines, difficulty modes, or
  teleport recovery.
- Collision gameplay is not tuned. This slice exists to find out whether
  ordinary contact is fun before designing any collision-specific rules.
- The course was already known to be demanding. It may prove too narrow or
  unforgiving for traffic; that is useful evidence, not a reason to hide the
  problem with additional systems.
- R is the recovery path for a racer that falls, stalls, or becomes trapped.

Record the following after the manual procedure in the First Race
[quickstart](specs/20260919-174545-first-race/quickstart.md):

| Question | Observation | Result |
|---|---|---|
| Are collisions fun or merely annoying? | Competition is established; detailed collision feel needs the next tuning pass. | Promising; needs tuning |
| Can another ball meaningfully alter the player’s line? | The multi-ball field creates meaningful traffic. | Yes, initial evidence |
| Can the player recover after contact? | Not separately characterised in this first sign-off. | Unresolved |
| Does traffic make bank positioning more interesting? | The final turn becomes substantially more demanding with competition. | Yes, but too punishing |
| Does overtaking happen naturally? | Competition was observed; overtaking was not separately characterised. | Unresolved |
| Can balls run side-by-side? | Not separately characterised in this first sign-off. | Unresolved |
| Do bottlenecks become interesting? | The final turn acts as a strong bottleneck. | Needs course tuning |
| Is the course wide enough for racing? | The final turn is currently far too difficult with traffic. | No; improve it next |
| Does mass control remain understandable under pressure? | Competition remains playable enough to continue the project. | Promising |
| Does the player care about race position? | The owner reported that the field creates genuine competition. | Yes, initial evidence |
| Does winning feel different from merely completing? | Not separately characterised in this first sign-off. | Unresolved |

The project owner signed off the First Race vertical slice on 19 September
2026. The next likely work is a bounded course-and-traffic tuning pass,
beginning with the final turn; it should not add more racing systems merely to
hide the bottleneck.
