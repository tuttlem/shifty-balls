# First Internal-Mass Experiment

This is the first small experiment in Shifty Balls' defining control idea:

**player moves mass -> balance changes -> ball responds**

It is deliberately not final steering, final tuning, or a claim that the game
is fun. Its job is to make the physical relationship visible and easy to
change.

## Selected approximation

The existing one-kilogram Avian rigid body remains the only ball body and
collider. A project-owned conceptual inner mass changes that body's Avian
`CenterOfMass` component immediately before Avian prepares the fixed physics
step. The resulting gravity and contact constraints determine the ball's
movement.

This is the smallest useful initial model: it is stable, uses the selected
physics library directly, and exposes a meaningful physical state without a
second rigid body, collider, joint, motor, external force, torque, impulse, or
vehicle-style steering system.

It is an approximation. Moving the conceptual mass does not model actuator
reaction or a changing angular-inertia tensor, and it should not be treated as
airborne propulsion. The current question is only whether a grounded offset
produces understandable influence. If the response is weak or unintuitive,
adjust this experimental model or replace it; do not hide the problem with
conventional steering.

## Temporary controls and reference frame

The player-facing frame is stable world-horizontal X/Z, rather than the ball's
rotating local frame:

| Key | Requested internal-mass direction |
|---|---|
| W | Down-track, positive Z |
| S | Up-track, negative Z |
| A | Track-left, negative X |
| D | Track-right, positive X |
| W/A/S/D together | A normalised diagonal within the same horizontal disk |
| Release all directions | Target returns toward neutral centre |

The current mass moves continuously toward the target; it does not teleport.
Both target and current position are clamped to the allowed horizontal disk.

## Initial tuning values

| Parameter | Value |
|---|---:|
| Outer shell mass | 0.55 kg |
| Conceptual inner mass | 0.45 kg |
| Dynamic body total | 1.00 kg |
| Inner-mass radius | 0.30 m |
| Maximum combined COM offset | 0.135 m |
| Target/return speed | 1.5 m/s |
| Ball radius | 0.50 m |

The values live in `src/physics.rs` and the focused `MassShiftTuning` resource
in `src/mass_shift.rs`, not in a general configuration framework.

## Development display

World-stable Bevy gizmos follow the ball centre without inheriting shell
rotation:

- white: outer-ball centre;
- red: current inner-mass position and line from centre;
- cyan: requested target and line from centre;
- blue circle: allowed horizontal displacement boundary.

This display is development-only and owns no physics or game state.

## Trial record and open questions

The implementation has deterministic tests for input mapping, diagonal
normalisation, disk clamping, smooth return, mass ratio, and world-to-local
conversion. A native Linux/Vulkan launch created the Shifty Balls window during
implementation. Desktop trial procedure is recorded in the feature
[quickstart](specs/20260919-161207-first-mass-shift/quickstart.md): compare
no-input, paired A/D, and W/S downhill runs from the same start.

Desktop validation is signed off: cardinal and diagonal requests remained
bounded and returned continuously to neutral; the world-stable display remained
readable; paired A/D and W/S downhill comparisons produced the intended
observable physical influence. This proves the first shift experiment, not that
the final racing mechanic is fun. The next course and sustained playtesting work
must answer that larger question.
