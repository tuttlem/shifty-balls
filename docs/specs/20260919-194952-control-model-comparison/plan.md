# Implementation Plan: Control Model Comparison

**Branch**: `control-model-comparison` | **Date**: 2026-09-19 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `docs/specs/20260919-194952-control-model-comparison/spec.md`

## Summary

Add a small, human-only comparison session to the existing prototype course so the player can make normalised attempts using SHIFT, TORQUE, or FORCE. SHIFT retains the established movable-centre-of-mass model. TORQUE and FORCE use Avian's one-step physics inputs in the fixed simulation schedule, never direct velocity, transform, or trajectory changes. A shared world-horizontal intent frame keeps all three models understandable despite ball spin.

The existing four-ball First Race remains a separate regression path, pinned to its established SHIFT controller for both the human and AI. The comparison does not generalise AI or add a game-mode framework: it provides a direct temporary selector, human-only reset/timer/readout, focused debug display, documentation, and deterministic tests for project-owned calculations.

## Technical Context

**Language/Version**: Rust 2024 edition; Rust 1.95 stable minimum

**Primary Dependencies**: Bevy 0.19.1; Avian 3D 0.7.0

**Storage**: In-memory session state only; no persistence

**Testing**: `cargo test` unit tests for pure control/session calculations; manual desktop comparison and First Race regression procedures

**Target Platform**: Native desktop on Linux, macOS, and Windows with a working graphics/display environment

**Project Type**: Single Cargo-workspace desktop game application

**Performance Goals**: Retain responsive single-course play and the current four-racer First Race; no optimisation work without measured evidence

**Constraints**: Exactly one human control effect at a time; no direct velocity/orientation/path changes; same ball/course/camera baseline for comparison; no AI rewrite; no new dependencies or production UI

**Scale/Scope**: Three human models, one existing hand-authored course, one human comparison ball, and the existing one-human/three-AI race regression path

## Constitution Check

*GATE: Passed before Phase 0 research. Re-checked after Phase 1 design: passed.*

| Constitution requirement | Plan response | Status |
|---|---|---|
| Fun and understandable physical control take priority | Compare three visible physical models on the same course and permit an inconclusive or negative result. | Pass |
| Physics is the control scheme | SHIFT changes centre of mass; TORQUE and FORCE use physical simulation inputs. No model may set velocity, orientation, or a target path. | Pass |
| Momentum and emergent systems remain central | Gravity, contact, friction, rotation, banking, and existing geometry remain active; no boosts, caps, traction assistance, or corrective steering are introduced. | Pass |
| Code coherence over cleverness | Add focused control/session state and reuse existing course, racer, progression, camera, and reset seams. No generic controller hierarchy or game-mode framework. | Pass |
| Deterministic and testable where practical | Unit-test input, selection, model mappings, neutral behaviour, session timing/best-time, and reset invariants; manually observe third-party physics. | Pass |
| Presentation must not own game concepts | Control intent, selected model, and comparison session remain ordinary focused game state. The readout/gizmos observe them rather than own them. | Pass |
| Playable progress and scope discipline | The comparison runs on the existing course, preserves First Race, and defers AI conversion, content, multiplayer, and polish. | Pass |
| Dependencies earn their place | Avian already supports fixed-step forces and torques; add no dependencies. | Pass |
| Quality and roadmap discipline | Run established Cargo checks, update durable control documentation and roadmap history only for demonstrated work, and record observations honestly. | Pass |

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260919-194952-control-model-comparison/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── desktop-control-comparison.md
└── tasks.md                 # Created later by $speckit-tasks
```

### Source Code (repository root)

```text
src/
├── main.rs              # App resources and explicit schedule ordering
├── control_model.rs     # Shared intent, selected model, TORQUE/FORCE mappings and tuning
├── comparison.rs        # Human-only comparison lifecycle, timing, selection and readout
├── mass_shift.rs        # Existing SHIFT-only mass state, smoothing and centre-of-mass coupling
├── race.rs              # Existing First Race lifecycle, progression, result and rematch
├── ai.rs                # Existing SHIFT-only AI; intentionally unchanged
├── scene.rs             # Racer physics participation and development gizmos
├── camera.rs            # Existing human-follow camera and reset snap
├── course.rs            # Existing hand-authored geometry and route gates
└── physics.rs           # Shared physical conventions and marker components

docs/
├── control-model-comparison.md  # Procedure, tuning and observations
├── roadmap.md                   # History and post-race milestone state
├── course.md                    # Comparison cross-reference
└── race.md                      # SHIFT-race regression status
```

**Structure Decision**: Retain the single application crate. Add at most the two focused modules shown above because control-model selection and comparison lifecycle have genuine responsibilities separate from SHIFT mechanics and First Race rules. Reuse the existing course, camera, physics, progression helpers, and human racer rather than duplicate a level or create generic controller, mode, or AI abstractions.

## Complexity Tracking

No constitution violations or complexity exceptions are required.
