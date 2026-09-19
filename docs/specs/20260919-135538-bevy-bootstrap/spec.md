# Feature Specification: Project Foundation and Bevy Bootstrap

**Feature Branch**: `master`

**Created**: 2026-09-19

**Status**: Draft

**Input**: User description: "Create the initial project foundation and a minimal, non-physics
Shifty Balls application that opens a readable 3D scene containing a ground surface and ball."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Launch the Visual Foundation (Priority: P1)

As a developer evaluating Shifty Balls, I can run the project and see a desktop window with a
readable reference surface and a visible ball, confirming that the project is ready for the first
physics experiment.

**Why this priority**: This is the smallest visible proof that the project foundation and rendering
path work. No later rolling-ball experiment can begin without it.

**Independent Test**: From a clean checkout with the documented prerequisites, run the documented
launch command and confirm a window titled “Shifty Balls” stays responsive until it is closed.

**Acceptance Scenarios**:

1. **Given** a developer has cloned the repository and installed the documented supported toolchain,
   **When** they run `cargo run`, **Then** a desktop window titled “Shifty Balls” opens.
2. **Given** the application window is open, **When** the developer views the initial scene,
   **Then** they can clearly see a flat reference surface and one sphere representing the future
   player ball.
3. **Given** the application is running, **When** the developer closes its window using normal
   desktop controls, **Then** the process exits without an error or forced termination.

---

### User Story 2 - Prepare a Trustworthy Local Project (Priority: P2)

As a developer joining the project, I can understand the project’s current purpose and use the
documented commands to build, test, format-check, and lint it.

**Why this priority**: A small foundation remains useful only if it is repeatable and easy to
validate before physics experimentation increases complexity.

**Independent Test**: Follow the root README from a clean checkout and run every documented
development command successfully.

**Acceptance Scenarios**:

1. **Given** a clean checkout, **When** a developer follows the README prerequisites and build/run
   instructions, **Then** they can build and launch the application without undocumented setup.
2. **Given** the project checkout, **When** a developer runs the documented validation commands,
   **Then** compilation, tests, formatting validation, and lint validation complete successfully.

---

### User Story 3 - Keep the Physics Experiment Uncommitted (Priority: P3)

As a developer planning the next feature, I can rely on this foundation without inheriting a
premature physics choice, gameplay controls, or speculative architecture.

**Why this priority**: The project’s core question is whether mass shifting is fun; a bootstrap
must not constrain the deliberate physics decision that follows.

**Independent Test**: Review the dependency list and running application to confirm that the ball
is stationary visual geometry and no gameplay input or physics behaviour exists.

**Acceptance Scenarios**:

1. **Given** the visual scene is running, **When** it is observed without interaction, **Then** the
   ball remains a visible presentation object with no gravity, rolling, collision, velocity, or
   internal-mass behaviour.
2. **Given** the completed project dependencies and source, **When** they are reviewed, **Then** no
   physics middleware, gameplay input, or speculative game architecture has been introduced.

### Edge Cases

- If the documented toolchain or platform prerequisites are unavailable, the README identifies the
  prerequisite rather than implying that the project can run without it.
- If the window cannot be created because the host lacks a usable desktop graphics environment, the
  normal process failure is clear and does not leave background processes running.
- If the developer closes the window before inspecting the scene, the application still performs a
  normal clean shutdown.
- The foundation must remain useful even when there is no gameplay logic substantial enough to
  justify automated logic tests.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The repository MUST provide a valid, minimal Rust project or workspace whose structure
  is justified by current bootstrap requirements; it MUST NOT add crates for anticipated features.
- **FR-002**: The project MUST use Bevy as its selected application technology and MUST NOT conduct
  a game-engine or rendering-stack evaluation.
- **FR-003**: The project MUST record an appropriate stable Bevy dependency compatible with the
  documented supported Rust toolchain and MUST include only the functionality needed by this feature.
- **FR-004**: Running `cargo run` from the repository root MUST launch an application titled
  “Shifty Balls” in a desktop window.
- **FR-005**: The initial application scene MUST present a fixed perspective view containing
  sufficient lighting, one flat ground/reference surface, and one visible sphere representing the
  future player ball.
- **FR-006**: The application MUST remain responsive during normal desktop operation and exit
  cleanly when the window is closed.
- **FR-007**: The sphere and reference surface MUST be presentation geometry only. This feature MUST
  NOT introduce gravity, rolling, collision, friction, restitution, velocity, angular velocity,
  centre-of-mass behaviour, an internal mass, player movement, or player control.
- **FR-008**: This feature MUST NOT add a physics library or select a physics approach. That decision
  belongs to the next coherent specification with concrete rolling-ball requirements.
- **FR-009**: This feature MUST NOT add gameplay camera behaviour, gameplay input, external art
  assets, track geometry or authoring systems, multiple balls, race systems, HUD, menus, audio,
  networking, AI, progression, or any other non-bootstrap feature.
- **FR-010**: The implementation MUST remain direct and readable and MUST NOT introduce generic
  engine, renderer, physics, entity, controller, service, dependency-injection, event-bus, or
  plugin architecture for hypothetical future extensibility.
- **FR-011**: The root README MUST concisely describe Shifty Balls and its current prototype status,
  prerequisites, supported toolchain policy, build/run/test/format/lint commands, and the locations
  of the roadmap and feature specifications.
- **FR-012**: The repository MUST include appropriate ignore rules and documented, standard
  project-wide commands for compilation, tests, formatting validation, and lint validation.
- **FR-013**: The project MUST establish the testing convention that future deterministic gameplay
  calculations receive unit tests where practical and useful gameplay bugs receive regression tests;
  this feature MUST NOT add visual-comparison, screenshot, GPU-rendering, or headless-rendering
  infrastructure.
- **FR-014**: The project MUST preserve SpecKit infrastructure and timestamped specification naming
  in the form `docs/specs/YYYYMMDD-HHMMSS-feature-name/`; its SpecKit feature-numbering setting
  MUST use timestamps rather than sequential numbers.
- **FR-015**: During implementation, the project roadmap MUST be maintained at the constitutional
  canonical path `docs/roadmap.md`. The existing `docs/ROADMAP.md` must be normalized to that
  path without losing its content, and only genuinely satisfied Project Foundation, Bevy Bootstrap,
  Near-Term Milestone, and Milestone A items may be checked off.
- **FR-016**: CI MUST be evaluated during implementation but MUST remain unimplemented and its
  roadmap items unchecked unless it offers meaningful value for this recreational, single-developer
  prototype.
- **FR-017**: Completion MUST leave the project compiling successfully, relevant tests passing,
  formatting validation passing, Clippy passing without unjustified warnings, documentation
  accurate, and no validation checks disabled merely to satisfy acceptance criteria.

### Key Entities *(include if feature involves data)*

- **Application session**: One launched desktop session that owns the initial window and its normal
  lifecycle.
- **Reference scene**: The fixed, readable visual context consisting of a ground surface, lighting,
  perspective view, and future-player-ball sphere.
- **Future player ball**: A visible sphere used solely to establish player-ball presentation; it has
  no physical or controllable state in this feature.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A developer following the README from a clean checkout can reach a running Shifty
  Balls window using one documented launch command, `cargo run`, with no undocumented project
  setup steps.
- **SC-002**: In 100% of normal manual launch checks on a supported desktop environment, the initial
  view visibly contains exactly one ground/reference surface and one ball sphere, both readable at
  first glance.
- **SC-003**: In 100% of normal manual shutdown checks, closing the application window ends the
  application without forced termination or an error exit.
- **SC-004**: All four documented validation commands—compilation, tests, formatting validation, and
  lint validation—finish successfully in a clean checkout.
- **SC-005**: Dependency and source review finds zero physics libraries, gameplay controls, dynamic
  ball behaviours, or speculative subsystem abstractions in the completed bootstrap.
- **SC-006**: All roadmap items marked complete by this feature have direct evidence in the
  repository or documented validation results; physics and later-milestone items remain unchecked.

## Assumptions

- The first supported environment is a developer desktop with a functioning graphics stack capable
  of opening a native application window; headless and web targets are out of scope.
- One application crate is the minimum justified structure unless implementation reveals a concrete
  need for another boundary.
- Primitive/generated geometry and simple materials are sufficient; no external art assets are
  needed.
- The currently committed roadmap is authoritative despite its uppercase filename; implementation
  will normalize it to the constitution’s canonical lowercase path as part of documentation
  maintenance.
- The existing 2024 Rust edition remains appropriate unless the selected stable Bevy version’s
  documented compatibility requires an explicitly justified change.
- No new automated graphics-test infrastructure is warranted because the only acceptance requiring
  visual inspection is intentionally small and manually verifiable.
- This feature stops after **window → ground → ball**. The next specification will deliberately
  choose a physics approach and introduce gravity only after this foundation is complete.
