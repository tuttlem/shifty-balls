# Feature Specification: First Course Kill Test

**Feature Branch**: `feature/first-course`

**Created**: 2026-09-19

**Status**: Draft

**Input**: Create a compact hand-authored course that tests whether repeated,
physics-driven internal-mass control is genuinely enjoyable before any racing
infrastructure is introduced.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Complete a Deliberate Physical Course (Priority: P1)

As a player, I can start one compact downhill course and use mass shifting to
negotiate a forgiving opening, a gentle turn, a stronger bank, a height-versus-
speed opportunity, a momentum challenge, and a recoverable mistake before
reaching an obvious finish.

**Why this priority**: This is the kill test. If deliberate mass control does
not create satisfying decisions on one coherent course, more game systems will
not solve the core problem.

**Independent Test**: Start a fresh attempt, complete the course using only
the existing internal-mass controls, and compare a deliberate line with an
unplanned line through the bank, height, and momentum sections.

**Acceptance Scenarios**:

1. **Given** a new attempt, **When** the player enters the opening downhill
   section, **Then** they can build momentum and learn the lateral effect of
   their mass position before a demanding challenge.
2. **Given** the gentle bend, **When** the player shifts mass laterally before
   and through it, **Then** a controlled line is visibly more useful than
   leaving the ball on its initial line.
3. **Given** the stronger bank and the later height opportunity, **When** the
   player anticipates the geometry and chooses a higher or lower line, **Then**
   the choice produces understandable differences in route, height, and speed.
4. **Given** a poor but survivable line, **When** the player uses the available
   geometry and mass control, **Then** they have an opportunity to recover
   without an automatic reset.

---

### User Story 2 - Retry and Measure Improvement (Priority: P2)

As a player, I can restart immediately, see an obvious completion indication,
and compare my current completed run with the best completed run in this
application session.

**Why this priority**: The course only answers its question when repeated
attempts are fast enough for the player to recognise where they lost time and
to see whether practice improves their result.

**Independent Test**: Complete a run, restart with one key press, complete a
second run, and confirm that the active, completed, and best time states make
the improvement comparison clear.

**Acceptance Scenarios**:

1. **Given** an attempt is underway, **When** the player requests a restart,
   **Then** the ball, its motion, its internal mass, and the view return to the
   defined attempt start without restarting the application.
2. **Given** the player reaches the finish region, **When** the run completes,
   **Then** completion is obvious, the elapsed time stops, and the best
   session time updates only if the completed result is faster.
3. **Given** an already completed attempt, **When** the player restarts,
   **Then** a new timed attempt begins and the previous best time remains
   available for comparison.

---

### User Story 3 - Understand and Evaluate the Kill Test (Priority: P3)

As a developer-player, I can read the course, its boundaries, current
internal-mass state, useful speed information, and timing well enough to make
an honest assessment of the control mechanic.

**Why this priority**: The project needs useful evidence, including negative
evidence, rather than a course that merely looks more like a game.

**Independent Test**: During an attempt, identify the intended route, upcoming
bank, physical boundaries, mass display, speed or motion cue, current time,
and finish state; then record the kill-test questions after repeated attempts.

**Acceptance Scenarios**:

1. **Given** a player approaches each challenge, **When** they look ahead,
   **Then** course surfaces, boundaries, elevation changes, and the finish are
   distinct enough to understand the next decision.
2. **Given** the ball is rolling or has restarted, **When** the player observes
   the development display, **Then** the existing internal-mass visualisation
   remains readable and the new information directly assists evaluation.
3. **Given** repeated playtesting, **When** observations are recorded, **Then**
   each kill-test question has an affirmative, negative, or unresolved result
   rather than a manufactured positive conclusion.

### Edge Cases

- A restart during a rolling, airborne, stalled, or completed attempt restores
  one defined start state without retaining unintended movement or mass state.
- Reaching the finish after completion does not overwrite a completed time or
  repeatedly update the best result.
- A ball that leaves useful course bounds, falls indefinitely, or becomes
  trapped remains recoverable through the immediate player-controlled restart.
- If a modest airborne feature makes the course unreadable or destabilises the
  ground-control experiment, it is omitted or reduced and the reason is
  recorded rather than adding mid-air steering.
- If a course element cannot demonstrate its intended physical choice during
  playtesting, the result is recorded and the element is revised or removed.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The prototype MUST provide one compact, coherent,
  hand-authored course with a defined start and obvious finish.
- **FR-002**: An initially competent successful run SHOULD normally take
  approximately 30 to 90 seconds, unless playtesting documents why a different
  duration better serves the kill test.
- **FR-003**: The opening section MUST be forgiving enough to establish
  movement, initial momentum, and lateral mass influence before a demanding
  challenge.
- **FR-004**: The course MUST include a broad, readable direction change where
  modest lateral mass shifting produces a useful, observable line change.
- **FR-005**: The course MUST include a stronger bank or curve that rewards
  anticipation and makes poor positioning visibly less effective without
  routinely forcing an immediate reset.
- **FR-006**: The course MUST include a section that permits a player to gain
  and lose height through track geometry, so height and speed can be compared.
- **FR-007**: The course MUST include a rise, compression, crest, or similar
  momentum challenge where the preceding line materially affects the outcome.
- **FR-008**: The course MUST include at least one common poor line that has a
  meaningful consequence and a skill-based recovery opportunity.
- **FR-009**: A modest airborne event MAY be included only when it helps assess
  existing momentum, landing, and readability without adding mid-air steering;
  its inclusion or omission MUST be documented.
- **FR-010**: Course geometry and visible surfaces MUST correspond closely
  enough that a player can understand the physical route, banks, elevations,
  edges, dangerous exits, and finish.
- **FR-011**: Course construction MUST remain limited to the smallest
  hand-authored helpers or primitives needed for this one course.
- **FR-012**: The feature MUST NOT add general track formats, procedural track
  generation, a track editor, race infrastructure, opponents, or multiple
  balls.
- **FR-013**: A single documented developer-friendly input MUST restart an
  attempt immediately.
- **FR-014**: Restarting MUST restore the defined ball position and orientation,
  all relevant motion, internal-mass state, attempt timing, and camera state
  needed for a fair retry.
- **FR-015**: The course MUST provide a clearly distinguishable finish region
  and indicate successful completion without adding results, scoring, laps, or
  championship systems.
- **FR-016**: Each attempt MUST measure elapsed time from its start until its
  first successful completion.
- **FR-017**: The prototype MUST make current attempt time, completed run time,
  and best completed time for the current application session available to the
  player in a lightweight development-oriented form.
- **FR-018**: A best time MUST persist across restarts during the current
  application session and update only when a completed attempt is faster.
- **FR-019**: The existing internal-mass visualisation MUST remain available.
  Any new speed, velocity, timing, or debug display MUST directly aid control
  evaluation and avoid becoming a general developer console.
- **FR-020**: The camera MUST provide sufficient early visibility of upcoming
  downhill geometry, curves, banks, moderate elevation changes, restarts, and
  any retained modest airborne event for deliberate mass-shifting decisions.
- **FR-021**: Targeted physical or camera tuning MAY improve the experiment,
  but MUST preserve the causal relationship: mass moves, balance changes, and
  the ball responds through physics.
- **FR-022**: The feature MUST NOT introduce direct ball steering, artificial
  propulsion, hidden traction assistance, direct velocity changes, desired
  orientation changes, or arbitrary lateral control forces.
- **FR-023**: The project MUST record observations for prediction, improvement,
  banking, height-versus-speed, momentum, recovery, repeated interest, faster
  runs, and player responsibility, including negative or unresolved findings.
- **FR-024**: The project MUST document the course layout intent, construction
  conventions, restart control, timing behaviour, relevant tuning/camera
  changes, observations, known issues, and unresolved questions.
- **FR-025**: Relevant pure timing, restart, finish, or construction
  calculations MUST have focused deterministic tests where practical.
- **FR-026**: The build, relevant tests, formatting, and lint checks MUST pass
  without disabled checks or unjustified warnings.
- **FR-027**: The roadmap MUST be updated only for outcomes genuinely
  demonstrated by implementation and playtesting; no First Race work may be
  marked complete.

### Key Entities *(include if feature involves data)*

- **Prototype Course**: The one connected sequence of physical challenges,
  including its defined start, finish, visible boundaries, and intended choices.
- **Attempt**: One run from a restart or fresh start until the first finish or
  another restart; it has a current elapsed time and completion state.
- **Session Best**: The fastest completed attempt retained only while the
  application remains open.
- **Finish Region**: The clearly identified area that completes an active
  attempt once.
- **Kill-Test Observation**: A recorded affirmative, negative, or unresolved
  answer to one question about control understanding and repeated play.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A player can begin at the defined start and reach the defined
  finish on the compact prototype course without restarting the application.
- **SC-002**: An initially competent completed attempt normally lasts between
  30 and 90 seconds, or the playtest record explains the intentional exception.
- **SC-003**: In at least three paired attempts using different lines through
  the bank or height section, the player can identify a visible difference in
  path, height, speed, or completion time caused by their mass-shifting choice.
- **SC-004**: A player can restart and be ready for a fair new attempt with one
  input action, and the session-best completed time remains available after at
  least three restarts.
- **SC-005**: On completion, a player can identify the active/completed time
  and current-session best time within five seconds using the lightweight
  display or log.
- **SC-006**: At least one documented common poor line has a consequence and a
  recoverable path; it does not require an automatic reset to continue.
- **SC-007**: The kill-test record contains one affirmative, negative, or
  unresolved observation for each of the ten roadmap evaluation questions.
- **SC-008**: During a course attempt, the player can identify the next major
  challenge and the internal-mass display before reaching it, without a camera
  view inherited from ball rotation obscuring either.
- **SC-009**: Automated project quality checks pass, and a review confirms no
  conventional steering, opponent, race, multiplayer, or general track-system
  feature was introduced.

## Assumptions

- The existing single-ball internal-mass controls, physics behaviour, and
  follow view are the baseline; this feature may make only targeted changes
  required by course readability and the kill test.
- A keyboard-oriented, development-facing restart control and timing display
  are appropriate for this single-developer prototype; persistence across
  application launches is not required.
- A fixed course layout is preferable to track generalisation until the project
  knows which physical choices are fun.
- A modest jump is optional because the primary evaluation remains grounded
  mass control; no airborne-control behaviour is inferred from its presence.
- The course may expose a weak control model. An honest negative or unresolved
  observation is a successful kill-test outcome and should direct future work
  toward the control model rather than more content.

## Out of Scope

- Multiple balls, AI, opponents, race positions, starts/countdowns, laps,
  checkpoints, multiplayer, networking, ghosts, persistent leaderboards,
  progression, menus, audio polish, cosmetics, or production presentation.
- General racing, time-trial, track-authoring, procedural-generation, or
  track-editor frameworks.
- Surface-type gameplay systems, arbitrary mid-air steering, conventional ball
  steering, or content intended to disguise an unsatisfying control mechanic.
