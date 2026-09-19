# Implementation Plan: First Race

**Branch**: `feature/first-race` | **Date**: 2026-09-19 | **Spec**: [spec.md](spec.md)

**Input**: First Race feature specification.

## Summary

Turn the approved single-ball course into a compact point-to-point physical race: one human-controlled ball and three AI-controlled balls share the current physics, countdown release, ordered course progression, live position, valid finish order, minimal race display, and one-key rematch. The implementation will refactor the existing single-player assumptions into small racer/controller boundaries, not create a general race framework. Both human and AI reach movement solely by requesting internal-mass displacement; existing gravity and contact solve the response.

## Technical Context

**Language/Version**: Rust 1.95.0 on stable  
**Primary Dependencies**: Bevy 0.19.1; Avian3D 0.7.0  
**Storage**: In-memory session race state only; no persistence  
**Testing**: Cargo unit tests for deterministic pure rules; manual native desktop race validation  
**Target Platform**: Native Linux, macOS, and Windows desktop graphics environments  
**Project Type**: Single desktop game application  
**Performance Goals**: Keep a four-ball field and existing course responsive and readable during ordinary contacts; establish no broader performance target before evidence requires it  
**Constraints**: Preserve internal-mass causality; no direct player/AI steering or collision forces; no new dependency unless a concrete requirement arises; no general track, race, multiplayer, or HUD framework  
**Scale/Scope**: One human, three AI, one existing point-to-point course, one countdown/race/rematch loop, four or five course-specific progression regions  

## Constitution Check

### Pre-design gate

| Principle | Plan response | Status |
|---|---|---|
| Fun and understandable physical control | Human and AI both manipulate a visible conceptual internal mass; gravity/contact own motion and collision response. | Pass |
| Momentum and collisions create gameplay | Four equal physical balls use ordinary ball-to-ball contact; no combat or scripted knockback. | Pass |
| Code coherence over cleverness | Introduce only racer, controller, race-state, and course-gate concepts demonstrated by the vertical slice. | Pass |
| Deterministic/testable simulation | Extract lifecycle, progression, eligibility, ordering, and reset rules as pure testable calculations; manually validate physics. | Pass |
| Presentation does not own game | Minimal display reads race state; race rules remain independent where practical. | Pass |
| Playable progress and scope discipline | Keep the existing course and field fixed; exclude laps, track selection, networking, advanced AI, production UI, and collision tuning. | Pass |
| Quality and documentation | Plan includes focused tests, native validation, documentation, and accurate roadmap update. | Pass |

No constitution violations require complexity tracking.

### Post-design gate

The completed design retains the same result: it uses racer-owned state, course-specific data, and a simple lifecycle without a new crate, generic engine interface, route graph, or multiplayer abstraction. It remains compliant.

## Project Structure

### Documentation (this feature)

    docs/specs/20260919-174545-first-race/
    ├── spec.md
    ├── plan.md
    ├── research.md
    ├── data-model.md
    ├── quickstart.md
    ├── contracts/
    │   └── desktop-race.md
    └── tasks.md                 # Created later by speckit-tasks

### Source Code (repository root)

    src/
    ├── main.rs                  # Module declaration and system ordering
    ├── physics.rs               # Shared physical conventions and racer markers
    ├── scene.rs                 # Shared racer spawning, visual identity, human-only gizmos
    ├── mass_shift.rs            # Per-racer mass state and shared physical coupling
    ├── course.rs                # Existing surfaces plus small ordered course-gate data
    ├── race.rs                  # Lifecycle, progress, finish/order, rematch, pure tests
    ├── ai.rs                    # Course-aware mass-intent AI
    └── camera.rs                # Human-only race framing/reset

**Structure Decision**: Keep the single existing application crate. Add focused modules only where the new demonstrated concepts are independently testable or otherwise clarify the human/AI/race distinction. Existing attempt functionality is folded into race state rather than retaining competing time-trial and race loops.

## Implementation Outline

1. Establish racer identity, human/AI controller markers, per-racer mass state, shared physical spawn/reset helper, and four held start transforms with distinct prototype colours.
2. Refactor the current human mass systems and camera/debug queries to select the human racer while applying the shared centre-of-mass conversion to every active racer.
3. Add a small pure race lifecycle and oriented ordered-gate model. Test countdown/timing, ordered progress, valid finish, stable order, and rematch reset before wiring presentation.
4. Stage racers during countdown and release them together at GO. Add the minimal race readout and human result.
5. Add route-aware AI that writes the same requested mass offset as human input, remains inactive until GO, and targets the next current-course region.
6. Integrate physical finishing/rematch and perform manual repeated-race validation. Record collision/AI/course observations honestly and update only delivered roadmap items.

## Complexity Tracking

No constitution-gate violations.
