---

description: "Task list for Project Foundation and Bevy Bootstrap"
---

# Tasks: Project Foundation and Bevy Bootstrap

**Input**: Design documents from docs/specs/20260919-135538-bevy-bootstrap/

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/desktop-scene.md, and
quickstart.md

**Tests**: No new automated test is required for this static visual bootstrap. Run cargo test as a
quality gate and document the future convention for ordinary deterministic gameplay logic. Manual
desktop-scene validation is the proportionate acceptance test.

**Organization**: Tasks are grouped by user story so each increment can be independently checked.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the minimal dependency, workspace, and roadmap foundation shared by all
stories.

- [X] T001 Update Cargo.toml with the root single-member workspace, Rust 1.95.0 minimum, and only the Bevy 0.19.1 default-feature dependency.
- [X] T002 [P] Rename docs/ROADMAP.md to docs/roadmap.md without changing or losing roadmap content.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Resolve and validate the selected dependency before feature work uses it.

**⚠️ CRITICAL**: Install or select stable Rust 1.95.0 or newer, then refresh Cargo.lock with Cargo.toml and confirm no dependency beyond Bevy is added.

- [X] T003 Run cargo check after T001 with Rust 1.95.0 or newer and update Cargo.lock with the resolved Bevy 0.19.1 dependency graph.

**Checkpoint**: The direct single-crate workspace and its deliberate dependency are ready; user
story work may begin.

---

## Phase 3: User Story 1 - Launch the Visual Foundation (Priority: P1) 🎯 MVP

**Goal**: A developer can launch a responsive Shifty Balls desktop window and immediately see a
readable, static ground plane and future-player-ball sphere.

**Independent Test**: Run cargo run, verify the exact window title, fixed perspective view, lit
flat ground surface, one stationary sphere, normal responsiveness, and clean close as defined in
contracts/desktop-scene.md.

- [X] T004 [US1] Replace the placeholder program with a direct Bevy application in src/main.rs that configures the Shifty Balls window and default plugins.
- [X] T005 [US1] Add the fixed perspective camera, basic lighting, generated flat ground primitive, and one generated stationary ball sphere in src/main.rs.
- [X] T006 [US1] Manually validate launch, readable scene, responsiveness, and normal close against docs/specs/20260919-135538-bevy-bootstrap/contracts/desktop-scene.md.

**Checkpoint**: User Story 1 delivers the MVP: window → ground → ball, with no movement or
physics.

---

## Phase 4: User Story 2 - Prepare a Trustworthy Local Project (Priority: P2)

**Goal**: A developer can understand the prototype and reproduce its build, run, test, format, and
lint validation from the root documentation.

**Independent Test**: Starting from README.md, follow the prerequisites and run every listed
command successfully without undocumented setup.

- [X] T007 [US2] Create README.md with the project description, prototype status, Rust 1.95.0-or-newer desktop prerequisites, and the cargo build/run/test/format/lint commands.
- [X] T008 [US2] Add the canonical docs/roadmap.md and docs/specs/ locations, deterministic-gameplay testing convention, CI deferral decision, and no-physics bootstrap boundary to README.md.
- [X] T009 [US2] Execute every validation command documented in README.md and reconcile any command or prerequisite discrepancy in README.md.

**Checkpoint**: User Story 2 provides a repeatable local development workflow and accurately
documents the intentionally small foundation.

---

## Phase 5: User Story 3 - Keep the Physics Experiment Uncommitted (Priority: P3)

**Goal**: The completed foundation remains free of premature physics, controls, and abstraction
decisions, leaving the next specification free to choose the rolling-ball approach deliberately.

**Independent Test**: Review Cargo.toml and src/main.rs against the desktop-scene contract and
confirm that the ball is presentation-only with no physics library, input, movement, or future
architecture.

- [X] T010 [US3] Audit Cargo.toml and src/main.rs against docs/specs/20260919-135538-bevy-bootstrap/contracts/desktop-scene.md; remove any physics, input, gameplay-camera, asset, or speculative abstraction additions found.

**Checkpoint**: The project proves only the rendering path and leaves the physics/control decision
uncommitted.

---

## Phase 6: Polish and Completion

**Purpose**: Apply the constitution's completion gate and update only roadmap work proven complete.

- [X] T011 Update docs/roadmap.md to check only the Near-Term Milestone, Project Foundation, Bevy Bootstrap, testing-foundation, documentation, and CI-decision items evidenced by T001–T010; leave CI implementation and all physics/later work unchecked.
- [X] T012 Run cargo check, cargo test, cargo fmt --all -- --check, and cargo clippy --all-targets --all-features -- -D warnings from the repository root; resolve all unjustified failures and warnings in Cargo.toml, Cargo.lock, src/main.rs, README.md, and docs/roadmap.md.
- [X] T013 Re-run the complete manual acceptance sequence in docs/specs/20260919-135538-bevy-bootstrap/quickstart.md and record any discovered nonessential follow-up in docs/roadmap.md.

---

## Dependencies and Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: T001 and T002 can start immediately.
- **Foundational (Phase 2)**: T003 depends on T001 and blocks all implementation that requires
  Bevy.
- **User Story 1 (Phase 3)**: T004–T006 depend on T003.
- **User Story 2 (Phase 4)**: T007–T009 depend on T003 and use the runnable application from T004.
- **User Story 3 (Phase 5)**: T010 depends on T004–T005 and may run after User Story 1.
- **Polish (Phase 6)**: T011–T013 depend on all desired user-story tasks.

### User Story Dependencies

- **US1 (P1)**: Requires T003 only; it is the MVP.
- **US2 (P2)**: Requires the selected toolchain/dependency and the runnable application so its
  commands reflect reality.
- **US3 (P3)**: Requires the implemented static scene; it has no dependency on README work.

### Parallel Opportunities

- T001 and T002 can proceed in parallel because they modify Cargo.toml and docs/ROADMAP.md.
- After T003, T004 may begin while a developer drafts T007, provided T007 is reconciled after the
  runnable application exists.
- After T005, T010 can run in parallel with T006 or T007–T009 because it reviews Cargo.toml and
  src/main.rs rather than modifying documentation.

## Parallel Example: User Story 1

~~~text
After T005:
Task: "Manually validate the desktop scene in contracts/desktop-scene.md" (T006)
Task: "Audit Cargo.toml and src/main.rs for prohibited scope" (T010, US3)
~~~

## Implementation Strategy

### MVP First

1. Complete T001–T003 to establish the deliberate, compatible foundation.
2. Complete T004–T006.
3. Stop and validate the MVP manually: cargo run → titled window → lit ground → stationary ball.

### Incremental Delivery

1. Add T007–T009 so the runnable foundation is reproducible.
2. Complete T010 to preserve the physics/control decision for the next feature.
3. Complete T011–T013 before declaring the specification complete.

## Notes

- Every task uses the required checkbox, sequential ID, and exact file path format.
- No automated visual-test task is included because the specification explicitly excludes
  screenshot, GPU, and headless rendering infrastructure.
- CI is evaluated and documented as deferred; do not add CI merely to satisfy a checklist.
