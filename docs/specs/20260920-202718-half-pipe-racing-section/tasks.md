---

description: "Task list for the Half-Pipe Racing Section"
---

# Tasks: Half-Pipe Racing Section

**Input**: Design documents from `docs/specs/20260920-202718-half-pipe-racing-section/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [desktop contract](contracts/desktop-half-pipe-race.md), and [quickstart.md](quickstart.md)

**Tests**: The specification requires deterministic coverage for the hand-authored strip layout, outer-wall placement, and broad exit-gate routing. Rolling feel, traffic, recovery, camera readability, and replay value are evaluated through the required desktop procedure.

**Organization**: Tasks are grouped by user story so the geometry experiment, four-ball race proof, and owner decision can be delivered and judged in increments.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Define one focused, course-local description for the hand-authored section without introducing a reusable track system.

- [X] T001 Define documented half-pipe strip dimensions, symmetric bank angles, outer-lip wall policy, and broad exit-gate width in `src/course.rs`.
- [X] T002 Add a course-local helper that derives the five matching visible/static-collision strip placements and its forward endpoint in `src/course.rs`.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Lock down deterministic geometry and ordered-route assumptions before adding the live course segment.

**⚠️ CRITICAL**: Complete this phase before placing the half-pipe in the playable course.

- [X] T003 Add pure layout tests for five adjacent, symmetric half-pipe strips and their single connected forward endpoint in `src/course.rs`.
- [X] T004 Add pure tests that only the two exposed outer lips receive retaining walls and that the broad exit gate accepts valid lateral lines while rejecting a route bypass in `src/course.rs`.
- [X] T005 Preserve and extend ordered progression/reset regression coverage around the inserted gate in `src/race.rs`.

**Checkpoint**: The section's physical layout and route boundary are deterministic; no live race behaviour has been changed.

---

## Phase 3: User Story 1 - Navigate a Forgiving Half-Pipe (Priority: P1) 🎯 MVP

**Goal**: A player can enter a broad physical U-channel, choose a low or higher line using ordinary SHIFT control, recover visibly from an in-bounds poor line, and continue into the course.

**Independent Test**: Run two normal races: use the low line in one and a higher wall line in the other, then reach the next required route region without `R` or a developer-only reset.

### Implementation for User Story 1

- [X] T006 [US1] Insert the five-strip half-pipe after the forgiving approach bend and before the existing wide traffic bank in `src/course.rs`.
- [X] T007 [US1] Connect the half-pipe exit to the existing traffic-bank/run-out surfaces and add its broad ordered gate in `src/course.rs`.
- [X] T008 [US1] Review `src/physics.rs`, `src/mass_shift.rs`, and `src/control_model.rs` to retain the existing gravity, material, collision, and SHIFT-only physical-control baseline without assistance changes.
- [X] T009 [US1] Document the section geometry, low/high-line intent, visible recovery route, and low/high/recovery owner observations in `docs/course.md`.
- [X] T010 [US1] Perform and record the low-line, higher-line, wall-climb, and low-speed recovery checks from `docs/specs/20260920-202718-half-pipe-racing-section/quickstart.md` in `docs/course.md`.

**Checkpoint**: The course independently supports readable low/high physical lines and an honest normal-control recovery attempt.

---

## Phase 4: User Story 2 - Race Through the Half-Pipe (Priority: P2)

**Goal**: The established four-ball race can use the new section without a routine bottleneck or loss of ordered progression, result, or rematch.

**Independent Test**: Complete repeated four-ball races, observe ordinary traffic, side-by-side running, or an order change in or around the pipe, and finish without any special recovery mechanic.

### Implementation for User Story 2

- [X] T011 [US2] Keep the existing centre-targeting SHIFT AI and generic gate progression unchanged for the new exit gate in `src/ai.rs` and `src/race.rs`.
- [X] T012 [US2] Verify the four-racer countdown, physical contact, ordered progression, place/result, and `R` rematch traverse the new section in `src/scene.rs` and `src/race.rs`.
- [X] T013 [US2] Record half-pipe traffic, side-by-side/order-change, route-progression, finish, and rematch findings in `docs/race.md` using `docs/specs/20260920-202718-half-pipe-racing-section/quickstart.md`.

**Checkpoint**: The half-pipe works in the real race rather than only as a solo geometry demonstration.

---

## Phase 5: User Story 3 - Learn Whether Half-Pipes Belong in the Game (Priority: P3)

**Goal**: Repeated rematches produce an honest owner decision on whether this track language improves understandable, replayable racing.

**Independent Test**: Complete three rematched races, attempt low and high lines, and record affirmative, negative, or unresolved findings for every required observation category.

### Implementation for User Story 3

- [X] T014 [US3] Assess high-line and post-contact framing with the existing follow camera; make a focused change only if the procedure identifies a specific unreadable state in `src/camera.rs`.
- [X] T015 [US3] Complete three rematched four-racer runs and record usefulness, line choice, momentum, recovery, traffic, camera readability, and replay-value findings in `docs/course.md` and `docs/race.md`.
- [X] T016 [US3] Record the resulting affirmative, negative, or unresolved half-pipe decision and any bounded follow-up question in `docs/roadmap.md`.

**Checkpoint**: The project has usable evidence for or against half-pipes, including valid negative evidence.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Verify scope, documentation, quality, and roadmap history before considering the feature signed off.

- [X] T017 [P] Reconcile half-pipe controls, expected behaviour, playtest evidence, and known limitations across `README.md`, `docs/course.md`, and `docs/race.md`.
- [X] T018 Review `src/course.rs`, `src/race.rs`, `src/ai.rs`, `src/camera.rs`, `src/physics.rs`, and `src/mass_shift.rs` for prohibited direct movement, auto-centering, teleporting, collision aid, scripted passing, AI strategy, or generic track infrastructure; record nonessential discoveries in `docs/roadmap.md`.
- [X] T019 Run `cargo check`, `cargo test`, `cargo fmt --all -- --check`, and `cargo clippy --all-targets --all-features -- -D warnings` from `Cargo.toml`.
- [X] T020 Re-run the complete desktop three-race validation in `docs/specs/20260920-202718-half-pipe-racing-section/quickstart.md` and update the evidence in `docs/course.md` and `docs/race.md` accurately.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Defines the one-off half-pipe layout and may begin immediately.
- **Foundational (Phase 2)**: Depends on setup and blocks live placement until deterministic layout and route checks exist.
- **US1 (Phase 3)**: Depends on the foundation and is the MVP physical-line experiment.
- **US2 (Phase 4)**: Depends on the live US1 section; it proves the same geometry under the established four-ball lifecycle.
- **US3 (Phase 5)**: Depends on the working race path and produces the required evidence-based decision.
- **Polish (Phase 6)**: Depends on all desired story work and complete owner observations.

### User Story Dependencies

- **US1 (P1)**: Starts after Phase 2 and delivers the smallest useful half-pipe experiment.
- **US2 (P2)**: Uses US1's placed section; no new race system, AI strategy, or collision rule is permitted.
- **US3 (P3)**: Uses US1 and US2's playable race evidence to decide whether this track language is worth keeping.

### Parallel Opportunities

- T003 and T005 can be prepared in parallel after T001/T002 because they cover separate `course.rs` and `race.rs` invariants.
- T009 can be drafted while T006/T007 are implemented, then updated against the actual section.
- T014's camera assessment and T013's traffic-record template can be prepared in parallel after US1, but their final observations require the live race.
- T017 can run alongside the final quality checks once the owner evidence is complete.

## Parallel Example: User Story 2

```text
Task: "Keep the centre-targeting SHIFT AI/gate behaviour unchanged in src/ai.rs and src/race.rs"
Task: "Prepare the traffic, progression, finish, and rematch observation record in docs/race.md"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete the course-local geometry and deterministic route checks.
2. Place and connect the five-strip half-pipe without changing physics or controls.
3. Run low/high/recovery checks with normal controls.
4. Stop if the section cannot produce an understandable, recoverable physical line choice; record that finding before expanding scope.

### Incremental Delivery

1. Setup + foundation: deterministic, hand-authored geometry ready for placement.
2. US1: a playable single-driver low/high/recovery experiment.
3. US2: proof that the same section survives the four-ball race lifecycle.
4. US3: three-race evidence and an honest roadmap decision.
5. Polish: quality gates and accurate documentation/history.

## Notes

- Every task uses the required checkbox, task ID, applicable user-story label, and exact file path.
- `[P]` appears only where the work can proceed independently without creating a same-file dependency.
- Do not mark a Milestone G item complete until the desktop evidence demonstrates its acceptance criteria.
