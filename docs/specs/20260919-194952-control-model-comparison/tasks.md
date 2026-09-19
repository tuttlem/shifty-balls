---

description: "Task list for the post-First-Race Control Model Comparison"
---

# Tasks: Control Model Comparison

**Input**: Design documents from `docs/specs/20260919-194952-control-model-comparison/`

**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [desktop contract](contracts/desktop-control-comparison.md), and [quickstart.md](quickstart.md)

**Tests**: The specification explicitly requires focused deterministic coverage for pure model, selection, timer, best-time, reset, and progression calculations. It intentionally does not require rigid-body simulation tests.

**Organization**: Tasks are grouped by user story so each outcome can be implemented and validated independently after shared control/session foundations are ready.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the focused module boundaries and expose their intent in the executable application without adding dependencies or a generic controller framework.

- [X] T001 Create the focused control-model module declaration and comparison module declaration in `src/main.rs`.
- [X] T002 Create `src/control_model.rs` with the `ControlModel`, shared world-horizontal `ControlIntent`, focused tuning resource, temporary selection labels, and documented no-hidden-steering invariants.
- [X] T003 Create `src/comparison.rs` with the human-only comparison/session state, attempt phase, per-model session-best record, and shared reset contract described in `docs/specs/20260919-194952-control-model-comparison/data-model.md`.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the pure, testable state and safe simulation seams that every comparison model needs before a playable attempt can exist.

**⚠️ CRITICAL**: Complete this phase before integrating any model into the live desktop loop.

- [X] T004 Add deterministic unit tests for direction normalisation, zero intent, world-space torque-axis mapping, force mapping, and model selection in `src/control_model.rs`.
- [X] T005 Add deterministic unit tests for comparison timer transitions, per-model best-time updates, model-change normalisation, and attempt reset invariants in `src/comparison.rs`.
- [X] T006 Refactor only genuinely shared human reset/progression/finish helpers in `src/race.rs` so comparison can reuse ordered gates and valid finish eligibility without importing race-position assumptions; retain existing race tests.
- [X] T007 Add the minimum marker/state support for race-only opponents and a human comparison participant in `src/physics.rs` and `src/scene.rs`, ensuring non-comparison balls can be physically excluded rather than merely hidden.
- [X] T008 Register the comparison/control resources and establish explicit Update and FixedPostUpdate schedule ordering in `src/main.rs`, with player force/torque work before `PhysicsSystems::Prepare`.

**Checkpoint**: Shared selection, timing, reset, route, and physics scheduling foundations are testable; no comparison model has yet been accepted as playable.

---

## Phase 3: User Story 1 - Compare a control model on the course (Priority: P1) 🎯 MVP

**Goal**: A player can select SHIFT, TORQUE, or FORCE and drive one isolated ball through the existing prototype course with exactly one honest physical control effect.

**Independent Test**: Launch the desktop app, select `1`, `2`, and `3` in turn, use identical W/A/S/D requests from a normalised start, and observe gravity/collision-driven SHIFT, rotational TORQUE, and centre-of-mass FORCE responses without direct velocity or trajectory manipulation.

### Tests for User Story 1

- [X] T009 [US1] Extend pure coverage in `src/control_model.rs` to prove SHIFT, TORQUE, and FORCE are mutually exclusive and that TORQUE/FORCE neutral input produces no player request.
- [X] T010 [US1] Extend pure coverage in `src/comparison.rs` to prove a model selection or `R` reset clears internal mass, control intent, motion-reset request, timer, and progression state before another comparison attempt begins.

### Implementation for User Story 1

- [X] T011 [US1] Read W/A/S/D once into the shared stable world-relative `ControlIntent` and implement the `1`/`2`/`3` selection/reset lifecycle in `src/control_model.rs` and `src/comparison.rs` according to `contracts/desktop-control-comparison.md`.
- [X] T012 [US1] Adapt `src/mass_shift.rs` so the human comparison ball consumes intent and applies its existing centre-of-mass model only for SHIFT, returns its mass/centre of mass to neutral for TORQUE/FORCE, and leaves AI SHIFT behaviour race-only.
- [X] T013 [US1] Apply TORQUE as fixed-step world torque derived from `up × intent` using Avian's one-step force interface in `src/control_model.rs`; do not write angular velocity, transform, or desired rotation.
- [X] T014 [US1] Apply FORCE as fixed-step horizontal centre-of-mass force using Avian's one-step force interface in `src/control_model.rs`; do not write linear velocity, transform, desired trajectory, or an off-centre force.
- [X] T015 [US1] Implement comparison entry/reset and human-only gate/finish timing in `src/comparison.rs`, reusing the existing prototype course, start, camera snap, route, and finish rules without duplicating track geometry.
- [X] T016 [US1] Update `src/scene.rs`, `src/camera.rs`, and `src/main.rs` so comparison physically excludes opponents, retains equivalent human ball/course/camera baseline, and schedules exactly one selected model while gravity, collision, friction, and momentum remain active.
- [X] T017 [US1] Manually validate SHIFT, TORQUE, and FORCE using the selection, reset, neutral-input, slope/bank, airborne, and no-direct-steering checks in `docs/specs/20260919-194952-control-model-comparison/quickstart.md`.

**Checkpoint**: All three control models can independently drive a comparable, single-ball course attempt with no conventional hidden steering.

---

## Phase 4: User Story 2 - Make a fair, observable comparison (Priority: P2)

**Goal**: A playtester can see which model is active, understand its physical request, repeat normalised attempts, and preserve evidence rather than rely on memory.

**Independent Test**: In each model, confirm the active label before input, inspect the appropriate F3 display, finish or reset attempts, and verify the timer/best record belongs only to the selected model.

### Tests for User Story 2

- [X] T018 [US2] Add deterministic tests for model-specific display labels and per-model best-time comparison/retention in `src/control_model.rs` and `src/comparison.rs`.

### Implementation for User Story 2

- [X] T019 [US2] Add a minimal comparison readout in `src/comparison.rs` showing selected model, current attempt time, selected-model session best, and temporary controls without replacing the established race readout.
- [X] T020 [US2] Extend the F3 development display in `src/scene.rs` to show world-stable shared intent and model-specific mass, torque, or force state alongside useful velocity context; keep it observational and development-only.
- [X] T021 [US2] Centralise and document quick-tuning values for TORQUE/FORCE in `src/control_model.rs` while retaining SHIFT tuning in `src/mass_shift.rs`; do not introduce a configuration framework or model-specific hidden assistance.
- [X] T022 [US2] Create `docs/control-model-comparison.md` and update `README.md` with comparison controls, reference frame, neutral/airborne behaviour, tuning table, fairness constants, playtest procedure, observation table, and an explicitly provisional-or-inconclusive decision record.
- [X] T023 [US2] Update `docs/course.md` and `docs/race.md` to distinguish the human-only comparison from the accepted First Race SHIFT path and retain the documented difficult-course/final-turn limitations as playtest confounders.
- [X] T024 [US2] Manually perform three rotated-order attempts per model and record honest findings for response, predictability, momentum, banks, recovery, airborne behaviour, completion/time where valid, enjoyment, and unresolved questions in `docs/control-model-comparison.md`.

**Checkpoint**: The comparison is visible, repeatable, and documented well enough to support an evidence-based provisional model decision.

---

## Phase 5: User Story 3 - Preserve the established race (Priority: P3)

**Goal**: The accepted First Race remains a usable four-racer SHIFT regression path while the control experiment deliberately leaves AI conversion for later.

**Independent Test**: Select `4`, race through countdown, physical traffic, valid progression, finish/result, and `R` rematch; verify the human and AI remain on their established SHIFT lifecycle.

### Tests for User Story 3

- [X] T025 [US3] Add or adjust focused regression tests for race/comparison session transitions and racer reset isolation in `src/race.rs` and `src/comparison.rs` without attempting multi-body physics tests.

### Implementation for User Story 3

- [X] T026 [US3] Implement the `4` transition and active-session `R` dispatch in `src/comparison.rs`, `src/race.rs`, and `src/main.rs` so First Race restores all four physical racers, countdown, progression, positions, result, and rematch state.
- [X] T027 [US3] Verify `src/ai.rs` remains deliberately SHIFT-only and gate it to the First Race session in `src/main.rs`; do not add TORQUE/FORCE AI, direct movement, or a generic controller hierarchy.
- [X] T028 [US3] Manually run the First Race regression procedure in `docs/specs/20260919-194952-control-model-comparison/quickstart.md`, including countdown, contact, finish/result, and rematch, then record the result in `docs/control-model-comparison.md`.

**Checkpoint**: The new experiment has not removed or weakened the earlier First Race vertical slice.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Complete honest documentation/history, verify scope, and leave the repository healthy.

- [X] T029 Update `docs/roadmap.md` to preserve First Roll → First Shift → First Course → First Race history and add the unchecked-or-demonstrated post-First-Race Control Model Comparison milestone without marking Physical Racing or future AI conversion complete.
- [X] T030 Review `src/control_model.rs`, `src/comparison.rs`, `src/mass_shift.rs`, `src/race.rs`, `src/scene.rs`, and `src/main.rs` for prohibited direct velocity/orientation/path changes, hidden traction/steering, stale model state, and unnecessary abstractions; record nonessential discoveries in `docs/roadmap.md`.
- [X] T031 Run `cargo check`, `cargo test`, `cargo fmt --all -- --check`, and `cargo clippy --all-targets --all-features -- -D warnings` from the repository root using `Cargo.toml`.
- [X] T032 Run the complete desktop procedure in `docs/specs/20260919-194952-control-model-comparison/quickstart.md`, confirm all 9 model labels before input and the First Race regression, and update the observation/decision record accurately.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1**: No dependencies; creates focused module boundaries.
- **Phase 2**: Depends on Phase 1 and blocks live model integration.
- **US1 (Phase 3)**: Depends on Phase 2; is the MVP comparison loop.
- **US2 (Phase 4)**: Depends on US1 because it observes real selected-model state and attempts.
- **US3 (Phase 5)**: Depends on the session switch from US1; it verifies the preserved race path independently.
- **Polish (Phase 6)**: Depends on all desired stories and their manual evidence.

### User Story Dependencies

- **US1 (P1)**: Starts after the foundation and produces the minimum useful three-model comparison.
- **US2 (P2)**: Builds on US1's live state to make evidence fair and inspectable.
- **US3 (P3)**: Builds on the session switch but does not require AI conversion; it is a regression verification of First Race.

### Parallel Opportunities

- T004 and T005 can proceed in parallel after T002/T003 because they test different focused modules.
- T007 and T006 can proceed in parallel after Phase 1 because they modify separate simulation/session seams.
- After US1 selection state exists, T018 can begin while T019/T020 are implemented in separate files.
- T022 and T023 can proceed in parallel once temporary controls and session behaviour are stable.
- T025 can be prepared while US2 documentation work is underway, then executed after session integration is complete.

## Parallel Example: User Story 2

```text
Task: "Add display/best-time tests in src/control_model.rs and src/comparison.rs"
Task: "Extend world-stable development display in src/scene.rs"
Task: "Create durable comparison record in docs/control-model-comparison.md and update README.md"
Task: "Update First Course and First Race cross-references in docs/course.md and docs/race.md"
```

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete focused control/session foundations.
2. Implement SHIFT, TORQUE, and FORCE selection with single-ball physical isolation.
3. Validate direct desktop behaviour and absence of conventional steering.
4. Stop before presentation/documentation expansion if the three models cannot yet be evaluated honestly.

### Incremental Delivery

1. Foundation + US1: tangible control-model comparison.
2. US2: observable state, timing/best evidence, and playtest record.
3. US3: verified First Race preservation with SHIFT-only AI.
4. Final validation/history: quality gates and an accurate roadmap decision record.

## Notes

- Every task uses the required checkbox, ID, path, and applicable user-story label format.
- `[P]` is intentionally omitted from individual tasks that share source files or depend on earlier live-session integration; the parallel opportunities above identify safe independent work.
- Do not mark a preferred model, roadmap item, or milestone outcome complete until manual playtest evidence exists.
