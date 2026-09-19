# Implementation Plan: First Physically Simulated Rolling Ball

**Branch**: feature/first-rolling-ball | **Date**: 2026-09-19 | **Spec**: [spec.md](spec.md)

## Summary

Add Avian 3D 0.7.0 as the deliberate, Bevy-native rigid-body implementation
for the first physics experiment. Replace the decorative sphere and plane with
one dynamic, visibly oriented ball and a small static cuboid test track: a
start area, a slope descending towards positive Z, a level run-out, retaining
edges, and an end-stop. Gravity alone moves the ball. A small upright follow
camera keeps the unattended traversal observable. The implementation documents
the physics decision and world conventions, with no input, internal mass,
player propulsion, or race systems.

## Technical Context

**Language/Version**: Rust edition 2024; MSRV Rust 1.95.0 via rust-toolchain.toml

**Primary Dependencies**: Bevy 0.19.1; add avian3d 0.7.0, both exact-version
pins. Avian default features provide f32 3D collision and its optional debug
plugin capability; no other ecosystem crate is needed.

**Storage**: N/A

**Testing**: cargo test for ordinary Rust logic; manual desktop observation for
rendered/physics behaviour; cargo check, cargo fmt --all -- --check, and cargo
clippy --all-targets --all-features -- -D warnings

**Target Platform**: Native desktop systems supported by Bevy and Rust 1.95.0

**Project Type**: Single Rust desktop application crate

**Performance Goals**: One-ball primitive scene is smooth and readable in a
normal development run; no frame-rate guarantee is required for this prototype.

**Constraints**: One dynamic ball only; world units are metres; no physics
wrapper, engine abstraction, gameplay input, internal-mass mechanics, custom
physics engine, production track, or additional physics dependencies.

**Scale/Scope**: One startup scene, one physics library, one ball, and several
static primitive colliders. It ends after slope-to-run-out observation.

## Constitution Check

### Pre-research gates

| Gate | Result | Planning response |
|---|---|---|
| Fun and learnable physical control precede realism | Pass | This is an unassisted observation baseline; prototype values are documented rather than presented as final realism. |
| Physics remains an understandable causal system | Pass | Gravity, material properties, explicit ball mass, and static geometry are the only sources of movement. |
| Bevy is the selected technology; dependencies earn their place | Pass | The plan makes one documented physics choice for concrete rigid-body requirements. |
| Determinism and tests are practical, not theoretical | Pass | Test only project-owned pure helpers if introduced; validate third-party simulation manually. |
| Presentation must not own all game concepts, without artificial abstraction | Pass | Direct Bevy and Avian components model the ball and track; only useful constants/spawners are separated. |
| Playable progress and scope discipline | Pass | No controls, multiplayer, race systems, track framework, or premature camera work. |
| Quality is completion | Pass | Build, tests, formatting, lint, documentation, and accurate roadmap changes are implementation tasks. |

### Post-design re-check

Pass. The selected design keeps the physical cause visible, preserves a direct
Bevy implementation, and creates no speculative framework. Avian's future mass
property support is only a documented option to investigate later; it does not
commit this feature to a centre-of-mass architecture.

## Project Structure

### Documentation

~~~text
docs/
├── physics.md                                      # Physics choice and world conventions
├── roadmap.md                                      # Long-lived backlog and burndown
└── specs/20260919-150334-first-rolling-ball/
    ├── spec.md
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── quickstart.md
    ├── contracts/
    │   └── desktop-observation.md
    └── tasks.md                                    # Created by speckit-tasks
~~~

### Source Code

~~~text
Cargo.toml                                          # Add avian3d only
src/
├── main.rs                                         # App assembly and direct startup wiring
├── physics.rs                                      # Prototype constants and physics plugin setup
├── scene.rs                                        # Ball and primitive static-track spawning
└── camera.rs                                       # Minimal upright observation camera
~~~

**Structure Decision**: Keep the single application crate. These small modules
separate current concerns that already have distinct ownership; they are not
generic engine, physics, ball-controller, or track abstractions. If the direct
implementation is clearer in fewer files, fewer files win.

## Implementation Outline

1. Add the exact Avian dependency and configure its normal PhysicsPlugins.
   Do not add a second physics library or enable a non-default runtime system.
2. Encode the documented world constants in one plainly named location and set
   the gravity resource explicitly to negative Y.
3. Spawn a dynamic PlayerBall with a spherical collider, explicit one kilogram
   mass, rolling materials, interpolation, and a visible orientation marker.
   Spawn matching visible static cuboids and colliders for the test track.
4. Replace the fixed bootstrap view with the narrow, world-upright observation
   follow behaviour in the contract. It reads ball translation only.
5. Optionally register Avian debug drawing only in the non-default
   physics-debug Cargo feature; standard cargo run stays visually clean.
6. Document the choice and conventions, update only delivered roadmap entries,
   and execute all project quality checks plus manual desktop validation.

## Complexity Tracking

No constitution violations require justification.
