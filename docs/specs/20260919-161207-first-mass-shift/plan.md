# Implementation Plan: First Internal Mass Shift

**Branch**: feature/first-shift | **Date**: 2026-09-19 | **Spec**: [spec.md](spec.md)

## Summary

Add the smallest honest first-shift experiment to the existing rolling ball.
Keyboard intent moves a conceptual 0.45 kilogram inner mass within a
world-relative horizontal disk. Its derived centre of mass is converted into the
rotating body's local frame and updated before fixed physics. Gravity and
ordinary contact then determine the outer-ball response. A world-stable
development display shows the current mass, target, centre, and limit. No
velocity, force, torque, orientation, trajectory, or traction operation steers
the ball.

## Technical Context

**Language/Version**: Rust edition 2024; Rust 1.95.0 via rust-toolchain.toml

**Primary Dependencies**: Existing Bevy 0.19.1 and avian3d 0.7.0; no new dependency

**Storage**: N/A

**Testing**: Unit tests for deterministic input, disk clamp, target movement,
mass-ratio, and frame conversion; manual desktop trials; cargo check, cargo
test, cargo fmt --all -- --check, and cargo clippy --all-targets --all-features
-- -D warnings

**Target Platform**: Existing supported native desktops

**Project Type**: Single Rust desktop application crate

**Performance Goals**: One fixed-step mass update, one ball, and a readable
display remain smooth in a normal development run; no scaling target is claimed.

**Constraints**: Input changes only desired internal state. Mass-property update
runs before Avian preparation. Do not write velocity or angular velocity, apply
force/torque, set rotation/path, or add traction/vehicle steering. No generic
controller, physics wrapper, second ball, or production course.

**Scale/Scope**: One conceptual mass, WASD keyboard input, world-relative X/Z
frame, one development display, and existing slope/run-out.

## Constitution Check

### Pre-research gates

| Gate | Result | Planning response |
|---|---|---|
| Physics is the control scheme | Pass | Input changes an inner-mass target; derived COM is the sole coupling. |
| Fun and learnability outrank realism | Pass | This documented approximation exposes grounded response, not scientific truth. |
| Momentum creates gameplay | Pass | Gravity, contact, and shifted balance determine altered lines. |
| Simple systems create emergence | Pass | One mass state and ordinary rigid-body contact replace bespoke rules. |
| Practical determinism and testing | Pass | Pure calculations get unit tests; solver outcomes get manual trials. |
| Presentation without artificial abstraction | Pass | The display reads state and never owns physical behaviour. |
| Playable progress and scope discipline | Pass | Work ends after one controllable experiment. |
| Dependencies earn their place | Pass | Existing Avian capability is used directly; no new crate. |

### Post-design re-check

Pass. Runtime CentreOfMass is the smallest direct model that lets input alter a
meaningful physical state. Its omitted actuator reaction and airborne behaviour
are documented rather than hidden by conventional steering.

## Project Structure

### Documentation

~~~text
docs/
├── mass-shift.md
├── physics.md
├── roadmap.md
└── specs/20260919-161207-first-mass-shift/
    ├── spec.md
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── quickstart.md
    ├── contracts/desktop-mass-shift.md
    └── tasks.md
~~~

### Source Code

~~~text
src/
├── main.rs          # Existing app and schedule wiring
├── physics.rs       # Existing shared conventions and markers
├── scene.rs         # Existing ball plus development display entities
├── mass_shift.rs    # State, pure math, input, fixed COM update, tests
└── camera.rs        # Reused unless a small visibility change is necessary
~~~

**Structure Decision**: Preserve the one-crate application. A single new
mass_shift module owns experimental state, pure logic, and fixed coupling. Scene
owns primitive display entities only. No generic player controller or framework
is introduced.

## Implementation Outline

1. Use shell mass 0.55 kilograms, inner mass 0.45 kilograms, total 1.0
   kilogram, 0.30 metre horizontal inner-mass disk, maximum combined COM offset
   0.135 metres, and 1.5 metres-per-second movement/return speed.
2. Map W/A/S/D to world positive-Z, negative-X, negative-Z, and positive-X.
   Normalise combined input, clamp it to the disk, and return target to centre
   after release.
3. Before PhysicsSystems::Prepare in the fixed schedule, move current position
   toward target; multiply it by inner/total mass; inverse-rotate into body
   local space; and update CenterOfMass directly.
4. Add unparented centre, current-mass, target-mass, line, and disk-limit
   visuals. Keep the display world-stable even as the shell rolls.
5. Preserve normal gravity/contact behaviour and add no direct movement,
   rotation, force, torque, impulse, or traction system.
6. Document controls, frame, values, compromises, and trial results; update only
   roadmap items demonstrated by validation.

## Complexity Tracking

No constitution violation needs justification.
