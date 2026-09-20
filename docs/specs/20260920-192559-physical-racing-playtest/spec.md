# Feature Specification: Physical Racing Playtest

**Feature Branch**: `20260920-192559-physical-racing-playtest`

**Created**: 20 September 2026

**Status**: Draft

**Input**: User description: "Define the next roadmap feature that moves Shifty Balls toward a playable game."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Race Through Recoverable Traffic (Priority: P1)

As a player, I can race alongside three opponents, make ordinary ball-to-ball
contact, and remain able to steer my run back toward the course instead of
being left in an unwinnable state.

**Why this priority**: A race is only enjoyable if traffic creates pressure
without routinely ending a player's attempt. This directly advances the
roadmap's collision-tuning and recovery goals.

**Independent Test**: Start a normal four-ball race, experience at least one
ordinary contact in the opening traffic, deliberately lose some line or speed,
recover to the course, and finish the race without using a developer-only
reset.

**Acceptance Scenarios**:

1. **Given** a new four-ball race, **When** the countdown releases the field,
   **Then** all racers begin together and ordinary contact can change their
   relative lines or momentum.
2. **Given** the player's ball is displaced by ordinary traffic but remains in
   the playable course area, **When** the player continues using normal
   controls, **Then** the player has a readable route back toward the next
   required gate or finish.
3. **Given** the player is recovering from a contact, **When** the player
   completes the route, **Then** progression, place, time, and result remain
   valid without special collision rules.

---

### User Story 2 - Create a Natural Passing Opportunity (Priority: P2)

As a player, I can choose a different physical line through a race section and
occasionally pass or be passed through momentum, position, and traffic rather
than scripted rubber-banding or a forced pass.

**Why this priority**: Overtaking turns a group of independently rolling balls
into a race with decisions rather than a procession from the starting grid.

**Independent Test**: Run repeated races and demonstrate at least one change in
relative order after the start that occurs through normal movement, track space,
or contact, then complete the race.

**Acceptance Scenarios**:

1. **Given** racers approach the designated racing section together, **When**
   the player takes a higher, lower, wider, or tighter available line, **Then**
   the player can remain on a valid route while gaining or losing position.
2. **Given** two racers enter that section with different speed or lateral
   position, **When** neither receives a special movement aid, **Then** either
   racer can emerge ahead through ordinary physics.
3. **Given** a player loses a passing attempt, **When** the player continues
   the race, **Then** the course leaves enough recoverable space to keep racing
   rather than requiring an immediate restart.

---

### User Story 3 - Repeat a Readable Physical Race (Priority: P3)

As a player, I can replay the same race and understand how track position,
momentum, and contact affected the outcome.

**Why this priority**: Repetition is needed to establish whether the prototype
has meaningful player skill rather than one-off chaos.

**Independent Test**: Complete three rematched races, observe traffic and one
passing or recovery situation in each, and record whether outcomes felt
understandable.

**Acceptance Scenarios**:

1. **Given** a completed race, **When** the player rematches, **Then** the
   field, race state, route, and camera return to a fair common start.
2. **Given** repeated races over the same course, **When** player line choice
   and contacts differ, **Then** the displayed position and finishing order
   reflect those differences.

### Edge Cases

- What happens when several racers occupy the same wide section at once?
  The player must retain a visible, physically reachable route forward or a
  recoverable route back to it; no racer may become permanently immobilised.
- What happens when contact pushes a racer toward a boundary? The racer must
  either remain able to rejoin through normal control or use an existing,
  clearly communicated recovery outcome; an invisible correction is not
  acceptable.
- What happens when an opponent falls far behind or ahead? The race must retain
  valid progression and finishing order without teleporting, artificial speed
  boosts, or changing the player's physics.
- What happens if a normal collision causes a temporary loss of camera framing?
  The player must regain a usable view quickly enough to attempt recovery.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The feature MUST retain the existing point-to-point, four-racer
  race lifecycle: common countdown, ordered route progression, displayed place,
  finish result, and rematch.
- **FR-002**: The race course MUST contain at least one deliberately readable
  racing section with enough lateral or vertical space for racers to take
  distinct valid physical lines.
- **FR-003**: The feature MUST allow ordinary ball-to-ball contact to transfer
  visible movement consequences without scripted collision outcomes, stun
  states, or direct repositioning.
- **FR-004**: A player displaced by ordinary contact within the playable course
  area MUST have a normal-control recovery path toward the race route.
- **FR-005**: The feature MUST make it possible for relative order to change
  after the start through player line choice, momentum, and ordinary contact;
  it MUST NOT grant catch-up speed, rubber-banding, automatic overtakes, or
  collision-specific control assistance.
- **FR-006**: The human racer and opponents MUST continue to use the established
  SHIFT physical-control baseline for this feature. This feature MUST NOT select
  a new preferred control model or expand AI into a generic controller system.
- **FR-007**: Camera framing and race readout MUST remain sufficient for a
  player to identify their position, next physical direction, and recovery
  opportunity during normal traffic.
- **FR-008**: The feature MUST preserve a fast rematch that restores all racers
  to the same start state and clears motion, progression, and prior result data.
- **FR-009**: The feature MUST include a documented manual playtest procedure
  for contact, recovery, overtaking, repeated rematches, and perceived fairness.
- **FR-010**: The feature MUST record a signed-off, affirmative, negative, or
  unresolved result for collision readability, recovery, overtaking, line
  choice, and player-skill influence. A negative or unresolved finding is valid
  evidence and must not be hidden.

### Key Entities

- **Racing section**: A bounded portion of the existing route intentionally
  shaped to allow distinct physical lines and traffic interaction.
- **Recovery route**: The visible, normal-control path from a traffic-induced
  displacement back toward the next valid race gate.
- **Passing event**: A change in relative race order after the start caused by
  ordinary physics, line choice, or traffic, rather than a scripted aid.
- **Physical-race observation**: A recorded owner-playtest finding about
  contact, recovery, overtaking, fairness, and whether outcomes reflect skill.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 3 consecutive owner-playtest races, the player observes at
  least 1 ordinary ball-to-ball contact per race and can complete each race
  without a developer-only reset.
- **SC-002**: In at least 2 of 3 owner-playtest races, relative race order
  changes after the start in the designated racing section through normal
  physics, line choice, or traffic.
- **SC-003**: In at least 2 of 3 owner-playtest races where the player is
  displaced by contact, the player can rejoin the valid route and reach the
  finish using normal controls.
- **SC-004**: During the 3-race playtest, the owner can correctly identify their
  displayed place and a visible forward or recovery direction after every
  observed traffic incident.
- **SC-005**: The playtest record explicitly classifies collision readability,
  recovery, overtaking, line choice, and player-skill influence as affirmative,
  negative, or unresolved before the feature is signed off.

## Assumptions

- The existing one-human, three-opponent point-to-point race is the baseline;
  local multiplayer, additional race formats, and network play are out of
  scope.
- The established SHIFT input remains the active baseline while the prior
  control comparison remains deliberately inconclusive.
- One targeted course/race-tuning iteration is sufficient for this feature;
  reusable track language, new track elements, and broad physics-engine work
  remain later roadmap items.
- A fair physical outcome may include falling behind after a collision. The
  feature evaluates whether recovery is understandable and possible, not whether
  every contact is favourable.
- Existing debugging display may support development observation, but completion
  is judged from the normal playable race rather than developer tooling.
