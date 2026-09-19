# Phase 0 Research: First Internal Mass Shift

## Decision: Runtime centre-of-mass offset

Use Avian 3D 0.7.0 CenterOfMass on the existing dynamic PlayerBall. Maintain a
project-owned conceptual inner mass with requested and current positions in a
stable world-relative horizontal frame. On every fixed physics tick, derive the
combined world COM offset and transform it into the rolling body's local frame
before updating CenterOfMass.

Initial values: shell 0.55 kilograms, conceptual inner mass 0.45 kilograms,
total one kilogram, inner-mass horizontal disk radius 0.30 metres, maximum
combined COM offset 0.135 metres, and 1.5 metres-per-second target/return
movement. The total body mass remains one kilogram.

### Rationale

CenterOfMass is Avian's native local mass-concentration component. It is safe to
update and a changed mass property queues recomputation of
ComputedCenterOfMass. Avian contact constraints use centres of mass, so an
offset affects contact moment arms and rolling response. Writing it before
physics preparation leaves gravity and normal contact to determine movement.

This keeps the physical story inspectable: input changes a bounded inner target;
current inner position derives a combined COM; ordinary contact produces any
response. It adds no independent collider, joint, motor, force, or torque.

### Compromises and limitations

This is a mass-property approximation rather than a fully constrained internal
rigid body. It does not model actuator reaction forces or angular-inertia change
as the inner mass travels. Its useful initial behaviour is expected to be
grounded/contact-driven, not airborne propulsion or final controls. If weak,
tune mass ratio, displacement, movement speed, and contact values before
considering another model; never compensate with steering code.

### Source evidence

- [Avian CenterOfMass documentation](https://docs.rs/avian3d/0.7.0/avian3d/dynamics/rigid_body/mass_properties/components/struct.CenterOfMass.html)
- Installed Avian component docs: mass_properties/components/mod.rs lines 780-915
- Installed Avian mass-property schedule: mass_properties/mod.rs lines 299-406
- Installed Avian physics schedule: src/schedule/mod.rs

## Decision: World-relative WASD

| Key | Requested mass direction |
|---|---|
| W | Positive Z, initial down-track |
| S | Negative Z, initial up-track |
| A | Negative X, track-left |
| D | Positive X, track-right |

Combined input is normalised and disk-clamped. Releasing every direction targets
centre. This is clearer than ball-local controls because the shell rotates, and
simpler than the current camera-relative alternative.

## Decision: World-stable development display

Keep the display separate from the rotating ball mesh. It shows an outer-ball
centre marker, bright current marker, translucent target marker,
centre-to-current and centre-to-target lines, plus an X/Z disk-limit reference.
It is development-only, unparented, and never influences physics.

## Alternatives considered

| Alternative | Decision |
|---|---|
| Child/compound collider and mass | Avian can aggregate child mass properties, but collision filtering, hierarchy management, and per-step recomputation add complexity without first-test value. |
| Constrained internal dynamic body | More literal, but a bounded 3D mass needs multiple joints, collision filtering, solver tuning, and has higher instability. |
| Explicit external torque or force | Rejected because it risks becoming disguised steering. |
| Direct CenterOfMass | Selected because it is native, compact, tuneable, and contact-mediated. |

## Testing decision

Unit-test only project-owned intent mapping, disk clamp, move-toward, mass-ratio
COM calculation, and identity/quarter-turn frame conversion. Do not reproduce
Avian gravity, collision, or contact solving.
