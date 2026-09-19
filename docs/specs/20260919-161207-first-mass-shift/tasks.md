---
description: "Implementation tasks for the first internal mass shift experiment"
---

# Tasks: First Internal Mass Shift

**Input**: Design documents in docs/specs/20260919-161207-first-mass-shift/.

**Tests**: The specification explicitly requires focused deterministic tests. Write
tests for project-owned mass-shift calculations before their implementation; do
not unit-test Avian's contact solver.

**Organization**: Tasks are grouped by user story so visible mass manipulation,
physical coupling, and experiment understanding remain independently reviewable.

## Phase 1: Setup

**Purpose**: Establish the one focused module boundary without adding a
dependency, crate, framework, or general controller.

- [X] T001 Declare the focused mass-shift module in src/main.rs

---

## Phase 2: Foundational Preparation

**Purpose**: Make the existing one-ball scene ready to identify and accept the
first experiment's mass property.

**CRITICAL**: Complete this before user-story work.

- [X] T002 Add the neutral explicit centre-of-mass property and any narrow mass-shift marker required by the existing player ball in src/scene.rs
- [X] T003 Define shared experimental world and mass constants that are genuinely common to the ball and shift model in src/physics.rs

**Checkpoint**: The existing ball remains a one-kilogram gravity-driven rigid
body, but has an explicit neutral mass-property foundation and no player control.

---

## Phase 3: User Story 1 - Shift a Visible Internal Mass (Priority: P1) MVP

**Goal**: Keyboard input changes a visible, bounded conceptual inner mass in
stable world directions, including diagonals and documented return behaviour.

**Independent Test**: Run cargo run, hold each W/A/S/D direction and at least
four diagonals, then release. The current mass marker follows the documented
target inside its valid region while the outer ball receives no direct movement.

### Tests for User Story 1

- [X] T004 [US1] Write failing deterministic tests for W/A/S/D mapping, diagonal normalisation, disk clamping, and move-toward-without-overshoot in src/mass_shift.rs

### Implementation for User Story 1

- [X] T005 [US1] Implement MassShiftTuning, InternalMassState, target intent mapping, disk clamping, and neutral return behaviour in src/mass_shift.rs
- [X] T006 [US1] Read keyboard intent and move current inner-mass state toward its requested world-relative target without touching PlayerBall motion in src/mass_shift.rs
- [X] T007 [US1] Spawn and update a world-stable current-mass development marker inside the visual ball region in src/scene.rs
- [X] T008 [US1] Register the mass-shift state and ordinary frame input/update systems in src/main.rs
- [X] T009 [US1] Manually validate cardinal, diagonal, clamp, and release behaviour using docs/specs/20260919-161207-first-mass-shift/contracts/desktop-mass-shift.md

**Checkpoint**: One visible conceptual mass can be moved predictably, but it has
not yet been coupled to change outer-ball physics.

---

## Phase 4: User Story 2 - Influence the Ball Through Balance (Priority: P2)

**Goal**: Couple the visible internal mass to the existing ball solely through
its derived centre of mass before fixed physics, so grounded contact produces
the response.

**Independent Test**: From the same starting condition, compare no-input runs
with left/right and forward/backward holds. A/D yields directionally opposite
path influence in at least eight of ten paired trials, without direct steering.

### Tests for User Story 2

- [X] T010 [US2] Write failing deterministic tests for mass-ratio COM conversion and identity/quarter-turn world-to-local COM conversion in src/mass_shift.rs

### Implementation for User Story 2

- [X] T011 [US2] Derive world and inverse-rotated local centre-of-mass offsets from InternalMassState without writing velocity, rotation, force, torque, impulse, or traction values in src/mass_shift.rs
- [X] T012 [US2] Schedule the direct CenterOfMass update before Avian PhysicsSystems::Prepare in the fixed physics path in src/main.rs
- [X] T013 [US2] Manually run no-input, paired A/D, and W/S downhill trials and record physical response evidence in docs/mass-shift.md

**Checkpoint**: The player alters the ball only by changing a mass property; the
resulting physical response is available for honest evaluation.

---

## Phase 5: User Story 3 - Understand and Tune the Experiment (Priority: P3)

**Goal**: Make the internal state, limits, reference frame, controls, and
limitations obvious enough to tune and interpret the experiment.

**Independent Test**: While the ball rolls, identify current mass, target,
centre, boundary, and input direction within ten seconds; then find the same
frame, controls, values, and compromises in documentation.

- [X] T014 [US3] Add world-stable centre, target, boundary, and centre-to-mass development display elements without parenting them to shell rotation in src/scene.rs
- [X] T015 [US3] Document the selected COM approximation, controls, world frame, parameters, trial observations, and limitations in docs/mass-shift.md
- [X] T016 [P] [US3] Update prototype status, temporary controls, and experiment documentation links in README.md
- [X] T017 [US3] Manually validate display readability and documentation against docs/specs/20260919-161207-first-mass-shift/contracts/desktop-mass-shift.md

**Checkpoint**: A developer can see what they changed and understand the limits
of the result without mistaking it for final steering or final tuning.

---

## Phase 6: Polish and Cross-Cutting Completion

**Purpose**: Validate the complete bounded experiment, preserve quality, and
update the long-lived backlog from evidence.

- [X] T018 Run cargo check, cargo test, cargo fmt --all -- --check, and cargo clippy --all-targets --all-features -- -D warnings from the repository root defined by Cargo.toml
- [X] T019 Run the complete desktop contract and paired-trial validation in docs/specs/20260919-161207-first-mass-shift/quickstart.md
- [X] T020 Update only demonstrated Internal Mass Model, Centre-of-Mass Behaviour, Player Control, Physics Tuning, Debugging Tools, Near-Term, and Milestone C items in docs/roadmap.md
- [X] T021 Review src/main.rs, src/physics.rs, src/scene.rs, and src/mass_shift.rs for prohibited direct steering, non-goal systems, and speculative abstractions

---

## Dependencies and Execution Order

### Phase dependencies

- Setup has no dependencies.
- Foundational work depends on T001 and blocks user stories.
- US1 depends on foundational preparation and is the first usable mass display.
- US2 depends on the state and visual meaning established in US1.
- US3 depends on US1 state and uses US2's physical coupling result to explain it.
- Completion work depends on every desired story.

### User-story graph

~~~text
Setup → Foundational → US1 visible mass → US2 physical COM coupling → US3 explain and tune
                                                             └──────→ completion validation
~~~

### Parallel opportunities

- T016 can proceed with T015 because README.md and docs/mass-shift.md are
  separate files after controls and values are decided.
- After US1 is complete, documentation work in T015/T016 can proceed while
  T010 starts the US2 pure-calculation tests.
- Manual checks are distinct but later checks depend on their preceding story.

## Parallel Example: Coupling and Documentation

~~~text
After User Story 1:
Task: T010 Write COM conversion tests in src/mass_shift.rs
Task: T015 Document the selected experiment in docs/mass-shift.md
Task: T016 Update prototype guidance in README.md
~~~

## Implementation Strategy

### MVP first

1. Complete setup and foundational preparation.
2. Complete T004 through T009.
3. Stop and validate that the mass itself is visible, bounded, stable, and
   understandable before coupling it to physical response.

### Incremental delivery

1. Add fixed-step COM coupling only after the visible state is trusted.
2. Compare trials and record weak/unstable behaviour honestly rather than adding
   steering.
3. Add development display/documentation, then run quality and roadmap review.

## Format Validation

All 21 tasks use the required checkbox, sequential T identifier, applicable
parallel marker, user-story label for story work, and exact file path.
