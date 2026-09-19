# Research: Control Model Comparison

## Decision: Retain one stable, world-horizontal input frame

All models use the existing convention: `W` requests down-track `+Z`, `S`
up-track `-Z`, `A` track-left `-X`, and `D` track-right `+X`; diagonals are
normalised. A focused control-intent value represents this request before a
selected model consumes it.

**Rationale**: The ball rotates continuously, so ball-local axes would make
identical input incomprehensible and prevent a fair comparison.

**Alternatives considered**: Ball-local controls were rejected because spin
changes their meaning. Camera- and track-relative systems are separate
reference-frame experiments and are deferred.

## Decision: Use Avian one-step forces and torques in the fixed physics path

TORQUE and FORCE use Avian 3D 0.7's `Forces` query interface in the fixed
simulation schedule before `PhysicsSystems::Prepare`. `apply_force` and
`apply_torque` act for one physics step, wake a sleeping body when non-zero,
and clear after that step.

**Rationale**: Held input maps to physical quantities without direct velocity or
transform writes and without frame-rate-dependent impulses. The relevant local
Avian sources are `src/dynamics/rigid_body/forces/mod.rs` and `query_data.rs`.

**Alternatives considered**: Impulses immediately change velocity and need
timestep scaling. Persistent force components create stale release/reset state.
Direct velocity or rotation writes are prohibited.

## Decision: Map TORQUE to intended world roll direction

For horizontal intent direction `d`, apply world torque `up × d × strength`.
This produces the roll axis intended to move a contacting ball in direction
`d`, while friction/contact decide how much rotation becomes translation.

**Rationale**: It is a clean rotational contrast to SHIFT and remains visible
in air as rotation only.

**Alternatives considered**: Ball-local torque is not comprehensible. An
off-centre force mixes translation and torque, obscuring the comparison.

## Decision: Apply FORCE at centre of mass only

For horizontal intent direction `d`, apply `d × strength` at the ball centre of
mass. Zero intent applies no force.

**Rationale**: This isolates translational intent while retaining gravity,
contact, collision, and natural rotation.

**Alternatives considered**: Ground-only force needs contact/traction rules and
would answer a different question. Airborne horizontal acceleration remains a
documented observation for this first FORCE experiment.

## Decision: Isolate SHIFT and preserve First Race

`InternalMassState`, smoothing, and centre-of-mass coupling remain SHIFT-only.
TORQUE/FORCE comparison attempts reset the human centre of mass and mass state
to neutral. First Race retains its existing SHIFT human and AI controllers.

**Rationale**: Exactly one physical effect may be active in a comparison run;
AI work is deferred until a human preference exists.

## Decision: Small session switch and evidence-first evaluation

Use a focused comparison/race session state. `1`, `2`, and `3` start a
normalised single-ball attempt in SHIFT, TORQUE, and FORCE. `4` enters/resets
First Race; `R` resets the active session; `F3` retains development display.
Race-only opponents are physically excluded from comparison, not merely hidden.

Record at least three attempts per model, rotate model order between rounds,
and document completion/time, predictability, banks, recovery, airborne
behaviour, enjoyment, and the existing difficult-course confounder. Then run
First Race as a regression check. A negative or inconclusive result is valid.
