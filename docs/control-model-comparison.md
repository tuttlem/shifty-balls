# Control Model Comparison

This is a post-First-Race design iteration, not a rewrite of project history.
The accepted First Race remains a four-ball SHIFT regression path.

**First Roll → First Shift → First Course → First Race → Control Model Comparison**

## Purpose and fairness constants

Every comparison attempt uses the same human ball, gravity, course, route gates,
finish region, camera purpose, start transform, reset behaviour, and contact
properties. Opponents are physically absent, not merely invisible, so traffic
cannot alter a time-trial-style result.

| Model | Physical request | Neutral behaviour | Initial tuning |
|---|---|---|---|
| SHIFT | Existing world-relative internal-mass/centre-of-mass offset. | Mass returns continuously to centre. | 45% internal mass, 0.30 m displacement, 1.5 m/s movement. |
| TORQUE | World-relative roll torque: `up × input`. | No player torque. | 3.0 N m. |
| FORCE | World-horizontal force at the centre of mass. | No player force. | 3.0 N. |

No model writes a desired velocity, transform, orientation, or trajectory.
Gravity, friction, collision, rotation, and momentum remain active. TORQUE stays
active in air as spin; FORCE stays active in air as horizontal acceleration.
Those differences are documented observations, not hidden rules.

## Controls and display

| Input | Behaviour |
|---|---|
| `1` / `2` / `3` | Begin a normalised SHIFT / TORQUE / FORCE comparison attempt. |
| `4` | Enter/reset First Race; human and AI retain established SHIFT control. |
| `W` / `A` / `S` / `D` | Down-track `+Z`, left `-X`, up-track `-Z`, right `+X`; diagonals normalise. |
| `R` | Reset the active comparison or race session. |
| `F3` | Toggle development display. |

The comparison readout identifies active model, current time, and that model's
session best. F3 shows stable world intent plus the SHIFT mass disk, TORQUE
axis, or FORCE vector. It is development visualisation, not production HUD.

## Playtest procedure

Run three attempts in each model. Rotate each round's order `SHIFT → TORQUE →
FORCE`, then `TORQUE → FORCE → SHIFT`, then `FORCE → SHIFT → TORQUE` to avoid
crediting course learning to the final model. For each attempt examine a gentle
turn, direction change while rolling, bank/recovery, input release, and any
airborne transition. Record completion/time where meaningful, but do not select
a model by time alone: the course and final turn are already demanding.

Then select `4` and complete/rematch a normal First Race to check that the
comparison has not weakened the accepted vertical slice.

## Observation record

| Model | Response and predictability | Banks/recovery/airborne behaviour | Time/enjoyment | Result |
|---|---|---|---|---|
| SHIFT | Signed off in the complete owner procedure. | Signed off across the required bank, recovery, and airborne checks. | No comparative timing or preference was retained. | Validated; no preference selected. |
| TORQUE | Signed off in the complete owner procedure. | Signed off across the required bank, recovery, and airborne checks. | No comparative timing or preference was retained. | Validated; no preference selected. |
| FORCE | Signed off in the complete owner procedure. | Signed off across the required bank, recovery, and airborne checks. | No comparative timing or preference was retained. | Validated; no preference selected. |

The owner completed the rotated-order, reset, neutral-input, model-label, and
First Race regression/rematch procedure. The race control regression discovered
during this pass was corrected before sign-off; the human ball now consumes the
same keyboard intent in both comparison and race sessions. Automated validation
also passes.

## Provisional decision

**Inconclusive by choice.** The completed validation establishes that all three
models and the restored First Race are usable, but does not select a preferred
human control model. Do not convert AI, remove SHIFT, add hidden steering, or
expand racing content until a follow-up makes that design decision.
