---

description: "Dependency-ordered tasks for the Physical Racing Playtest"
---

# Tasks: Physical Racing Playtest

**Input**: Design documents from `docs/specs/20260920-192559-physical-racing-playtest/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [desktop contract](contracts/desktop-physical-race.md), and [quickstart.md](quickstart.md)

**Tests**: Add deterministic tests only for project-owned course/gate geometry
and existing race invariants. Validate multi-body traffic, recovery, and passing
through the native desktop procedure rather than reproducing physics middleware.

**Organization**: Tasks are grouped by user story so recoverable traffic can be
implemented and assessed before passing and repeated-race evidence are added.

## Phase 1: Setup (Shared Course Boundaries)

**Purpose**: Make the targeted racing section explicit without introducing a
track system or changing the established race lifecycle.

- [X] T001 Define named physical-racing section dimensions and an explicit per-gate width path in `src/course.rs` so the strong bank, run-out, and matching exit gate can be tuned together.
- [X] T002 Add deterministic tests for the new configurable gate width and the existing surface endpoint/transform helpers in `src/course.rs`.

---

## Phase 2: Foundational (Regression Preconditions)

**Purpose**: Lock in the route and reset invariants that traffic tuning must not
weaken before modifying the live course.

**⚠️ CRITICAL**: Complete this phase before validating any user story.

- [X] T003 [P] Add focused full-width valid-line and out-of-route shortcut coverage for the racing-section gate in `src/course.rs` and `src/physics.rs`.
- [X] T004 [P] Add or retain focused regression coverage that common release/rematch clears motion, mass, centre of mass, progress, and result state in `src/race.rs`.

**Checkpoint**: The new racing-section bounds and existing fair race/reset rules
are deterministic and protected; live physics remains intentionally unmocked.

---

## Phase 3: User Story 1 - Race Through Recoverable Traffic (Priority: P1) 🎯 MVP

**Goal**: A four-ball race reaches a visible, recoverable traffic section with
ordinary contact and no collision-specific assistance.

**Independent Test**: Start a race, suffer an ordinary in-bounds traffic
displacement, regain a visible route with normal SHIFT control, and finish
without `R`.

### Implementation for User Story 1

- [X] T005 [US1] Retune the strong bank and immediate run-out into the approximately 16 m wide, modestly softened, continuously walled recovery section in `src/course.rs`.
- [X] T006 [US1] Apply the matching full-width progression gate to the exit of the racing section in `src/course.rs` without changing ordered-route semantics in `src/race.rs`.
- [X] T007 [US1] Verify ordinary shared ball/track contact remains the only collision behaviour and document any measured, narrowly justified material adjustment in `src/physics.rs` and `docs/physics.md`; leave values unchanged when no repeatable issue is observed.
- [X] T008 [US1] Run the single-race contact and recovery scenario from `docs/specs/20260920-192559-physical-racing-playtest/quickstart.md` and record an affirmative, negative, or unresolved recovery finding in `docs/race.md`.

**Checkpoint**: Traffic can be encountered and recovered from using ordinary
physics and SHIFT control, or the recorded result identifies the bounded tuning
problem honestly.

---

## Phase 4: User Story 2 - Create a Natural Passing Opportunity (Priority: P2)

**Goal**: The same connected route offers distinguishable physical lines that
allow a post-start position change without AI strategy or scripted help.

**Independent Test**: Complete repeated races through the widened section and
observe at least one post-start order change caused by line, momentum, or
ordinary traffic.

### Implementation for User Story 2

- [X] T009 [US2] Keep the fair four-racer start and centre-targeted SHIFT AI unchanged while confirming the widened exit gate preserves their valid route in `src/scene.rs`, `src/ai.rs`, and `src/course.rs`.
- [X] T010 [US2] Run the passing-line scenarios from `docs/specs/20260920-192559-physical-racing-playtest/quickstart.md`, classify every observed order change, and record overtaking/line-choice evidence in `docs/race.md`.

**Checkpoint**: A passing opportunity emerges from physical lines and traffic,
or the playtest record explains why this one geometry pass did not establish it.

---

## Phase 5: User Story 3 - Repeat a Readable Physical Race (Priority: P3)

**Goal**: A player can rematch the tuned race, understand its traffic outcomes,
and retain usable camera/readout context.

**Independent Test**: Complete three consecutive rematched races, identify
place and a forward/recovery direction after traffic, and record each outcome.

### Implementation for User Story 3

- [X] T011 [US3] Evaluate human camera framing and race-readout clarity during traffic against `contracts/desktop-physical-race.md`, making only a focused proven-necessary adjustment in `src/camera.rs` or `src/race.rs`.
- [X] T012 [US3] Complete the three-race rematch procedure in `docs/specs/20260920-192559-physical-racing-playtest/quickstart.md` and record collision readability, recovery, overtaking, line choice, fairness, camera clarity, and player-skill influence in `docs/race.md`.

**Checkpoint**: The physical-racing playtest has repeatable owner evidence,
including valid negative or unresolved results, without broader race systems.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Leave the bounded experiment documented, healthy, and accurately
represented in the long-lived roadmap.

- [X] T013 [P] Update `README.md`, `docs/course.md`, and `docs/race.md` with the tuned physical-racing controls, course intent, limitations, and signed-off observation record.
- [X] T014 [P] Update demonstrated Milestone F criteria and any deferred follow-up in `docs/roadmap.md` without marking undiscovered collision, AI, or track-language work complete.
- [X] T015 Run `cargo check`, `cargo test`, `cargo fmt --all -- --check`, and `cargo clippy --all-targets --all-features -- -D warnings` from the repository root; record any justified exception in `docs/specs/20260920-192559-physical-racing-playtest/quickstart.md`.
- [X] T016 Review `src/course.rs`, `src/physics.rs`, `src/ai.rs`, `src/camera.rs`, and `src/race.rs` for forbidden direct movement, collision-specific assistance, rubber-banding, unmeasured solver tuning, stale state, and unnecessary abstractions; record only nonessential discoveries in `docs/roadmap.md`.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Starts immediately; provides one explicit racing-section
  and gate-width seam.
- **Foundational (Phase 2)**: Depends on T001–T002 and blocks live-course work.
- **US1 (Phase 3)**: Depends on Phase 2; produces the recoverable-traffic MVP.
- **US2 (Phase 4)**: Depends on the tuned section from US1; it verifies passing
  without adding AI strategy.
- **US3 (Phase 5)**: Depends on US1 and is most useful after US2; it establishes
  repeated-race readability and evidence.
- **Polish (Phase 6)**: Depends on the desired story evidence and quality run.

### User Story Dependencies

- **US1 (P1)**: Creates the single wide/recoverable physical section and can
  stand alone as the collision-recovery MVP.
- **US2 (P2)**: Reuses US1's section and full-width gate to assess post-start
  passing; it does not require AI changes.
- **US3 (P3)**: Reuses the tuned race and validates rematch/readability across
  multiple runs; it does not add a new race system.

### Parallel Opportunities

- T003 and T004 can proceed in parallel after T001 because they cover separate
  pure course/race files.
- T013 and T014 can proceed in parallel after the signed-off observation record
  exists because they update distinct documentation files.
- The live course changes in T005–T006 remain sequential because they share
  `src/course.rs`; traffic validation follows those changes.

## Parallel Example: Foundational Phase

```text
Task: "Add full-width valid-line and shortcut route tests in src/course.rs and src/race.rs"
Task: "Add rematch-state regression coverage in src/race.rs"
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete T001–T004 to establish deterministic course and reset seams.
2. Complete T005–T007 to deliver the single recoverable racing section while
   retaining ordinary contact.
3. Complete T008 and stop for native desktop validation.
4. If recovery fails, record the failure and make only the bounded geometry or
   documented material follow-up justified by the observation.

### Incremental Delivery

1. Foundation + US1: recoverable physical traffic.
2. US2: evidence that a high/low physical line permits natural passing.
3. US3: repeatable rematch, readable camera/readout, and owner evidence.
4. Polish: accurate documentation, roadmap state, and quality gates.

## Notes

- Every task uses the required checkbox, sequential ID, applicable story label,
  and explicit file path.
- `[P]` is intentionally omitted where tasks share `src/course.rs` or depend on
  observed desktop behaviour; the listed parallel opportunities are safe.
- Solver configuration is not a planned tuning lever. Change it only when the
  documented desktop procedure demonstrates a repeatable stability defect.
- Do not sign off a Milestone F checkbox without the corresponding recorded
  owner evidence.
