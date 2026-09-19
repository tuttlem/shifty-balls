---

description: "Implementation tasks for the first physically simulated rolling ball"
---

# Tasks: First Physically Simulated Rolling Ball

**Input**: Design documents in docs/specs/20260919-150334-first-rolling-ball/.

**Tests**: No physics-solver unit tests are planned. There is no new
project-owned deterministic calculation needing a test. Use the workspace test
suite for ordinary Rust and manually validate the desktop contract.

**Organization**: Work is grouped by story so simulation, documentation, and
camera scope remain independently reviewable.

## Phase 1: Setup

**Purpose**: Add the single approved physics dependency without changing the
single-crate boundary.

- [X] T001 Add exact avian3d 0.7.0 while retaining Bevy 0.19.1 and no other new runtime dependency in Cargo.toml

---

## Phase 2: Foundational Physics Setup

**Purpose**: Establish shared physical conventions and direct application
wiring used by all stories.

**CRITICAL**: Complete this phase before user-story work.

- [X] T002 Create direct Avian setup, explicit negative-Y gravity, prototype constants, and project-owned markers in src/physics.rs
- [X] T003 Wire the physics module and Avian PhysicsPlugins into the Bevy app, without input or a generic framework, in src/main.rs

**Checkpoint**: A configured 3D physics runtime exists with no player movement
or speculative systems.

---

## Phase 3: User Story 1 - Observe a Natural Downhill Roll (Priority: P1) MVP

**Goal**: Launch one visibly rotating dynamic ball on a primitive static track
and observe gravity-driven rolling, collision, and a contained slope-to-run-out
transition without player input.

**Independent Test**: Follow the scene and motion portions of
docs/specs/20260919-150334-first-rolling-ball/contracts/desktop-observation.md:
cargo run shows one ball moving downhill, visibly rotating, and remaining on
the primitive track during its first traversal.

- [X] T004 [US1] Spawn the Dynamic PlayerBall with a 0.5-metre sphere collider, one-kilogram mass, rolling material, interpolation, and generated orientation marker in src/scene.rs
- [X] T005 [US1] Build matching visible Static cuboids and cuboid colliders for the start deck, positive-Z slope, overlapping run-out, retaining edges, and end-stop in src/scene.rs
- [X] T006 [US1] Replace the bootstrap plane/sphere startup with the physics test scene and readable lighting in src/main.rs
- [X] T007 [US1] Manually validate unattended gravity, visible rotation, stable contact, and containment using docs/specs/20260919-150334-first-rolling-ball/contracts/desktop-observation.md

**Checkpoint**: The core experiment works with no keyboard, mouse, gamepad,
steering, force, impulse, internal mass, or second ball.

---

## Phase 4: User Story 2 - Trust the Physics Foundation (Priority: P2)

**Goal**: Make the chosen approach, prototype values, coordinate conventions,
future-experiment boundary, and current no-control status clear to a developer.

**Independent Test**: Review docs/physics.md and README.md against
docs/specs/20260919-150334-first-rolling-ball/data-model.md. They identify
Avian, scale, axes, gravity, radius, mass, materials, and deferred
centre-of-mass control.

- [X] T008 [US2] Document the Avian decision, alternatives, world/physics conventions, values, debug policy, and deferred internal-mass investigation in docs/physics.md
- [X] T009 [P] [US2] Update current-prototype status and development commands without creating an architecture guide in README.md

**Checkpoint**: The physical system is understandable without code archaeology,
and no future control model is presented as delivered.

---

## Phase 5: User Story 3 - Read the Moving Experiment (Priority: P3)

**Goal**: Keep the uncontrolled ball and useful downhill space observable with
an upright, deliberately minimal camera.

**Independent Test**: Run cargo run and verify that the view keeps the ball
visible through its initial traversal while the orientation marker rotates
independently, per
docs/specs/20260919-150334-first-rolling-ball/contracts/desktop-observation.md.

- [X] T010 [P] [US3] Implement an upright translation-only follow camera with fixed offset, positive-Z look-ahead, and simple smoothing in src/camera.rs
- [X] T011 [US3] Register camera startup and follow systems without velocity, FOV, shake, orbit, or ball-rotation behaviour in src/main.rs
- [X] T012 [US3] Manually validate framing and non-inheritance of ball rotation using docs/specs/20260919-150334-first-rolling-ball/contracts/desktop-observation.md

**Checkpoint**: The experiment is readable, but the camera is not a gameplay
camera framework.

---

## Phase 6: Polish and Cross-Cutting Completion

**Purpose**: Verify the bounded feature and update long-lived project state only
from evidence.

- [X] T013 Run cargo check, cargo test, cargo fmt --all -- --check, and cargo clippy --all-targets --all-features -- -D warnings from the repository root defined by Cargo.toml
- [X] T014 Run end-to-end desktop validation and clean-exit check in docs/specs/20260919-150334-first-rolling-ball/quickstart.md
- [X] T015 Update only demonstrated Physics Technology, World and Coordinate Conventions, First Rolling Ball, minimal Camera, Near-Term, and Milestone B checkboxes in docs/roadmap.md
- [X] T016 Review src/main.rs, src/physics.rs, src/scene.rs, and src/camera.rs against non-goals and remove accidental controls, hidden movement, extra balls, race systems, or speculative abstractions

---

## Dependencies and Execution Order

### Phase dependencies

- Phase 1 has no dependencies.
- Phase 2 depends on T001 and blocks every user story.
- US1 depends on T002–T003 and is the MVP.
- US2 depends on the concrete dependency and constants in T001–T002; it does
  not need the camera.
- US3 depends on the PlayerBall marker and transforms from US1.
- Phase 6 depends on all desired stories.

### User-story graph

~~~text
Setup → Foundational physics → US1 (natural roll) → US3 (observation camera)
                         └──→ US2 (documented foundation)
US1 + US2 + US3 → completion validation and roadmap
~~~

### Parallel opportunities

- T009 may run in parallel with T008 because it changes README.md rather than
  docs/physics.md after the phase-two decision is known.
- After US1 creates PlayerBall, T010 can proceed while outstanding US2
  documentation proceeds because it changes src/camera.rs.
- T007 and T012 are distinct manual checks, though T012 requires T011.

## Parallel Example: Documentation and Camera Work

~~~text
Task: T008 Document conventions and decision in docs/physics.md
Task: T009 Update prototype guidance in README.md

After US1:
Task: T010 Implement observation camera in src/camera.rs
Task: T008/T009 documentation work if it remains incomplete
~~~

## Implementation Strategy

### MVP first

1. Complete setup and foundational physics.
2. Complete T004–T007 for US1.
3. Stop and validate the unattended slope-to-run-out traversal.

At that checkpoint the project can assess its physical baseline. Do not add
controls to make the observation more exciting.

### Incremental delivery

1. Add US2 documentation once constants and the dependency are real.
2. Add the smallest camera only after the ball trajectory shows static framing
   is inadequate.
3. Complete quality checks, contract/quickstart observation, roadmap, and
   non-goal review before declaring the feature complete.

## Format Validation

All 16 implementation tasks use the required checkbox, sequential T identifier,
applicable parallel marker, user-story label for story work, and exact file
path.
