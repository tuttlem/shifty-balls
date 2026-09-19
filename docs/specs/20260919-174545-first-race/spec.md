# Feature Specification: First Race

**Feature Branch**: `feature/first-race`  
**Created**: 2026-09-19  
**Status**: Draft  
**Input**: User description: "Create Milestone E — First Race: a deliberately small point-to-point race on the existing prototype course, with one human ball, physical AI opponents, a fair start, progression, positions, finishing order, results, and a fast rematch."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Race a Physical Field (Priority: P1)

As a player, I can begin alongside several distinct opponent balls, influence my own ball with the existing internal-mass controls, and race through the prototype course while ordinary physical contact can change every racer's line and momentum.

**Why this priority**: This is the first direct test of whether traffic makes the proven single-ball control mechanic more compelling. Without a shared physical race, none of the race presentation or scoring work has value.

**Independent Test**: Launch the game, wait for the start, race the player ball through the opening and banks beside opponents, and observe that contact causes physical, recoverable changes to one or more racers without direct steering or collision attacks.

**Acceptance Scenarios**:

1. **Given** a newly launched race, **When** its countdown completes, **Then** one human racer and three AI racers are released together from a readable starting arrangement.
2. **Given** two or more racers meet on the course, **When** they make ordinary contact, **Then** their movement and momentum respond physically and their visible paths may change.
3. **Given** an AI opponent, **When** it navigates the course, **Then** it influences its own internal mass rather than receiving direct movement or conventional steering.
4. **Given** the player is displaced by an ordinary collision, **When** the player still has usable track space, **Then** the player can continue attempting to recover with internal-mass control.

---

### User Story 2 - Understand Who Is Winning (Priority: P2)

As a player, I can see the countdown, my current race position, and a clear finish result, so that I understand whether I am beating the other balls rather than merely completing the course.

**Why this priority**: Traffic becomes competition only when the player can understand legitimate progress, changing position, and whether they won or lost.

**Independent Test**: Race at least two participants past the required course milestones, compare their displayed order during the race, and finish the player after valid progression to confirm the result agrees with the recorded order.

**Acceptance Scenarios**:

1. **Given** racers are preparing to start, **When** the countdown is active, **Then** the player sees a clear 3–2–1–GO indication and neither the race timer nor racers gain a meaningful early advantage.
2. **Given** racers have crossed different ordered course milestones, **When** their positions are displayed, **Then** the ordering favours the racer with legitimate further progress and remains understandable for nearby racers.
3. **Given** a racer reaches the finish area before required course progress, **When** it enters that area, **Then** it does not receive a finish position.
4. **Given** the player completes all required course progress and crosses the finish, **When** the finish is recorded, **Then** the player sees a fixed finishing position, elapsed race time, and a clear win-or-loss result.

---

### User Story 3 - Race Again and Evaluate Traffic (Priority: P3)

As a player, I can quickly begin another fair race and use development information and recorded observations to judge whether physical traffic improves the game.

**Why this priority**: Fast repetition is necessary to distinguish one amusing collision from a racing experience worth developing further.

**Independent Test**: Start a race, create or observe traffic, trigger a rematch before and after a finish, and confirm the next countdown begins with all racers, progress, timing, and results restored to a fair initial state.

**Acceptance Scenarios**:

1. **Given** a race is preparing, racing, or has completed, **When** the player requests a rematch, **Then** every racer returns to its assigned start state and a new countdown begins without restarting the application.
2. **Given** a rematch begins, **When** the countdown reaches GO, **Then** no previous velocity, finish order, progression, race time, internal-mass state, or AI intent affects the new race.
3. **Given** development display is enabled or disabled, **When** a race proceeds, **Then** it does not alter race outcomes or player control.

### Edge Cases

- A racer reaches a later gate or finish area out of order, moves briefly backwards, or is pushed sideways: only sequential, legitimate progress counts toward its position and eligibility to finish.
- Two racers are nearly level in the same course section: their order remains stable enough to be readable rather than visibly flickering every frame.
- A racer finishes, then is displaced by another ball: its recorded finishing position remains final.
- A player requests a rematch during countdown, active racing, or after a result: all racers reset cleanly and the new race has no early release.
- An AI is knocked away from its desired line or temporarily stalls: it may make a recoverable mistake, but must not teleport, directly steer, or receive hidden speed assistance to rejoin.
- A racer falls, becomes trapped, or cannot sensibly continue: the player retains the existing fast rematch path; this feature does not add checkpoint or penalty rules.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The game MUST support one human racer and three AI racers in the same race; the chosen field size MUST be centralised enough to adjust for prototype testing without a configuration framework.
- **FR-002**: Every racer MUST be a visually distinguishable, physically simulated ball with independent movement, rotation, collision participation, internal-mass state, progression, and finish state.
- **FR-003**: All racers MUST use substantially equivalent baseline physical characteristics for this experiment.
- **FR-004**: The human racer MUST retain the existing internal-mass controls and MUST NOT gain direct steering, propulsion, trajectory snapping, or hidden traction assistance.
- **FR-005**: AI racers MUST request internal-mass displacement to navigate; they MUST NOT set their own movement directly, rotate toward a desired heading, teleport to recover, or receive arbitrary steering forces.
- **FR-006**: AI racers MAY use knowledge of the current prototype course and simple desired lines, but the feature MUST NOT include advanced overtaking, collision avoidance, rubber-banding, difficulty modes, or route strategy.
- **FR-007**: Racers MUST collide with one another through ordinary physical contact and be able to exchange momentum and alter each other's lines.
- **FR-008**: The starting arrangement MUST place all racers close enough to create shared traffic while avoiding an immediate unstable pile-up, and each assigned start MUST be physically fair for the current course.
- **FR-009**: The game MUST provide a visible 3–2–1–GO countdown. Before GO, race timing and AI racing behaviour MUST remain inactive and player input MUST NOT create a meaningful launch advantage.
- **FR-010**: At GO, all racers MUST be released, timing MUST begin, and player and AI internal-mass control MUST become effective.
- **FR-011**: The race MUST use ordered, course-specific progression milestones to establish legitimate advancement through the existing prototype course.
- **FR-012**: A racer MUST not finish unless it has satisfied the required ordered progression, and a recorded finish position MUST remain fixed.
- **FR-013**: The game MUST calculate a readable current order for all active racers using legitimate progress first and advancement within the current course section second.
- **FR-014**: The player MUST see a minimal race display containing countdown status, current position expressed relative to the field, active race time, and the final player result.
- **FR-015**: When the player finishes, the game MUST display the player's finishing position, elapsed time, and whether the player won. Remaining AI racers MAY continue to establish a fuller order only if doing so does not complicate the vertical slice materially.
- **FR-016**: The game MUST provide one fast rematch input that restores all racer transforms, movement, rotations, internal-mass states, AI intent, progression, timing, positions, finish records, result display, and camera state before a new countdown.
- **FR-017**: The camera MUST continue to follow only the human racer and provide sufficient framing to understand the start and nearby traffic without introducing opponent, replay, split-screen, or cinematic cameras.
- **FR-018**: Existing development mass, velocity, and timing visibility MUST remain separately toggleable and MUST NOT become required race presentation.
- **FR-019**: The feature MUST document the racer/controller distinction, field and start arrangement, countdown/race lifecycle, ordered progression model, race ordering, AI control approach, rematch controls, known limitations, and first-race observations.
- **FR-020**: The feature MUST add focused automated coverage for deterministic race rules, including lifecycle/countdown transitions, ordered progression, finish eligibility and ordering, position comparison, active timing, and reset behaviour where those rules can be represented independently of physics simulation.
- **FR-021**: The feature MUST preserve passing build, relevant tests, formatting, linting, and existing single-ball internal-mass behaviour.
- **FR-022**: The feature MUST NOT add local or network multiplayer, additional tracks, laps, general checkpoints, production UI, track selection, ball classes, advanced AI, collision combat, persistent records, menus, progression, audio polish, or racing systems beyond this point-to-point vertical slice.

### Key Entities

- **Racer**: A participating physical ball with an identity, visual identity, assigned start, controller, race progress, current position, and finish record.
- **Controller**: The source of a racer's requested internal-mass displacement. The two controllers in this feature are the human input and the simple course-aware AI.
- **Race lifecycle**: The current shared phase—preparing, counting down, racing, or showing the player's completed result—which governs release, timing, control, progression, and rematch.
- **Progress milestone**: One required, ordered region on the current prototype course. It establishes legitimate advancement and prevents a shortcut from producing a valid finish.
- **Race result**: The immutable finish position and elapsed time recorded when an eligible racer reaches the finish; the player result also states win or loss.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A new race presents exactly four visually distinguishable racers—one human and three AI—and releases all four only after a visible 3–2–1–GO sequence.
- **SC-002**: In a manual race session, the player can observe at least one ordinary ball-to-ball contact that visibly changes the movement of one or both racers without a collision-specific control.
- **SC-003**: During a race in which racers reach different required milestones, the player's displayed place is expressed as one of four positions and agrees with legitimate ordered course progress in every manually observed comparison.
- **SC-004**: A racer that reaches the finish before completing required milestones is not recorded as finished, while an eligible racer receives one immutable finishing position.
- **SC-005**: The player can complete a valid race, receive a result showing place and elapsed time, and start a new fair countdown with one rematch input without relaunching the game.
- **SC-006**: In manual first-race observations, AI opponents complete or credibly contest the prototype course often enough to produce traffic in at least three consecutive starts; any failures are documented rather than hidden through direct movement assistance.
- **SC-007**: Deterministic race-rule tests, existing tests, formatting validation, and lint validation pass with no unjustified warnings.

## Assumptions

- The existing prototype course remains the sole point-to-point route; this feature may add only the ordered progression regions needed to race it.
- The initial field is one human racer plus three AI racers. The exact count may be adjusted after playtesting only through one focused field-size setting.
- The existing restart input becomes the fast rematch input, with its documented behaviour updated to reset a race rather than a single time-trial attempt.
- The player result may be shown immediately upon the player's valid finish. Continuing AI finishes and a full final order are optional only when they remain a small, clear extension.
- Current course difficulty and first-course limitations remain known issues. This feature evaluates traffic on that course; it does not broaden into a course-redesign or collision-tuning milestone.
- Manual playtesting will record whether traffic is fun, understandable, recoverable, and worth further collision-focused work. Positive outcomes must not be assumed in advance.
