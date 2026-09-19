# Implementation Plan: First Course Kill Test

**Branch**: `feature/first-course` | **Date**: 2026-09-19 | **Spec**: [spec.md](spec.md)

## Summary

Build one compact, connected, hand-authored physical course which answers
whether mass shifting creates a desire for another run. Replace the disposable
test geometry with a broad opening descent, a gradual gentle bend, a stronger
bank with a high/low line, a momentum rise and crest, a recoverable run-out,
and an obvious finish. Add only a focused attempt resource: R restarts the
single ball; current, completed, and session-best time appear in a development
readout. Preserve the existing centre-of-mass control and world-stable mass
display; no conventional steering or racing framework is added.

## Technical Context

**Language/Version**: Rust edition 2024; Rust 1.95.0 via `rust-toolchain.toml`

**Primary Dependencies**: Existing Bevy 0.19.1 and avian3d 0.7.0; no new dependency

**Storage**: In-memory attempt and session-best state only

**Testing**: Focused deterministic unit tests for attempt timing, best-time
comparison, finish bounds, and restart-state calculations; manual course and
kill-test trials; `cargo check`, `cargo test`, `cargo fmt --all -- --check`,
and `cargo clippy --all-targets --all-features -- -D warnings`

**Target Platform**: Existing supported native desktops

**Project Type**: Single Rust desktop application crate

**Performance Goals**: A normal development desktop run remains visually
smooth with one ball, static primitive course, development overlay, and mass
gizmos; no scalability target is claimed

**Constraints**: One hand-authored course; one dynamic ball; world-up readable
camera; fast R restart; session-only best time; direct mass-property coupling
remains the only player-to-ball control path. No jump is planned initially so
the course evaluates grounded control cleanly.

**Scale/Scope**: Approximately 80–115 metres of connected primitive course,
normally targeting a 30–90 second competent run; four small source modules
plus documentation and no new crate

## Constitution Check

### Pre-research gates

| Gate | Result | Planning response |
|---|---|---|
| Fun and satisfying physical control | Pass | The course is an evidence-gathering kill test, not content expansion. |
| Physics is the control scheme | Pass | Course choices arise from the existing centre-of-mass system, gravity, contact, and geometry only. |
| Momentum creates gameplay | Pass | Descent, high/low bank line, compression, and crest deliberately make line quality matter. |
| Emergent simple systems | Pass | Repeated static cuboids and ordinary collision replace splines, boosts, and bespoke course rules. |
| Practical determinism and testing | Pass | Pure attempt, finish, and comparison logic is tested; third-party contact solving is manually assessed. |
| Presentation without artificial abstraction | Pass | Direct Bevy geometry, UI, and gizmos read project-owned state without an engine wrapper. |
| Playable progress and scope discipline | Pass | One replayable course and timing loop; no race, AI, multiplayer, or general track system. |
| Dependencies earn their place | Pass | Existing Bevy and Avian features suffice; no dependency is added. |

### Post-design re-check

Pass. The selected primitive-surface and focused-attempt design gives the
player an immediate physical course to repeat without predicting future track
or racing needs. The optional airborne event is deliberately omitted: a modest
crest supplies the momentum question without introducing a distracting new
control question.

## Project Structure

### Documentation

```text
docs/
├── course.md
├── mass-shift.md
├── roadmap.md
└── specs/20260919-165535-first-course/
    ├── spec.md
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── quickstart.md
    └── contracts/desktop-course.md
```

### Source Code

```text
src/
├── main.rs          # App wiring and schedule ordering
├── physics.rs       # Existing shared scale, ball, and contact conventions
├── mass_shift.rs    # Existing internal-mass state and sole physics coupling
├── scene.rs         # Ball, lighting, existing mass display, shared primitive visuals
├── course.rs        # One hand-authored course and finish presentation
├── attempt.rs       # Restart, time, finish-state, readout, and pure tests
└── camera.rs        # Targeted course-readable world-up follow camera
```

**Structure Decision**: Preserve the one-crate application. `course` owns only
the one physical layout and its finish marker. `attempt` owns only one
restartable timed attempt and session best. Existing physics, mass-shift,
scene, and camera modules remain direct and focused. No reusable segment,
racing-state, or track-authoring framework is created.

## Complexity Tracking

No constitution violation needs justification.
