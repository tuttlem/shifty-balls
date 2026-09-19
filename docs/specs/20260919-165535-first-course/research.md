# Phase 0 Research: First Course Kill Test

## Decision: One course from direct, overlapping primitive surfaces

Build the course with the existing static rigid-body cuboid pattern. A small
course-local helper accepts a surface start point, width, logical length, and
orientation; it spawns matching visible and collision cuboids and returns the
next surface point. Side boundaries use the same orientation and matching
static collision, so a readable wall is never visual-only.

The helper is intentionally a local construction convenience, not a segment
library or parametric track system. Neighbouring surfaces overlap slightly
along their logical travel direction while their pitch, yaw, and bank change
gradually. This avoids gaps and sharp seams that would snag the 0.5 metre ball.

### Rationale

- Rendering and collision share the same dimensions and transform.
- Gradual rotated cuboids are enough to make broad bends, banks, high/low
  lines, compression, and a crest understandable.
- It is easy to adjust after playtesting and has no new dependency or format.

### Candidate sequence

1. Eight-metre level start deck and a generous 16-metre downhill opening.
2. Four gently turning, slightly banked surfaces forming a broad right bend.
3. Six gradually turning, more strongly banked surfaces forming a left bend
   with generous retaining walls and a high/low line.
4. A broad shallow descent/compression that rewards leaving the bank with
   speed and useful position.
5. A rise and mild crest whose outcome visibly depends on preserved momentum.
6. A forgiving descent and level finish apron with a bright finish marker.

The initial route is roughly 80–115 metres before playtest adjustment. It is
designed to target the requested 30–90 second competent run, not to make that
duration a substitute for feel.

### Alternatives considered

| Alternative | Decision |
|---|---|
| Splines or procedural meshes | Rejected: they expand authoring infrastructure before the project knows what a fun line is. |
| Separate render-only rails | Rejected: misleading physical boundaries make the control experiment harder to read. |
| Real gap/drop or jump | Omitted initially: the crest asks a momentum question without confounding grounded control with airborne behaviour. |
| Disconnected obstacle collection | Rejected: it cannot reveal whether a player anticipates a connected physical line. |

## Decision: A focused attempt resource, not a racing-state framework

Represent an attempt as either running or completed, with elapsed time and an
optional best completed time retained for the application session. A simple
finish-volume bounds predicate completes only a running attempt. R resets the
ball and current attempt but retains the best result.

### Rationale

The course needs only fast iteration and a reason to improve. An explicit,
small state model makes restart, completion, best-time comparison, and their
pure tests clear without introducing laps, checkpoints, results, or generic
game state.

### Reset boundary

Reset the dynamic ball through its normal authored transform plus zero linear
and angular velocity and a neutral centre of mass. Reset both requested and
current internal-mass positions. The physics integration synchronises the
normal transform path and wakes changed motion; do not write competing physics
transform representations or use force/impulse to wake the ball. Snap the
camera to the start view during the same restart so the player never waits for
it to pan across the course.

### Alternatives considered

| Alternative | Decision |
|---|---|
| Collision-event finish trigger | Rejected for this first course: an explicit finish bounds test is easier to inspect, test, and keep one-shot. |
| Checkpoint or respawn system | Rejected: player-controlled whole-course retry is the desired experiment loop. |
| Persistent records or leaderboards | Rejected: session-only best time answers the current question. |
| General state machine | Rejected: a focused running/completed attempt is sufficient. |

## Decision: World-up course camera and a small development readout

Keep the camera world-up and independent from rolling-shell rotation. Move its
look target ahead along the initial down-track direction and, only if course
playtesting needs it, blend in a bounded horizontal velocity lead while
retaining a down-track fallback at low speed. Do not rotate the camera around
erratic lateral recovery movement, add camera roll, shake, or speed FOV.

Show one lightweight readout: current or completed time, session best, speed,
restart hint, and a concise completion state. Preserve the existing world-stable
mass gizmos. A simple F3 development-display toggle may hide the gizmos and
readout together if they obstruct a test.

### Rationale

The camera must show the next physical decision early while maintaining a
readable horizon. Speed and time directly support the kill-test question; a
large telemetry system does not.

### Alternatives considered

| Alternative | Decision |
|---|---|
| Velocity-facing camera | Rejected initially: sideways recovery and backward movement can make it disorienting. |
| Ball-rotation camera | Rejected: it obscures the horizon and player intent. |
| Console-only timing | Rejected: timing must be immediately comparable during retries. |
| Production HUD framework | Rejected: one development readout is enough. |

## Testing decision

Unit-test only project-owned calculations: attempt timing transitions,
best-time selection, one-shot completion, inclusive finish bounds, and any
extracted restart/camera target helper. Do not reproduce Avian's contact solver.
Manual trials compare at least three intentional versus unplanned lines, assess
the ten roadmap questions honestly, and document either affirmative, negative,
or unresolved observations.
