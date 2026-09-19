# Feature Specification: Control Model Comparison

**Feature Branch**: `control-model-comparison`  
**Created**: 2026-09-19  
**Status**: Draft  
**Input**: User description: "Compare the established internal-mass control with torque-driven and force-driven human controls without removing the existing First Race vertical slice."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Compare a control model on the course (Priority: P1)

As a player evaluating Shifty Balls, I can choose SHIFT, TORQUE, or FORCE before
an attempt and drive the existing prototype course with that one selected model,
so I can experience the differences directly rather than infer them from a
technical description.

**Why this priority**: Direct, repeatable hands-on comparison is the purpose of
this milestone. Without it, the project cannot make an evidence-based control
decision.

**Independent Test**: Start a fresh single-player course attempt in each of the
three modes, use the same directional inputs, and observe a distinct physical
response while the ball remains governed by normal gravity and collision.

**Acceptance Scenarios**:

1. **Given** the prototype course is ready for an attempt, **When** the player
   selects SHIFT, **Then** directional input moves the existing conceptual
   internal mass and the ball responds through its changed balance.
2. **Given** the prototype course is ready for an attempt, **When** the player
   selects TORQUE, **Then** directional input produces rotational intent on the
   ball without setting a desired path or velocity.
3. **Given** the prototype course is ready for an attempt, **When** the player
   selects FORCE, **Then** directional input produces translational physical
   intent without setting a desired path or velocity.
4. **Given** a player changes model or restarts an attempt, **When** the new
   attempt begins, **Then** the ball starts from the same defined course state
   and no effect from the previous model remains.

---

### User Story 2 - Make a fair, observable comparison (Priority: P2)

As a playtester, I can identify the active model and review comparable attempt
results and development information, so I can judge predictability, recovery,
momentum, control authority, and enjoyment rather than confuse one mode with
another.

**Why this priority**: A three-mode prototype only answers the design question
if its modes, inputs, starting conditions, and observations are clear enough to
compare honestly.

**Independent Test**: Complete or restart repeated attempts in each mode using
the same course and inspect the active-mode indication, session timing, and
mode-appropriate development display.

**Acceptance Scenarios**:

1. **Given** an active comparison attempt, **When** the player views the
   development-facing readout, **Then** it identifies the active control model
   and exposes enough model-specific state to relate input to physical response.
2. **Given** a completed or abandoned attempt, **When** the player begins
   another attempt in a different model, **Then** the normalised starting state
   and timing loop allow a meaningful comparison rather than carrying momentum
   or hidden state across modes.
3. **Given** the comparison has been playtested, **When** a developer reviews
   the project documentation, **Then** it records the procedure, tuning values,
   observed strengths and weaknesses, and an honest provisional preference or
   unresolved result for all three models.

---

### User Story 3 - Preserve the established race (Priority: P3)

As a player returning to First Race, I can still start, race, finish, and
rematch against the existing opponents, so an experiment for human control does
not remove or silently weaken the already accepted racing vertical slice.

**Why this priority**: The comparison is intentionally human-focused. It must
not require rewriting the AI controller three times or discard the evidence
already gained from physical multi-ball racing.

**Independent Test**: Launch the existing race path, complete the countdown,
race physical opponents, receive a result, and rematch using its established
behaviour.

**Acceptance Scenarios**:

1. **Given** the race path is selected, **When** a race starts, **Then** the
   existing opponent field, countdown, progression, positions, finish order,
   result, and rematch loop remain available.
2. **Given** the control comparison is being implemented, **When** opponents
   race, **Then** they are not required to support TORQUE or FORCE for this
   milestone and their established control behaviour remains intact.

### Edge Cases

- What happens when a player changes the selected model during a moving
  attempt? The game must require a reset or otherwise normalise the attempt;
  modes must not be mixed in one timed comparison run.
- What happens when no directional input is held? Each model must have a
  documented neutral behaviour and must not retain unexplained propulsion.
- What happens when the ball is airborne, colliding, resting, or moving
  backwards? The selected model must remain physically comprehensible and any
  intentionally limited behaviour must be visible in documentation.
- What happens when a comparison implementation is unpleasant or too weak to
  evaluate? Record that finding honestly; do not add hidden vehicle steering to
  manufacture a positive result.
- What happens when control selection is unavailable during a normal race? The
  race must remain usable with its existing human and AI control arrangement;
  the comparison environment must not make race progression invalid.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The project MUST retain the existing First Race functionality,
  including multiple physical racers, opponents, collisions, race lifecycle,
  progression, positions, results, and rematch.
- **FR-002**: The project MUST provide a controlled, repeatable single-player
  use of the existing prototype course for comparing the three named human
  control models: SHIFT, TORQUE, and FORCE.
- **FR-003**: The player MUST be able to select the active comparison model
  before a fresh attempt using clearly documented temporary controls or an
  equivalently direct development-facing interaction.
- **FR-004**: SHIFT MUST preserve the existing causal model: directional intent
  changes the internal mass or centre-of-mass state, and the ball's response
  emerges from physical simulation.
- **FR-005**: TORQUE MUST map directional intent to rotational physical input;
  it MUST NOT set ball velocity, set a target trajectory, rotate the ball
  directly toward a requested direction, or add hidden steering assistance.
- **FR-006**: FORCE MUST map directional intent to translational physical
  input; it MUST NOT set ball velocity, set a target trajectory, teleport the
  ball, or add hidden steering assistance.
- **FR-007**: All three modes MUST use the same prototype course, camera
  purpose, ball baseline properties, restart behaviour, and timing rules unless
  a documented mode-specific physical necessity makes an exception essential to
  the comparison.
- **FR-008**: The project MUST establish and document a stable, understandable
  directional reference convention for all three modes. The convention MUST
  avoid making ordinary input depend on the ball's visible spin.
- **FR-009**: Each mode MUST define and document its neutral-input behaviour,
  applicable tuning values, and any intended limitations while grounded,
  airborne, resting, or in contact.
- **FR-010**: The player MUST be able to identify the currently active model
  during an attempt and inspect enough development-facing visualisation or
  telemetry to understand the relationship between input and physical response.
- **FR-011**: A restart and model change MUST restore a comparable initial
  attempt state, including relevant linear and angular movement, model state,
  timer state, and camera state.
- **FR-012**: The prototype MUST continue to use ordinary gravity, track
  collision, rolling, and momentum; it MUST NOT add artificial speed boosts,
  direct path correction, automatic traction steering, or velocity caps merely
  to favour a control mode.
- **FR-013**: The project MUST keep important model tuning values focused,
  visible, and quick to adjust without creating a general configuration system.
- **FR-014**: The project MUST add focused deterministic coverage for pure
  comparison logic where practical, such as selection, reset normalisation,
  input conversion, clamping, and neutral-state calculations. It MUST NOT
  duplicate whole third-party physics simulations in brittle tests.
- **FR-015**: Documentation MUST describe the comparison environment, controls,
  reference frame, tuning, repeatable playtest procedure, constraints, and
  observations for SHIFT, TORQUE, and FORCE.
- **FR-016**: Documentation MUST record a provisional preferred model or
  explicitly state that evidence remains inconclusive. A preference MUST be
  based on playtest observations, not presumed from the original design.
- **FR-017**: `docs/roadmap.md` MUST record the actual development sequence as
  **First Roll → First Shift → First Course → First Race → Control Model
  Comparison**, and identify this work as a post-First-Race design iteration.
- **FR-018**: The project MUST preserve its established build, test, formatting,
  lint, documentation, and roadmap quality requirements.

### Key Entities

- **Control model**: One named way a human player's directional intent changes
  a ball's physical state: SHIFT, TORQUE, or FORCE.
- **Comparison attempt**: A time-trial-style traversal of the existing
  prototype course under exactly one active control model and a normalised
  starting state.
- **Control intent**: Directional player request interpreted in the documented
  stable reference frame before the selected model responds.
- **Model tuning**: Small, explicit values that determine each model's control
  authority, response, neutral behaviour, and safe experimental limits.
- **Comparison observation**: A recorded playtest finding about predictability,
  momentum, recovery, physical readability, satisfaction, or limitations of a
  particular model.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A playtester can start a fresh attempt in each of SHIFT, TORQUE,
  and FORCE in no more than two direct interactions after launching the
  comparison environment.
- **SC-002**: In three repeated attempts per model from the standard start, a
  playtester can identify the active mode from the development-facing display
  before providing movement input in 9 of 9 attempts.
- **SC-003**: For each model, a playtester can intentionally demonstrate at
  least one meaningful directional change or momentum-affecting action on the
  prototype course without direct velocity or trajectory control.
- **SC-004**: The same documented start, course, timing, and restart procedure
  is usable for all three models, allowing an observer to compare attempts
  without a different course or race configuration.
- **SC-005**: A normal First Race can still be started, completed, and restarted
  with its existing four-racer lifecycle after the comparison feature is added.
- **SC-006**: The comparison record contains observations for all three models
  and identifies either a provisional preferred model with reasons or a clear
  unresolved decision and the evidence still needed.
- **SC-007**: All established automated quality commands complete successfully
  with no unjustified warnings or disabled checks.

## Assumptions

- The existing hand-authored prototype course and time-trial-style attempt loop
  are the controlled comparison environment; this feature does not create a
  second course or general game-mode menu.
- Temporary keyboard controls are sufficient for selecting and testing models;
  gamepad support, rebinding, and production UI remain out of scope.
- The existing First Race path remains the regression target and may continue
  to use SHIFT for the human and opponents during this experiment. Adapting AI
  to a selected winner is deferred to a later specification.
- The same ball size, gravity, course geometry, visual presentation, and camera
  baseline provide the fairest useful comparison. Mode-specific strength or
  response tuning is allowed only when documented as an experimental parameter.
- The physical models are evaluated for gameplay value and learnability, not
  scientific simulation fidelity. A model may be rejected if it is unstable,
  opaque, weak, or simply less enjoyable.
- This specification is an experiment discovered after First Race. It does not
  claim that any original milestone was invalid or that the comparison happened
  earlier in project history.

## Out of Scope

- Replacing or removing the existing First Race systems.
- Rewriting AI opponents to support all three control models.
- Adding conventional vehicle steering, direct velocity assignment, target-path
  snapping, automatic traction steering, or hidden corrective forces.
- New race modes, opponents, tracks, track construction systems, laps,
  multiplayer, networking, race setup menus, progression, audio, production
  UI, production art, or persistent leaderboards.
- Declaring the long-term control model permanently settled without documented
  playtest evidence.
