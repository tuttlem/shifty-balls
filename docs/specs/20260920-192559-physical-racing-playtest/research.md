# Research: Physical Racing Playtest

## Decision: Tune one existing traffic bottleneck into the passing experiment

**Decision**: Retune the existing strong-bank section and its immediate run-out
into one deliberately broad, readable high/low-line racing section. Make the
bank and run-out a consistent approximately 16 m wide, soften the current peak
bank modestly, retain continuous low walls, and give the section exit a matching
approximately 7–7.5 m half-width progression gate. Keep a single connected
route; the passing choice is physical space within that route, not an
alternate-route system.

**Rationale**: The accepted First Race record identifies the final turn as too
difficult with traffic and too narrow for racing. The current 14 m strong bank
funnels into 12 m and 10 m surfaces, while route gates remain 10 m wide;
`course.rs` already owns these values, retaining walls, and broad route gates.
Widening and softening this one area creates an immediately playable experiment
while preserving current route/order rules and keeping a poor line consequential
but recoverable.

**Alternatives considered**:

- Add branching paths or a route graph: rejected because one wide high/low line
  answers the current passing question without new progression architecture.
- Add a new course or several track elements: rejected because it would hide
  the known final-turn problem behind content expansion.
- Leave the course unchanged and retune collision response alone: rejected
  because the signed-off race record identifies course space as the immediate
  limitation.

## Decision: Preserve ordinary collision material behaviour

**Decision**: Keep default ball-to-ball contact and the existing shared ball and
track friction/restitution baseline. Change material values only if a manual
playtest identifies a specific, repeatable contact problem; expose any such
value alongside the existing visible physics constants.

**Rationale**: The project already has dynamically released equal-mass balls,
static surfaces, and ordinary contact. Collision-specific impulses, freezes,
repositioning, damage, or catch-up would undermine the physical experiment.
The feature's first question is whether space and momentum make contact
recoverable, not whether custom collision rules can mask a bottleneck.

**Alternatives considered**:

- Add collision layers or bespoke contact events: rejected because current
  balls already collide and the feature needs no new category of interaction.
- Add a player-only recovery force or automatic re-centering: rejected because
  it removes meaningful physical consequences and violates the feature spec.
- Retune all physics values at once: rejected because it prevents a useful
  causal playtest and risks destabilising the established comparison/race paths.

## Decision: Treat physics solver tuning as a demonstrated stability fallback

**Decision**: Retain the current Avian default solver/substep configuration for
the initial course pass. If the desktop procedure demonstrates repeated
penetration, tunnelling, or jitter at observed race speeds, make one documented,
narrow stability adjustment and rerun the full procedure.

**Rationale**: Four equal 1 kg, 0.5 m-radius balls already participate in
ordinary Avian contact against static cuboid track geometry. The current
problem is an identified course bottleneck, not an observed solver failure.
Increasing solver work prematurely would cost performance and obscure whether
the geometry created a playable line choice.

**Alternatives considered**:

- Increase substeps before playtesting: rejected because no current evidence
  requires it.
- Build custom deterministic collision handling: rejected because middleware
  contact is deliberately the physical experiment and the constitution forbids
  custom physics for theoretical determinism.

## Decision: Retain common SHIFT participants and simple route-targeting AI

**Decision**: Keep the current fair four-racer start, human keyboard intent,
and AI target-at-next-gate behaviour. AI does not receive avoidance, overtaking,
rubber-banding, new control models, or teleport recovery.

**Rationale**: Existing AI uses the same internal-mass pipeline as the human,
and its central gate target gives the player space to choose a different line in
the widened section. This supports unscripted passes while keeping the
experiment understandable.

**Alternatives considered**:

- Add AI line-selection logic: rejected because the player can test a physical
  line advantage against simple central traffic first.
- Convert AI to torque or force: rejected because the prior comparison records
  an explicit inconclusive result and the race baseline remains SHIFT.
- Script opponents to create collisions: rejected because success must emerge
  from ordinary starts, momentum, and contact.

## Decision: Verify rules deterministically and contacts by desktop playtest

**Decision**: Add focused tests only for pure surface/gate geometry, route
progress tolerance, ordering, and rematch invariants changed by the tuning.
Use the documented three-race native desktop procedure to assess collision,
passing, camera readability, and recovery.

**Rationale**: The existing project separates pure ordered-route calculations
from middleware physics. Reproducing multi-body contact in unit tests would add
fragile coverage without proving the player-facing behaviour. Three consecutive
races give the owner a bounded evidence set for the Milestone F decision.

**Alternatives considered**:

- Unit-test full rigid-body traffic: rejected because solver outcomes are not a
  useful project-owned deterministic contract.
- Sign off from a single spectacular collision: rejected because it cannot
  establish repeatable recovery or passing.
- Add telemetry or replay capture: rejected as infrastructure beyond this
  playable slice.
