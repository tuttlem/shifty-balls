# Feature Specification: Half-Pipe Racing Section

**Feature Branch**: `20260920-202718-half-pipe-racing-section`

**Created**: 20 September 2026

**Status**: Signed off

**Input**: User description: "Create the next roadmap specification after the
signed-off Physical Racing Playtest, moving Shifty Balls toward a playable game."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Navigate a Forgiving Half-Pipe (Priority: P1)

As a player, I can enter a broad half-pipe section, use the ball's physical
momentum and internal-mass control to choose a line through it, and exit toward
the next race section without being forced into an opaque failure.

**Why this priority**: The roadmap identifies half-pipes as a core track
language question. This is the smallest new geometry that can prove whether
height, banking, recovery, and momentum create a satisfying physical decision.

**Independent Test**: Start a normal race, drive through the half-pipe using a
low and a higher line on separate runs, and reach the next required route region
without a developer-only reset.

**Acceptance Scenarios**:

1. **Given** a player reaches the half-pipe entry, **When** the player follows
   a low line, **Then** the ball retains a readable route through the section
   and can exit toward the next required region.
2. **Given** a player reaches the half-pipe entry with usable momentum,
   **When** the player chooses a higher wall line using normal controls,
   **Then** the ball's height and exit position visibly differ from the low
   line without a scripted lane or direct steering.
3. **Given** a player enters the half-pipe poorly but remains in its playable
   area, **When** the player continues using normal controls, **Then** the
   player has a readable recovery route rather than an invisible correction.

---

### User Story 2 - Race Through the Half-Pipe (Priority: P2)

As a player, I can share the half-pipe with opponents and see traffic, line
choice, and momentum affect the race without turning the section into an
unrecoverable bottleneck.

**Why this priority**: A track element belongs in the playable game only if it
works under the pressure of the four-ball race, not merely in an isolated
physics demonstration.

**Independent Test**: Complete repeated four-ball races through the half-pipe,
observe an ordinary traffic or order-change situation there, and finish without
a special recovery mechanic.

**Acceptance Scenarios**:

1. **Given** multiple racers enter the half-pipe, **When** they use different
   physical lines or make ordinary contact, **Then** their momentum and
   relative order may change without collision attacks, catch-up assistance, or
   automatic repositioning.
2. **Given** traffic displaces a player within the half-pipe, **When** the
   player continues using normal controls, **Then** the player can identify a
   route toward the exit or records an explicit negative finding.
3. **Given** the player exits the half-pipe, **When** the player reaches later
   route regions and the finish, **Then** existing ordered progression, place,
   result, and rematch remain valid.

---

### User Story 3 - Learn Whether Half-Pipes Belong in the Game (Priority: P3)

As a player, I can repeat the same race and decide whether the half-pipe makes
the course more interesting, understandable, and worth replaying.

**Why this priority**: The point of this milestone is evidence for future track
language, not accumulating geometry for its own sake.

**Independent Test**: Complete three rematched races, try low and high lines,
and record affirmative, negative, or unresolved evidence about momentum,
recovery, traffic, readability, and enjoyment.

**Acceptance Scenarios**:

1. **Given** a completed race, **When** the player rematches, **Then** the
   half-pipe, fair start, route, camera, and race state return to the same
   usable baseline.
2. **Given** repeated runs with different lines, **When** the player compares
   the outcomes, **Then** the player can describe a visible consequence of line
   choice or record that the section did not create one.

### Edge Cases

- What happens when a ball climbs high on a half-pipe wall? The player must
  retain a readable normal-control route back into the section or an explicit,
  visible recovery outcome; no invisible force may return the ball to a line.
- What happens when several balls enter together? Ordinary contact may be
  chaotic, but it must not routinely trap racers or invalidate ordered route
  progression.
- What happens when a ball loses momentum near the half-pipe centre? The player
  must be able to attempt a normal physical recovery, and the observation
  record must state whether that is satisfying.
- What happens when the camera loses useful framing near a wall? The player
  must regain enough forward/recovery context to continue driving, or the
  result must be recorded as negative.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The feature MUST add one broad half-pipe racing section to the
  existing connected point-to-point course, with a readable entry and exit that
  preserve the current ordered route.
- **FR-002**: The half-pipe MUST support distinct low and higher physical lines
  whose height, momentum, or exit position can differ through normal ball
  physics and the established internal-mass control.
- **FR-003**: The feature MUST retain ordinary gravity, contact, momentum,
  collision, and the existing SHIFT physical-control baseline for every racer.
- **FR-004**: The half-pipe MUST provide a visible, normal-control recovery
  path for an in-bounds poor entry, wall climb, traffic displacement, or
  low-speed state; it MUST NOT use automatic centering, direct motion,
  teleporting, collision-specific assistance, or a hidden lane.
- **FR-005**: The half-pipe MUST remain usable by the existing four-racer race
  without adding AI avoidance, scripted passing, rubber-banding, collision
  attacks, or a generic controller architecture.
- **FR-006**: The existing race lifecycle, ordered progression, position,
  result, camera purpose, and rematch MUST remain valid before, during, and
  after the half-pipe.
- **FR-007**: The player MUST be able to recognise the half-pipe's intended
  path, available line space, and exit direction from the normal race view.
- **FR-008**: The feature MUST include a documented three-race desktop
  procedure covering low/high lines, traffic, recovery, rematch, camera
  readability, and perceived enjoyment.
- **FR-009**: The feature MUST record an affirmative, negative, or unresolved
  owner finding for half-pipe usefulness, line choice, momentum, recovery,
  traffic, camera readability, and replay value. Negative evidence is valid and
  must not be concealed with unrelated track content.

### Key Entities

- **Half-pipe racing section**: A bounded course section with a readable entry,
  curved/banked containment, usable low and higher lines, and a visible exit.
- **Half-pipe line**: A player-selected physical path through the section whose
  height, momentum, or exit position differs from another valid line.
- **Half-pipe recovery**: A normal-control attempt to regain a usable line
  after a poor entry, traffic displacement, wall climb, or low-speed state.
- **Track-language observation**: A recorded owner finding on whether the
  section creates understandable, satisfying, replayable racing decisions.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: In 3 consecutive owner-playtest races, the player enters and
  exits the half-pipe through the valid route without a developer-only reset.
- **SC-002**: Across those 3 races, the player successfully attempts at least
  1 low line and 1 higher line and can identify a visible difference in height,
  momentum, exit position, or race outcome.
- **SC-003**: In at least 2 of 3 races, an ordinary traffic, order-change, or
  side-by-side situation occurs in or immediately around the half-pipe without
  invalidating route progression.
- **SC-004**: In at least 2 of 3 deliberate in-bounds recovery attempts, the
  player can identify a route toward the exit and finish using normal controls.
- **SC-005**: Before sign-off, the owner records affirmative, negative, or
  unresolved findings for usefulness, line choice, momentum, recovery, traffic,
  camera readability, and replay value.

## Assumptions

- The new element extends the existing one-human, three-AI, point-to-point race;
  alternative routes, laps, new race modes, multiplayer, and another track are
  out of scope.
- The existing SHIFT control baseline remains in use for human and AI racers.
- One hand-authored half-pipe section is sufficient to evaluate this track
  element; a reusable track system, bowls, funnels, jumps, drops, and surface
  variants remain separate roadmap decisions.
- A poor line may cost position or momentum. Success means its consequence is
  understandable and recoverable, not that every line is equally fast.
- Existing race camera and readout are the baseline. A focused adjustment is
  allowed only if playtest evidence shows the half-pipe prevents readable play.
