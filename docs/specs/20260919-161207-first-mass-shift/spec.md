# Feature Specification: First Internal Mass Shift

**Feature Branch**: feature/first-shift

**Created**: 2026-09-19

**Status**: Draft

**Input**: Implement the first experimental Shifty Balls mechanic: a player shifts a visible internal mass and the outer rolling ball responds through an understandable physical cause rather than conventional steering.

## User Scenarios & Testing

### User Story 1 - Shift a Visible Internal Mass (Priority: P1)

As a developer evaluating the central control idea, I can use temporary keyboard directions to move a clearly visible internal mass within one rolling ball, including diagonal directions, so I understand what physical state my input changes.

**Why this priority**: The player must first be able to deliberately change an internal physical state before the project can evaluate whether it is a satisfying control mechanism.

**Independent Test**: Launch the existing environment, press and release each documented direction alone and in diagonal pairs, and confirm the visible mass moves to the matching constrained position and follows defined release behaviour.

**Acceptance Scenarios**:

1. **Given** the ball is running, **When** the player holds a documented left, right, forward, or backward direction, **Then** current and requested mass positions visibly move in the corresponding stable gameplay direction.
2. **Given** the player holds two compatible directions, **When** input is processed, **Then** the mass moves diagonally while staying in its permitted region.
3. **Given** input is released, **When** no direction remains, **Then** the mass follows documented resting or return behaviour without directly changing ball motion.

---

### User Story 2 - Influence the Ball Through Balance (Priority: P2)

As a developer, I can move the internal mass while the ball rolls and observe a useful physical change in the ball, so I can judge whether shifting weight rather than steering has potential.

**Why this priority**: This is the central causal test: mass moves, balance changes, and the ball responds. Without it, the visible mass is decoration.

**Independent Test**: Compare no-input downhill rolls with rolls holding the mass left, right, forward, and backward. The player can repeatedly produce a directionally related change without direct ball steering.

**Acceptance Scenarios**:

1. **Given** the ball rolls downhill, **When** the player holds left or right mass displacement, **Then** the path changes measurably in a repeatable direction relative to the chosen input frame.
2. **Given** the ball moves, **When** the player holds the mass forward or backward, **Then** the ball has an observable response worth comparing with no-input motion.
3. **Given** the experiment is inspected, **When** ball motion changes, **Then** it contains no direct velocity setting, desired-path snap, vehicle steering, arbitrary lateral steering force, or hidden traction assistance.

---

### User Story 3 - Understand and Tune the Experiment (Priority: P3)

As a developer, I can see the mass, its requested position, permitted region, and ball centre while experimenting, and can find the selected approach, controls, input frame, and tuning values in project documentation.

**Why this priority**: An honest experiment must make its cause visible and easy to tune. Otherwise weak results cannot be distinguished from opaque code.

**Independent Test**: Change directions while reviewing the development display and documentation. A developer can explain where the mass is, what it is trying to do, how input is interpreted, and which values can be adjusted.

**Acceptance Scenarios**:

1. **Given** the player changes mass direction, **When** the development view is visible, **Then** it shows outer-ball centre, current mass position, target or direction, and the displacement limit in a stable frame.
2. **Given** a developer reviews the documentation, **When** they plan another experiment, **Then** they find the model, alternatives, compromises, controls, reference frame, tuning values, and known questions.

### Edge Cases

- A diagonal request outside the valid region is clamped rather than allowing the mass to leave the ball.
- Rapid direction reversal follows documented mass transition behaviour and never teleports the ball or bypasses physical coupling.
- Nearly stationary input still changes the visible internal state, even when outer-ball movement is small.
- Physical ball rotation does not make input unintelligible because its initial gameplay reference frame is stable and documented.
- If behaviour is weak, unstable, or unintuitive, documentation records the result rather than adding disguised steering.

## Requirements

### Functional Requirements

- **FR-001**: The project MUST select and document one small, practical initial internal-mass representation after evaluating runtime centre-of-mass offset, compound representation, constrained internal body, displaced-mass torque, and simplified physically causal alternatives.
- **FR-002**: The decision MUST evaluate understandability, stability, controllability, rolling and slope behaviour, future airborne implications, tuning range, implementation complexity, and rapid experimental value.
- **FR-003**: The feature MUST represent one conceptual internal mass associated with the existing player ball.
- **FR-004**: The feature MUST centralise explicit initial values for outer-ball mass, internal mass, their ratio, maximum displacement, movement speed, default position, and any smoothing, inertia, or coupling.
- **FR-005**: The mass MUST remain within a defined physically sensible region inside the outer ball, including diagonal movement.
- **FR-006**: The feature MUST provide and document temporary keyboard controls for left, right, forward, backward, and combined diagonal requested mass displacement.
- **FR-007**: Input MUST specify desired internal-mass displacement, not desired ball velocity, rotation, path, or conventional steering direction.
- **FR-008**: The feature MUST select and document an initial stable input reference frame; rotating ball-local directions MUST NOT be the default without a documented readability reason.
- **FR-009**: The feature MUST clamp requested displacement and define its initial response when directional input is released.
- **FR-010**: Moving the mass MUST change meaningful physical state so the ball response follows the causal relationship: mass moves, balance changes, ball responds.
- **FR-011**: The feature MUST NOT directly set ball velocity, rotate the ball toward a desired direction, snap it onto a path, add vehicle steering, apply arbitrary lateral steering forces, or add hidden traction assistance.
- **FR-012**: In the primitive environment, holding left or right displacement MUST repeatedly influence the ball path in a direction related to the selected reference frame.
- **FR-013**: Forward and backward displacement MUST each produce observable behaviour worth comparing against a no-input roll.
- **FR-014**: Development visualisation MUST show the outer-ball centre, current mass position, and permitted displacement region or equally clear limit. It SHOULD expose requested position or direction when useful.
- **FR-015**: The display MUST remain understandable while the ball rolls and MUST NOT become confusing solely because the ball rotates.
- **FR-016**: The primitive environment and follow camera MUST be reused unless a small documented change is necessary to evaluate slope, lateral influence, or recovery.
- **FR-017**: The system MUST keep mass state, input interpretation, physical coupling, tuning, and development display small and replaceable without a generic player-controller, multi-ball, or track architecture.
- **FR-018**: Deterministic project-owned direction conversion, displacement clamping, and target movement calculations MUST have focused tests where they are introduced; third-party simulation behaviour MUST NOT be duplicated.
- **FR-019**: Documentation MUST state the approach, alternatives, approximations, reference frame, controls, tuning values, and limitations.
- **FR-020**: Existing rolling-ball physics MUST remain functional; relevant build, test, formatting, lint, documentation, and roadmap checks MUST pass.
- **FR-021**: The feature MUST NOT add a production track/course, multiple balls, racing, checkpoints, timing, AI, opponents, gamepad/rebinding, production UI/art/audio, surfaces, jumping or airborne tuning, rotational-inertia mechanics, multiple weights, gyroscopes, procedural tracks, menus, progression, or multiplayer.

### Key Entities

- **Internal mass state**: The conceptual mass, current and requested positions, default behaviour, displacement constraint, and tuning values.
- **Input intent**: The player-requested mass displacement in a stable gameplay reference frame.
- **Physical coupling**: The documented relationship turning displaced mass into a balance change for the outer ball.
- **Development mass display**: Temporary reference for ball centre, current mass, target or direction, and permitted region.
- **First-shift experiment record**: The selected model, controls, tuning, results, limitations, and next questions.

## Success Criteria

### Measurable Outcomes

- **SC-001**: In each supported-desktop manual validation, a developer moves the visible mass in four cardinal directions and at least four diagonal combinations, with every position inside the displayed permitted region.
- **SC-002**: In ten paired downhill trials from the same start, holding left versus holding right displacement produces visibly distinguishable and directionally opposite path influence in at least eight pairs.
- **SC-003**: In each manual validation, forward and backward displacement each have an observable response comparable with a no-input roll; documentation records whether the response is useful.
- **SC-004**: Within ten seconds while moving, a developer can identify current mass location, requested direction or target, ball centre, and displacement limit from the development display.
- **SC-005**: Source and dependency review finds zero direct velocity writes, desired-direction ball rotations, vehicle steering operations, arbitrary lateral steering forces, hidden traction assistance, or non-goal systems.
- **SC-006**: Workspace compilation, relevant automated tests, formatting, and lint pass; documentation names the approach, controls, frame, parameters, and honest limitations.
- **SC-007**: The existing gravity-driven no-input roll remains available as a comparison baseline.

## Assumptions

- Planning will decide the representation after inspecting the installed physics integration; this specification does not preselect an equation or architecture.
- Keyboard-only temporary controls are sufficient; mouse and gamepad are deferred.
- A stable world-relative or camera-relative horizontal frame is a reasonable initial candidate because rotating ball-local axes are not readable.
- Releasing input initially returns mass toward neutral unless the selected simple model needs another documented behaviour.
- Primitive development visualisation and small environment adjustments are sufficient; production HUD, course, and track tooling are unnecessary.
- Identifying an unsatisfying model is a useful experimental result when it is documented honestly without conventional steering.
