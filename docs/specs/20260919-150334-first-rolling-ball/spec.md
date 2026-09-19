# Feature Specification: First Physically Simulated Rolling Ball

**Feature Branch**: feature/first-rolling-ball

**Created**: 2026-09-19

**Status**: Draft

**Input**: Create the first physics gameplay milestone: a visible ball that gravity rolls down a
primitive test track, without any player control or internal movable mass.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Observe a Natural Downhill Roll (Priority: P1)

As a developer evaluating the first gameplay-physics milestone, I can launch Shifty Balls and
watch a ball rest on a primitive downhill test track, accelerate under gravity, rotate as it rolls,
collide with the track, and reach a safe run-out area.

**Why this priority**: This is the smallest tangible proof that the project has useful physical
behaviour. It establishes the substrate on which the future internal-mass experiment will depend.

**Independent Test**: Launch the application and observe one unattended ball progress from its
starting area down the slope into the receiving geometry without falling through the track or
requiring player input.

**Acceptance Scenarios**:

1. **Given** the application launches into the physics test environment, **When** the simulation
   begins, **Then** the ball responds to gravity without any player action.
2. **Given** the ball is on the incline, **When** gravity accelerates it downhill, **Then** it
   visibly rotates and remains in contact with the primitive track geometry.
3. **Given** the ball reaches a transition or run-out area, **When** it collides with the track,
   **Then** it remains within the test environment rather than falling forever or passing through
   geometry.

---

### User Story 2 - Trust the Physics Foundation (Priority: P2)

As a developer planning later ball mechanics, I can understand the selected initial physics
approach, its intended world conventions, and why it supports the immediate experiment without
precommitting the future internal-mass implementation.

**Why this priority**: Physics is central to Shifty Balls. A documented, bounded decision prevents
the first rolling experiment from becoming an accidental architecture choice.

**Independent Test**: Review the project documentation and confirm it records the selected approach,
world scale and axes, gravity, ball size, and the limits of this feature.

**Acceptance Scenarios**:

1. **Given** the project documentation, **When** a developer reviews the physics decision, **Then**
   they can see concise reasoning against the immediate and anticipated rigid-body requirements.
2. **Given** the documented world conventions, **When** rendering and physical values are reviewed,
   **Then** they use the same stated scale and vertical direction.

---

### User Story 3 - Read the Moving Experiment (Priority: P3)

As a developer observing the rolling ball, I can keep the ball and useful downhill space in view
without the view tumbling with the ball's physical rotation.

**Why this priority**: The experiment is only useful if its motion, contact, and transition between
track pieces are readable.

**Independent Test**: Launch the physics test environment and observe the entire initial downhill
movement from a stable, readable view.

**Acceptance Scenarios**:

1. **Given** the ball moves beyond the original static framing, **When** the camera follows it,
   **Then** the ball remains visible with useful space ahead.
2. **Given** the ball rotates physically, **When** the camera observes it, **Then** the camera
   does not inherit that rotation.

### Edge Cases

- If the ball starts with minimal movement, gravity still creates an observable downhill response.
- If the ball contacts a slope-to-run-out transition, the collision remains stable enough to retain
  the ball in the test environment.
- If the ball loses most of its energy in the run-out area, it may slow or settle naturally rather
  than receiving hidden motion assistance.
- If physics debug visualisation is enabled for development, it is clearly optional and can be
  disabled without changing gameplay behaviour.
- If the chosen approach cannot directly model the future internal-mass idea, documentation states
  the limitation instead of introducing a premature workaround.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The project MUST make and document one deliberate initial rigid-body physics approach
  suitable for dynamic spheres, gravity, static collision, friction, restitution, angular motion,
  forces or impulses, future ball-to-ball collision, development visibility where practical, and
  maintainability.
- **FR-002**: The decision documentation MUST consider how the chosen approach can support or be
  extended for the future internal movable-mass or centre-of-mass experiment, without implementing
  that mechanic now.
- **FR-003**: The project MUST document the minimum world conventions used by this experiment:
  vertical axis, downhill/forward convention, world scale, gravity direction and initial magnitude,
  initial ball radius, and the rendering-to-physics scale relationship.
- **FR-004**: The initial scene MUST contain one visible dynamic ball with an explicit initial mass
  or density and an appropriate spherical physical boundary.
- **FR-005**: Gravity MUST cause the unattended ball to accelerate downhill from its initial
  starting area; no player input, steering, propulsion, arbitrary acceleration, or hidden movement
  force may contribute to that movement.
- **FR-006**: The ball MUST physically collide with static primitive test-track geometry and its
  visible presentation MUST follow its physical position and rotation.
- **FR-007**: The test environment MUST include a starting area, an inclined/downhill surface, and
  receiving or run-out geometry sufficient to prevent the initial ball from immediately falling
  forever after the slope.
- **FR-008**: The test geometry MAY include one simple bank, retaining edge, shallow channel, or
  curved section only when it materially helps evaluate rolling; it MUST NOT become a production
  track system.
- **FR-009**: Initial prototype values for gravity, ball mass or density, friction, restitution,
  and any necessary linear or angular damping MUST be explicit and documented.
- **FR-010**: The initial physical behaviour MUST permit observation of gravity-driven
  acceleration, frictional rolling, natural angular rotation, sliding versus rolling where
  applicable, transition between primitive track pieces, and slowing or rest when the physical
  situation permits.
- **FR-011**: The ball presentation MUST contain enough simple visual orientation detail for a
  developer to observe physical rotation; it MUST NOT add production art assets.
- **FR-012**: The camera MUST provide a stable, readable view of the moving experiment. A simple
  follow behaviour MAY be added only if the static camera cannot keep the ball and useful downhill
  space visible; it MUST NOT inherit ball rotation or implement final-camera effects.
- **FR-013**: Physics debug visualisation MAY be included only when the selected approach provides
  it cheaply and it materially helps developers inspect ball and track collision geometry. It MUST
  be clearly development-oriented and easy to disable.
- **FR-014**: The implementation MUST remain direct and small. It MAY separate physics setup, ball
  spawning, primitive-track construction, and a minimal camera concern when this improves clarity;
  it MUST NOT introduce a general physics engine, production track framework, generic game
  architecture, or abstractions for hypothetical racers, AI, networking, or ball types.
- **FR-015**: The feature MUST NOT add an internal movable mass, centre-of-mass control, keyboard,
  mouse, gamepad, or conventional steering, multiple balls, opponents, race infrastructure,
  procedural tracks, production graphics, sound, menus, or surface-material gameplay systems.
- **FR-016**: Focused automated checks MAY cover deterministic project-owned calculations or
  important invariants, but the feature MUST NOT create brittle tests that duplicate third-party
  physics-engine behaviour.
- **FR-017**: Workspace compilation, relevant tests, formatting validation, and Clippy validation
  MUST pass without unjustified warnings or disabled checks.
- **FR-018**: Documentation and the roadmap MUST be updated accurately. Only delivered Physics
  Technology, World and Coordinate Conventions, First Rolling Ball, minimal Camera, Near-Term, and
  Milestone B items may be checked off.

### Key Entities *(include if feature involves data)*

- **Physics configuration**: The documented initial physical values and selected rigid-body approach
  governing the experiment.
- **Simulated player ball**: One visible, dynamic sphere with physical mass, a spherical collision
  boundary, and no player-controlled behaviour.
- **Primitive test track**: Static starting, slope, and receiving geometry created solely to
  evaluate the ball's physical motion.
- **Observation camera**: A stable view that frames the moving experiment without inheriting ball
  rotation.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 100% of supported-desktop manual launches, one unattended ball visibly begins
  moving downhill under gravity from the initial test position within five seconds of simulation
  start.
- **SC-002**: In 100% of supported-desktop manual launches, the ball remains visibly colliding with
  the initial slope and run-out geometry and does not pass through or immediately leave the test
  environment during its first downhill traversal.
- **SC-003**: A developer can visibly distinguish at least one change in the ball's orientation
  during the initial roll, demonstrating physical rotation rather than pure translation.
- **SC-004**: In 100% of normal launch checks, the camera keeps the ball and a useful portion of the
  nearby downhill path visible throughout the initial traversal without rotating with the ball.
- **SC-005**: Project documentation states one chosen physics approach, the immediate evaluation
  rationale, all minimum world conventions, and the explicit absence of player control and
  internal-mass behaviour.
- **SC-006**: Dependency and source review finds zero player-steering, propulsion, internal-mass,
  race, multiple-ball, or production-track features in this milestone.
- **SC-007**: Compilation, relevant tests, formatting validation, and lint validation complete
  successfully, and each checked roadmap item has direct evidence in the completed feature.

## Assumptions

- The existing single visible sphere and generated scene primitives are replaced or extended rather
  than preserved as a separate nonphysical duplicate ball.
- One physical ball and static primitive track geometry are sufficient to evaluate this milestone;
  no player interaction is necessary.
- Physics-library selection will be researched during planning against current compatible releases,
  not repeated as a rendering-engine evaluation.
- The initial camera may remain fixed if it keeps the entire experiment readable; a minimal follow
  view is allowed only if observation proves otherwise.
- Prototype values favour understandable ordinary physical behaviour over arcade exaggeration and
  are expected to change during later control-model experiments.
- Manual visual validation on a supported desktop is appropriate for the physical motion and camera
  outcomes; automated tests focus only on project-owned deterministic logic and useful invariants.
