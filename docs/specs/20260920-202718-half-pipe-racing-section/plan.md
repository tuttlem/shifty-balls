# Implementation Plan: Half-Pipe Racing Section

**Branch**: `20260920-202718-half-pipe-racing-section` | **Date**: 20 September 2026 | **Spec**: [spec.md](spec.md)

## Summary

Add one broad, hand-authored half-pipe to the accepted four-ball race. A flat low channel plus symmetric banked side strips creates a physical high/low-line experiment; visible outer walls, a connected exit, and a broad ordered gate keep recovery readable. Existing SHIFT control, ordinary contact, simple AI, race lifecycle, camera, and rematch remain in place.

## Technical Context

**Language/Version**: Rust 2024; Rust 1.95 minimum

**Primary Dependencies**: Bevy 0.19.1; Avian 3D 0.7.0

**Storage**: N/A; local ephemeral race/playtest state

**Testing**: Pure layout/gate and existing race-invariant unit tests; native desktop three-race procedure; cargo check, test, formatting, and strict Clippy

**Target Platform**: Native Linux, macOS, and Windows desktop

**Project Type**: Single-crate desktop game prototype

**Performance Goals**: Stable, readable four-racer play; no optimisation absent measured evidence

**Constraints**: Preserve ordinary gravity/contact and SHIFT-only racers; no direct movement, automatic recovery, collision help, AI strategy, track graph, or new dependency

**Scale/Scope**: One 18 m wide, 16 m long five-strip half-pipe after the forgiving bend and before the signed-off wide traffic bank; three-race owner playtest

## Constitution Check

**Pre-design review: PASS.** Static, matching visible/collision surfaces let gravity, momentum, contact, and internal-mass control decide the line. The plan adds one focused course helper, not a geometry pipeline or track framework. Pure layout/routing is tested; multi-body contact and feel are manually judged. No constitutional violation or complexity exception is required.

## Project Structure

```text
docs/specs/20260920-202718-half-pipe-racing-section/
├── plan.md
├── research.md
├── data-model.md
├── contracts/desktop-half-pipe-race.md
├── quickstart.md
└── tasks.md

src/
├── course.rs            # Half-pipe strips, outer walls, route gates, pure geometry tests
├── race.rs              # Existing ordered progression/result/rematch invariants
├── ai.rs                # Existing centre-targeting SHIFT opponents, unchanged
├── camera.rs            # Adjust only if playtest proves framing failure
└── physics.rs           # Existing material baseline, unchanged absent measured fault

docs/
├── course.md            # Geometry intent and owner observation
├── race.md              # Traffic/race finding
└── roadmap.md           # Demonstrated Milestone G updates only
```

**Structure Decision**: `course.rs` owns this single hand-authored primitive section and matching route data. `race.rs` stays generic. No reusable track abstraction is warranted for one element.

## Post-design Constitution Check

**PASS.** The design explicitly rejects hidden recovery, scripted lines/passes, collision aids, AI strategy, and generic infrastructure. A negative desktop finding remains valid evidence rather than a reason to add unrelated content.
