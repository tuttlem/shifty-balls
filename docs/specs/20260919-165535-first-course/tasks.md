---
description: "Implementation tasks for the first-course control kill test"
---

# Tasks: First Course Kill Test

**Input**: Design documents in `docs/specs/20260919-165535-first-course/`.

**Tests**: The specification requires deterministic tests for project-owned
attempt, finish, restart, and any reusable construction calculations. Do not
duplicate Avian's rigid-body contact solver in tests.

**Organization**: Tasks are grouped by user story so the physical course,
fast-repeat loop, and honest evaluation can each be reviewed independently.

## Phase 1: Setup

**Purpose**: Establish the two focused module boundaries, without adding a
crate, dependency, plugin framework, or general game state system.

- [X] T001 Declare focused `course` and `attempt` modules in src/main.rs

---

## Phase 2: Foundational Preparation

**Purpose**: Define the few shared conventions required by every course story.

**CRITICAL**: Complete this before user-story work.

- [X] T002 Define the course start transform and any shared course/finish marker components in src/physics.rs
- [X] T003 Move the ball spawn to the defined course start transform while preserving its existing rigid-body, centre-of-mass, and visual-rotation setup in src/scene.rs

**Checkpoint**: The one existing physics-controlled ball has one explicit,
reusable start state but no course, timer, or racing system yet.

---

## Phase 3: User Story 1 - Complete a Deliberate Physical Course (Priority: P1) 🎯 MVP

**Goal**: Deliver one connected, readable, hand-authored physical course whose
opening, bends, high/low line, crest, and boundaries require intentional mass
shifting but keep a poor line recoverable.

**Independent Test**: Run `cargo run`, travel from the defined start to the
visually obvious final apron, and compare deliberate versus unplanned lines
through the gentle bend, stronger bank, and momentum crest.

### Tests for User Story 1

- [X] T004 [US1] Write failing deterministic tests for surface-endpoint advancement and continuous, thickness-aware primitive placement in src/course.rs

### Implementation for User Story 1

- [X] T005 [US1] Implement the focused matching render-and-collider surface helper with gentle overlap and optional matching retaining walls in src/course.rs
- [X] T006 [US1] Hand-author the connected opening descent, gradual gentle bend, stronger bank/high-low recovery section, compression, crest, finish apron, and readable course materials in src/course.rs
- [X] T007 [US1] Spawn the prototype course at startup and remove the disposable first-roll test geometry from src/main.rs
- [X] T008 [US1] Add an obvious visual finish gate and a broad final-apron FinishRegion marker without a race/checkpoint system in src/course.rs
- [X] T009 [US1] Make only targeted world-up follow-camera framing changes needed to reveal the opening, bends, bank, and crest early in src/camera.rs
- [X] T010 [US1] Manually validate the complete physical route, deliberate versus poor bank lines, height-versus-speed opportunity, crest momentum, and one recovery path using docs/specs/20260919-165535-first-course/contracts/desktop-course.md

**Checkpoint**: The kill-test course can be physically traversed with the
existing mass-shift mechanic, though attempts are not timed or restartable yet.

---

## Phase 4: User Story 2 - Retry and Measure Improvement (Priority: P2)

**Goal**: Make a fair fresh attempt one key press away and make improvement
visible through current, completed, and session-best timing.

**Independent Test**: Complete a run, press R, complete another run, and verify
the ball resets fully while best time persists and only improves for a faster
finish.

### Tests for User Story 2

- [X] T011 [US2] Write failing deterministic tests for attempt ticking, restart preservation of best time, one-shot completion, best-time selection, and inclusive finish bounds in src/attempt.rs

### Implementation for User Story 2

- [X] T012 [US2] Implement the focused Running/Completed attempt state, elapsed/completed/session-best calculations, and pure finish-bounds predicate in src/attempt.rs
- [X] T013 [US2] Implement R restart to restore ball transform, linear/angular velocity, centre of mass, internal-mass state, and active attempt while retaining session best in src/attempt.rs
- [X] T014 [US2] Add one-shot FinishRegion detection and current/completed/best time progression without race, lap, checkpoint, or result systems in src/attempt.rs
- [X] T015 [US2] Add the lightweight development readout for attempt state, best time, speed, and restart hint in src/attempt.rs
- [X] T016 [US2] Add a camera start-transform helper and snap the camera on restart before normal follow smoothing resumes in src/camera.rs
- [X] T017 [US2] Wire course attempt startup, restart, finish, timing, readout, and camera-reset system ordering in src/main.rs
- [X] T018 [US2] Manually validate R during rolling, stalled, and completed states plus two completed attempts and session-best retention using docs/specs/20260919-165535-first-course/quickstart.md

**Checkpoint**: The player can repeat the complete course rapidly and compare
results, with no racing infrastructure beyond one active attempt.

---

## Phase 5: User Story 3 - Understand and Evaluate the Kill Test (Priority: P3)

**Goal**: Keep the course, mass state, speed, timing, and camera readable
enough to record honest gameplay evidence rather than manufacture a positive
result.

**Independent Test**: During repeated attempts, identify each upcoming major
challenge, mass state, speed, active/best time, and finish; then record every
kill-test question as affirmative, negative, or unresolved.

### Implementation for User Story 3

- [X] T019 [US3] Add a focused F3 development-display toggle that gates the mass gizmos and attempt readout without changing gameplay state in src/attempt.rs
- [X] T020 [US3] Make the existing mass display honour the focused development-display state and add only a bounded velocity-direction cue in src/scene.rs
- [X] T021 [US3] Document the course layout, construction conventions, controls, timing, jump omission, tuning/camera changes, kill-test observation template, known issues, and unresolved questions in docs/course.md
- [X] T022 [US3] Update current prototype status, course controls, and documentation links in README.md
- [X] T023 [US3] Run the repeated manual kill test, record all ten observation outcomes honestly, and validate display/camera readability in docs/course.md

**Checkpoint**: The project has evidence for its next decision, including any
negative or unresolved result, rather than a cosmetic course completion claim.

---

## Phase 6: Polish and Cross-Cutting Completion

**Purpose**: Validate the bounded experiment, update only supported roadmap
outcomes, and verify no conventional steering or general racing work entered.

- [X] T024 Run cargo check, cargo test, cargo fmt --all -- --check, and cargo clippy --all-targets --all-features -- -D warnings from the repository root defined by Cargo.toml
- [X] T025 Run the complete desktop course contract and repeated-attempt validation in docs/specs/20260919-165535-first-course/quickstart.md
- [X] T026 Update only demonstrated Near-Term, Core Control Kill Test, Camera, Track Geometry, Track Representation, Speed and Momentum, Recovery and Failure, Debugging Tools, Physics Tuning, and Milestone D items in docs/roadmap.md
- [X] T027 Review src/main.rs, src/course.rs, src/attempt.rs, src/camera.rs, src/scene.rs, and src/mass_shift.rs for direct steering, race infrastructure, speculative track frameworks, or other excluded work

---

## Dependencies and Execution Order

### Phase dependencies

- Setup has no dependencies.
- Foundational work depends on T001 and blocks user-story work.
- US1 depends on the explicit start state and is the first playable course.
- US2 depends on US1's course start and finish marker.
- US3 depends on the course and attempt information established by US1 and US2.
- Completion work depends on every desired user story.

### User-story graph

```text
Setup -> Foundational -> US1 physical course -> US2 restart and timing -> US3 evaluate -> completion
```

### Parallel opportunities

- After US1's course geometry is settled, T011 can start alongside T009's
  manual course evaluation because they modify different files.
- After US2 is complete, T021 and T022 can be prepared in parallel because
  their documentation files are distinct, but T023 waits for the manual runs.
- Automated validation in T024 can run after the code tasks while the manual
  kill-test record is being completed, but roadmap updates wait for evidence.

## Parallel Example: Timing and Course Validation

```text
After the US1 route is implemented:
Task: "Write attempt-state tests in src/attempt.rs"
Task: "Manually validate the physical course contract in docs/specs/20260919-165535-first-course/contracts/desktop-course.md"
```

## Implementation Strategy

### MVP first

1. Complete setup and foundational preparation.
2. Complete T004 through T010.
3. Stop and validate that the course itself creates understandable physical
   choices before adding timing or polish.

### Incremental delivery

1. Add the connected physical course and validate its lines, recovery, and
   crest without compensating for weak control.
2. Add R, finish, and timing only after the physical route is trusted.
3. Add focused evaluation visibility and record the honest kill-test result.
4. Run quality checks, then update the roadmap only from demonstrated evidence.

## Format Validation

All 27 tasks use the required checkbox, sequential T identifier, applicable
user-story label, and exact file path. Tests precede their associated
implementation work; no task adds a dependency, crate, general track system,
or racing feature.
