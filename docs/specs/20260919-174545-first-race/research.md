# Research: First Race

## Racer ownership and multi-ball physics

**Decision**: Give every simulated ball racer-owned identity, controller marker, start state, internal-mass state, progression state, and finish state. Keep one shared mass-tuning value, but move the present singleton internal-mass state onto each racer. Generalise the existing centre-of-mass application to iterate racers; human input and AI are simply different writers of the same requested mass state.

**Rationale**: A ball/racer must own its physical and race state independently. This preserves the proven causal chain—requested mass position, current mass position, centre-of-mass offset, physical response—without coupling a ball to its controller or adding direct steering. Human-focused camera and debugging can still select the human marker.

**Alternatives considered**:

- Keep one shared mass resource and add separate opponent movement: rejected because it makes only the player a real internal-mass participant.
- Give AI direct velocity, force, or heading control: rejected because it violates the project’s physical-control principle.
- Build generic multiplayer/controller abstractions: rejected as premature for one local human and simple local AI.

## Collision participation and race staging

**Decision**: Keep the current default collision behaviour for balls and track. Stage every racer as physically held during preparing/countdown, then release all racers together into normal dynamic simulation at GO. Reset each racer to the same held state on rematch.

**Rationale**: The current default collision participation already allows ball-to-ball and ball-to-track contact, so collision layers add no value. A held start prevents gravity, contact, or early mass shifts from creating a false start. Releasing a held ball at GO is not propulsion; gravity and the internal-mass model remain the sources of movement. A non-neutral mass shift must continue to wake its own ball because the selected physics middleware does not automatically wake a sleeping body merely because its centre of mass changes.

**Alternatives considered**:

- Disable gravity during the countdown: rejected because racers could still be pushed or otherwise gain an early advantage.
- Rely on sleeping bodies: rejected because contact and centre-of-mass changes can wake them unpredictably.
- Lock movement axes: rejected because it creates artificial physics rather than a clean release.
- Add collision layers or custom contact logic now: rejected because ordinary physical contact is the experiment.

## Course-specific progress and finishing

**Decision**: Add four or five broad, ordered, oriented progression regions to the existing connected course, plus the existing finish. A racer advances only by entering its exact next region; short backwards travel never revokes prior progress. Only a racer that has completed the ordered regions can finish.

**Rationale**: The course turns and banks, so distance to the finish is not a valid measure of race progress. Broad oriented regions tolerate lateral bank lines and physics-step motion while preventing a shortcut from producing a win. The regions are deliberately course data, not a reusable track graph or checkpoint framework.

**Alternatives considered**:

- Use straight-line distance to the finish: rejected because it gives incorrect positions on the curved route.
- Use a universal spline, graph, or route system: rejected because there is one hand-authored course and no alternate routes.
- Use only thin crossing planes: rejected because a racer can miss a plane between physics steps.

## Position and finish ordering

**Decision**: Sort racers by immutable finish place first; otherwise by completed gate count, then by clamped forward advance through the current route leg, then by stable racer identity as a deterministic final tie-break. Simultaneous eligible finish candidates use stable identity order before places are allocated.

**Rationale**: This provides understandable, non-flickering-enough ordering for one connected course without pretending to solve general race timing. It makes race logic pure and testable independently of physics.

**Alternatives considered**:

- Recalculate only from live world position: rejected because close or backwards racers would flicker and shortcuts could lead.
- Add precision transponders, laps, or split timing: rejected because this is a first vertical slice.

## AI control

**Decision**: Use a simple route-aware AI that selects the next broad route target, derives a stable world-horizontal desired internal-mass direction, and writes that target through the same clamp/smoothing/centre-of-mass pipeline as the human. The AI remains neutral when no useful target direction exists.

**Rationale**: World-relative controls match the documented human frame and avoid the unintelligibility of sphere-local directions. Course knowledge is explicitly permitted for this prototype. Broad target points and ordered gates should make opponents capable of producing traffic while leaving their mistakes physical and visible.

**Alternatives considered**:

- Ball-local controls: rejected because rolling continuously scrambles input meaning.
- Kinematic or scripted path following: rejected because it would undermine physical collisions.
- Advanced racing lines, avoidance, overtaking, or recovery teleportation: rejected as Milestone F-or-later work.

## Player-facing and development visibility

**Decision**: Replace the single attempt readout with a minimal race display for countdown, place, elapsed time, and final result. Retain the existing independently toggleable development mass/velocity display for the human racer only. Use the existing restart key as a race rematch key.

**Rationale**: The player needs just enough feedback to understand competition. Showing debug state for all racers would obscure the race; a full production HUD or result screen would distract from the traffic experiment.

**Alternatives considered**:

- Preserve the time-trial UI alongside a separate race UI: rejected because two active timing models would confuse the vertical slice.
- Add menus, persistent results, detailed standings, or replay views: rejected as outside the experiment.
