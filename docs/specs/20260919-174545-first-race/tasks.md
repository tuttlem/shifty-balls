---

description: "Dependency-ordered task list for the First Race vertical slice"
---

# Tasks: First Race

**Input**: Design documents from docs/specs/20260919-174545-first-race/

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [desktop-race.md](contracts/desktop-race.md), [quickstart.md](quickstart.md)

**Tests**: Add focused Rust unit tests for deterministic lifecycle, progression, ordering, finish, and reset rules. Do not duplicate third-party physics behaviour with brittle tests.

**Organization**: Tasks are grouped by independently testable user story. The foundational phase establishes the shared racer/controller and mass-state boundary required by every story.

## Format: [ID] [P?] [Story] Description

- **[P]**: Can run in parallel because it uses different files and has no incomplete dependency.
- **[USn]**: Maps the task to its feature-spec user story.

## Phase 1: Setup

**Purpose**: Establish the small modules and documented baseline for the race slice without adding a crate, dependency, or framework.

- [X] T001 Declare focused race and AI modules and replace obsolete attempt-module wiring in src/main.rs
- [X] T002 [P] Record the First Race implementation decisions and source ownership boundaries in docs/specs/20260919-174545-first-race/plan.md

---

## Phase 2: Foundational

**Purpose**: Replace one-ball assumptions with the smallest shared racer/controller, physical mass, course-progress, and pure-rule foundations. This phase blocks all user stories.

**Critical**: Do not begin user-story work until every racer can own independent internal-mass state and the core race lifecycle/progression rules exist.

- [X] T003 Define shared Racer, HumanRacer, AiRacer, start-state, and current-course progression-gate components/constants in src/physics.rs
- [X] T004 Refactor singleton InternalMassState into per-racer mass state and generalise human intent, smoothing, centre-of-mass conversion, and wake-on-offset application in src/mass_shift.rs
- [X] T005 Refactor the human-only scene, camera, and development-gizmo queries to select HumanRacer while retaining shared physical-ball rendering support in src/scene.rs and src/camera.rs
- [X] T006 Add small ordered, oriented progression-gate construction data alongside the existing prototype course without creating a general track system in src/course.rs
- [X] T007 Write failing deterministic lifecycle, gate-order, valid-finish, stable-position, finish-order, timer, and reset tests in src/race.rs
- [X] T008 Implement the pure race lifecycle, racer progress/result records, oriented gate predicate, stable ordering, and reset calculations in src/race.rs
- [X] T009 Wire shared racer/mass/race resources and fixed/update ordering so controller output reaches physics before preparation in src/main.rs

**Checkpoint**: Four independent racers can be represented, controlled through shared internal-mass state, and compared with deterministic current-course race rules.

---

## Phase 3: User Story 1 - Race a Physical Field (Priority: P1) 🎯 MVP

**Goal**: Deliver a shared four-ball physical start and simple opponents that use the same internal-mass model as the human.

**Independent Test**: Launch the game, observe one human plus three distinct opponents staged together, wait for GO, and verify ordinary contacts visibly change racer lines without direct steering or collision attacks.

### Implementation for User Story 1

- [X] T010 [US1] Create a shared baseline physical-racer spawn helper, human visual identity, three distinct AI visual identities, and a fair spaced start arrangement in src/scene.rs
- [X] T011 [US1] Implement held preparing/countdown racers, visible 3–2–1–GO transition, simultaneous dynamic release, neutral pre-GO mass state, and active-only timer/control gating in src/race.rs
- [X] T012 [US1] Implement the minimal route-aware AI that selects its next current-course target and writes only world-horizontal requested internal-mass displacement in src/ai.rs
- [X] T013 [US1] Integrate racer spawning, countdown release, AI mass intent, human mass gating, and ball-to-ball physical simulation scheduling in src/main.rs
- [X] T014 [US1] Manually validate four-ball staging, no meaningful false start, simultaneous GO release, ordinary collision response, and recoverable player control using docs/specs/20260919-174545-first-race/contracts/desktop-race.md

**Checkpoint**: The player can race physical traffic on the existing course, and every competitor reaches motion through the internal-mass model.

---

## Phase 4: User Story 2 - Understand Who Is Winning (Priority: P2)

**Goal**: Turn the physical field into an understandable race through valid progression, live position, fixed finish order, and a minimal result.

**Independent Test**: Observe racers cross required regions in different orders, confirm the player place reflects valid progress, reject an invalid finish, then complete a valid player finish and compare its result with the physical order.

### Implementation for User Story 2

- [X] T015 [US2] Connect each racer's transform to its exact-next oriented progression gate, reject out-of-order gates, record eligible finishes once, and update the stable field ordering in src/race.rs
- [X] T016 [US2] Replace the single-ball attempt readout with the minimal countdown, player-place, race-time, and player-result display required by the desktop race contract in src/race.rs
- [X] T017 [US2] Make targeted human-follow camera framing adjustments for the staged start, nearby opponents, and finish result in src/camera.rs
- [X] T018 [US2] Wire progression, finish/result, ordering, and race display updates into the active race lifecycle in src/main.rs
- [X] T019 [US2] Manually validate valid versus invalid finishes, fixed finish order, readable player position, win/loss result, and camera readability using docs/specs/20260919-174545-first-race/quickstart.md

**Checkpoint**: The player can tell who is leading, cannot shortcut to win, and receives a clear win-or-loss result.

---

## Phase 5: User Story 3 - Race Again and Evaluate Traffic (Priority: P3)

**Goal**: Make repeated fair races quick and preserve enough development visibility and documentation to assess whether traffic improves the game.

**Independent Test**: Rematch during countdown, active racing, and after a result; each time verify all racer/race state is cleared and a fresh fair countdown begins, then repeat races and record traffic observations.

### Implementation for User Story 3

- [X] T020 [US3] Replace single-ball restart with full-race rematch reset of held body state, transforms, velocities, rotations, mass state, AI intent, progression, results, timer, display, and human camera in src/race.rs
- [X] T021 [US3] Preserve the independently toggleable human-only mass/velocity development display and update its race-facing hints in src/scene.rs and src/race.rs
- [X] T022 [US3] Update temporary controls, race lifecycle, field arrangement, progression/position rules, AI mass-control approach, rematch, limitations, and first-race observation record in README.md and docs/race.md
- [X] T023 [US3] Manually run at least three starts plus rematches during each lifecycle phase, then record collision, traffic, side-by-side, recovery, course-width, position-pressure, AI, and win-versus-completion observations honestly in docs/race.md

**Checkpoint**: The vertical slice supports fast repeated races and leaves evidence for the next collision-focused decision.

---

## Phase 6: Polish and Cross-Cutting Completion

**Purpose**: Verify the bounded implementation, protect the physical-control principle, and accurately record delivered roadmap work.

- [X] T024 Run cargo check, cargo test, cargo fmt --all -- --check, and cargo clippy --all-targets --all-features -- -D warnings from the repository root
- [X] T025 Run the complete desktop race contract and repeated-race procedure in docs/specs/20260919-174545-first-race/quickstart.md
- [X] T026 Update only demonstrated Multiple Balls, Player Identity, Race Structure, Starts, AI Racers, Camera, Recovery and Failure, Debugging Tools, Physics Tuning, Near-Term, and Milestone E items in docs/roadmap.md
- [X] T027 Review src/main.rs, src/physics.rs, src/scene.rs, src/mass_shift.rs, src/course.rs, src/race.rs, src/ai.rs, and src/camera.rs for direct steering, race-framework scope creep, multiplayer, advanced AI, or other excluded work

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: Can begin immediately.
- **Foundational (Phase 2)**: Depends on T001 and blocks all user stories.
- **User Story 1 (Phase 3)**: Depends on T003–T009.
- **User Story 2 (Phase 4)**: Depends on the physical racer field from User Story 1 and the foundational pure race rules.
- **User Story 3 (Phase 5)**: Depends on the active lifecycle/result work from User Story 2.
- **Polish (Phase 6)**: Depends on every intended story and manual validation.

### User Story Dependencies

- **US1 — Race a Physical Field**: The MVP; delivers physical traffic and same-model AI.
- **US2 — Understand Who Is Winning**: Builds on US1 because positions and results require active racers.
- **US3 — Race Again and Evaluate Traffic**: Builds on US2 because rematch must clear race results and lifecycle state.

### Parallel Opportunities

- T002 can proceed alongside T001.
- T003 and T006 can proceed in parallel after module setup; T007 can begin once the race module exists.
- T010, T011, and T012 touch distinct primary files after the foundation is complete, but T013 integrates their results and must follow them.
- T017 can proceed alongside T015–T016 once human-racer selection exists.
- T021 and T022 can proceed in parallel after the race lifecycle stabilises.

## Parallel Example: User Story 1

After T003–T009:

    Task: T010 shared racer spawning and visuals in src/scene.rs
    Task: T011 countdown/release in src/race.rs
    Task: T012 same-model AI intent in src/ai.rs

Then complete T013 integration in src/main.rs before T014 manual validation.

## Implementation Strategy

### MVP First

1. Complete setup and the shared racer/race-rule foundation.
2. Complete User Story 1 only.
3. Stop and manually validate the central question: does physical traffic create useful pressure without violating mass-based control?

### Incremental Delivery

1. Add valid progression, position, and a result only after the physical field is fun enough to race.
2. Add rematch and playtest documentation only after finish state is dependable.
3. Use observations to choose Milestone F work; do not add content to conceal weak traffic.

## Notes

- All checklist entries use a checkbox, sequential task ID, required story label where applicable, and exact file path.
- Manual tasks are deliberately explicit. Their completion must record observed results rather than assume traffic is fun.
