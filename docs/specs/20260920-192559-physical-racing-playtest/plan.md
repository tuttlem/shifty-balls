# Implementation Plan: Physical Racing Playtest

**Branch**: `20260920-192559-physical-racing-playtest` | **Date**: 20 September 2026 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `docs/specs/20260920-192559-physical-racing-playtest/spec.md`

## Summary

Turn the accepted four-ball SHIFT race into a fairer physical-racing playtest.
Retune the current hand-authored final turn and its approach to create a readable
wide high/low-line section, retain ordinary ball and track contact, and verify
that a player can recover and pass through physics rather than scripted aids.
Preserve the existing race lifecycle, route ordering, simple route-targeting AI,
camera purpose, and rematch path. Validate the result through focused pure tests
and a three-race native desktop procedure.

## Technical Context

**Language/Version**: Rust 2024 edition; Rust 1.95 minimum

**Primary Dependencies**: Bevy 0.19.1; Avian 3D 0.7.0

**Storage**: N/A; ephemeral local race/playtest state only

**Testing**: `cargo test` for project-owned route/order/reset helpers; native desktop manual physical-race procedure; `cargo check`, formatting, and strict Clippy

**Target Platform**: Native Linux, macOS, and Windows desktop with graphics/display support

**Project Type**: Single-crate desktop game prototype

**Performance Goals**: Retain responsive play and stable readable rendering for four simultaneous racers on the existing compact course; no optimisation work without measured evidence

**Constraints**: Preserve SHIFT-only human/AI baseline; no direct velocity, transform, orientation, path, catch-up, collision-specific steering, teleport recovery, new dependencies, networking, or generic track/controller framework

**Scale/Scope**: One four-racer point-to-point course; widen the current strong-bank section and immediate run-out to a consistent approximately 16 m racing zone, soften its peak bank modestly, and widen its exit gate to match; retain existing camera/readout/lifecycle and perform a three-race owner playtest

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Pre-design review: PASS.**

| Principle | Plan response |
|---|---|
| Fun and understandable physics | Use visible course space, gravity, contact, and existing internal-mass control so collisions create recoverable pressure rather than arbitrary outcomes. |
| Physics is the control scheme | Preserve SHIFT centre-of-mass control for human and AI; add no vehicle steering, impulse button, or collision override. |
| Momentum and collisions create gameplay | Make a broad bank/approach the passing experiment; success is observed recovery and changed order, not a scripted pass. |
| Simple systems and scope discipline | Retune one current hand-authored course section and reuse current ordered route/race state. Defer track graphs, alternate-route rules, AI strategy, checkpoints, and multiplayer. |
| Testability and quality | Add only pure geometry/progression tests where meaningful; manually validate third-party physics contacts; run project quality gates. |
| Playable progress | Produce a directly playable four-ball race iteration before adding presentation or infrastructure. |

No constitutional violations or complexity exceptions are required.

## Project Structure

### Documentation (this feature)

```text
docs/specs/20260920-192559-physical-racing-playtest/
├── plan.md              # This file ($speckit-plan command output)
├── research.md          # Phase 0 output ($speckit-plan command)
├── data-model.md        # Phase 1 output ($speckit-plan command)
├── quickstart.md        # Phase 1 output ($speckit-plan command)
├── contracts/           # Phase 1 output ($speckit-plan command)
└── tasks.md             # Phase 2 output ($speckit-tasks command - NOT created by $speckit-plan)
```

### Source Code (repository root)
```text
src/
├── ai.rs                # Existing SHIFT route-targeting opponents; no strategic controller expansion
├── camera.rs            # Existing human-following camera; targeted readability adjustment only if playtest proves necessary
├── course.rs            # Hand-authored surface, wall, and ordered-gate geometry for the passing/recovery section
├── physics.rs           # Visible shared ball/track material constants, only if measured tuning is needed
├── race.rs              # Existing lifecycle, ordering, reset, and any focused pure regression tests
└── scene.rs             # Existing fair four-racer start grid and racer visual identity

docs/
├── race.md              # Updated physical-race observations and signed-off outcome
└── roadmap.md           # Mark only demonstrated Milestone F outcomes
```

**Structure Decision**: Retain the single crate and current focused modules.
`course.rs` owns the course-space experiment, `race.rs` owns generic race
invariants, and existing AI remains a simple writer of the same SHIFT mass
state. This avoids premature track, collision, or controller abstractions.

## Post-design Constitution Check

**PASS.** Research and design retain ordinary Avian contact, the existing
centre-of-mass control path, and a single hand-authored race course. The desktop
contract forbids rubber-banding, scripted passes, collision attacks, and
teleport recovery. Design artifacts require documented negative findings rather
than expanding scope to hide them.
