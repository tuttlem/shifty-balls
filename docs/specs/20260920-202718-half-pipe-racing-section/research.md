# Research: Half-Pipe Racing Section

## Decision: Use five adjacent primitive strips

**Decision**: Build a 16 m-long faceted U-channel from one 4 m flat centre strip, two 4 m strips on each side, and outer-lip retaining walls only. Inner and outer side strips use symmetric moderate/steeper banks (approximately 18° and 38°); every strip keeps matching visible and static collision geometry.

**Rationale**: The existing course already uses overlapping cuboid surfaces. Five strips provide a real low lane and climbable higher lines without a curved mesh, custom collider, or hidden containment. Per-strip walls would create traps, so only the two outer lips receive walls.

**Alternatives considered**:

- Curved mesh/trimesh: rejected as a disproportionate collision/asset pipeline.
- Two V-panels: rejected because they pinch traffic and lack a usable low line.
- A stronger single bank: rejected because it repeats the existing bank rather than testing a concave recovery shape.

## Decision: Insert before the signed-off traffic bank

**Decision**: Preserve the first gradual bank segments as approach, add the half-pipe around the continuing course direction, then flatten into the existing wide traffic bank/run-out. Add one broad exit gate centred on the low line.

**Rationale**: Racers gain readable downhill momentum before the pipe while the validated passing zone remains a separate later-race test. Current AI already targets the next gate centre, creating simple central traffic and leaving the human a physical wall-line choice.

## Decision: Retain all physics and controller baselines

**Decision**: Keep current gravity, friction, restitution, solver, SHIFT control, AI target logic, camera, and race state. Change any of them only after a repeatable desktop observation identifies a specific defect.

**Rationale**: The feature tests track geometry, not assistance or tuning. Static surfaces and visible lips provide recovery; no auto-centering, force, teleport, collision events, catch-up, or scripted line is allowed.

## Decision: Test layout deterministically; test feel in the race

**Decision**: Unit-test strip symmetry, adjacent placement, outer-wall positions, and broad exit-gate bounds. Manually test low/high lines, wall climb, low speed, traffic, camera framing, rematch, and enjoyment in three races.
