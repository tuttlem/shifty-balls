# Phase 0 Research: First Physically Simulated Rolling Ball

## Decision: Avian 3D 0.7.0

Use avian3d version 0.7.0, pinned exactly alongside Bevy 0.19.1. Avian 0.7 is
the version line that supports Bevy 0.19. Its native ECS components provide the
immediate requirements: dynamic and static rigid bodies, sphere and cuboid
colliders, gravity, friction, restitution, damping, sleeping, angular motion,
forces, impulses, collision events, transform interpolation, and debug drawing.

Register its normal PhysicsPlugins group. Use the existing f32 3D defaults; do
not enable serialization, diagnostics UI, SIMD, a separate deterministic mode,
or mesh collider generation for this one-ball prototype. The optional
PhysicsDebugPlugin capability is development-only: expose it through a
non-default physics-debug feature if the initial implementation finds it useful.

### Rationale

Avian is designed as a Bevy ECS physics engine rather than a wrapper around a
separate simulation world. That makes the initial direct implementation smaller
and easier to inspect. Its component-oriented mass properties, including centre
of mass and angular inertia, leave a credible later experimental path for an
internal movable mass. This is not a commitment that modifying those components
will be the final control model; the next feature must test the causal behaviour
and APIs in isolation.

The project needs ordinary rolling rigid-body behaviour now, not a mature
networked or highly constrained simulation. Avian meets the actual requirement
without making a custom engine or a speculative wrapper necessary.

### Alternatives considered

| Alternative | Result |
|---|---|
| bevy_rapier3d 0.36.0 | Viable mature fallback with Bevy 0.19 support, debug rendering, and mass-property APIs. It is more established and feature-rich, but maintains a separately synchronised physics world and is less direct for the Bevy-native future experiment. |
| bevy_xpbd_3d | Rejected. It is the prior project lineage; Avian is the current maintained continuation for the compatible Bevy generation. |
| Custom rigid-body implementation | Rejected. It would delay the kill test and violates the constitution's instruction not to build custom physics merely for theoretical determinism. |
| Decorative/manual rolling | Rejected. It cannot establish gravity, collision, friction, or rotation as the shared causal system required by the feature. |

### Sources

- [Avian 3D 0.7 crate metadata and Bevy compatibility](https://docs.rs/crate/avian3d/latest)
- [Avian 3D getting started, dynamics, collision, debugging, and version table](https://docs.rs/avian3d/latest/avian3d/)
- [bevy_rapier3d 0.36 crate metadata](https://docs.rs/crate/bevy_rapier3d/latest)
- [Rapier collider mass-properties guide](https://rapier.rs/docs/user_guides/templates/collider_mass_properties/)

## Decision: Minimal world conventions

Use a conventional right-handed 3D world.

| Convention | Value |
|---|---|
| Up | Positive Y |
| Down-track in the first test | Positive Z |
| Track-right | Positive X |
| Unit scale | One world unit is one metre for render transforms and physics positions alike |
| Gravity | Explicit (0.0, -9.81, 0.0) metres per second squared |
| Ball radius | 0.5 metres |
| Ball mass | Explicit 1.0 kilogram mass property |
| Angular unit | Radians per second |

The ball begins at negative Z, clear of a descending static slope. The slope
falls towards positive Z and overlaps a level run-out. This keeps the world
small, lets familiar gravity values remain meaningful, and eliminates a
conversion layer between Bevy meshes and colliders.

## Decision: Primitive cuboid test track

Use visible cuboids with matching Static rigid bodies and cuboid colliders:

- a short, level starting deck;
- an 8-metre-wide, roughly 12-metre-long slope descending about 15–20 degrees
  toward positive Z;
- an 8-metre-wide level run-out at least 16 metres long;
- simple fixed retaining edges and a low end-stop.

The individual pieces overlap slightly at transitions so an accidental
collision seam does not dominate the experiment. This is intentionally not a
track system: no mesh collision, spline, segment abstraction, authoring format,
or reusable gameplay geometry is needed.

## Decision: Initial material and damping values

Start from a relatively high-friction, low-restitution contact: ball friction
0.8 and restitution 0.1; track friction 0.9 and restitution 0.05. Use zero
linear and angular damping initially so the observation shows ordinary contact
behaviour rather than hidden slowing. Only add small explicit damping if a
reproducible jitter/non-settling issue appears, and document the reason.

The end-stop and low restitution make a stationary result possible without
claiming a frictionless flat surface should magically stop a moving ball.

## Decision: Camera and visual debugging

Use a simple observation camera because the initial fixed bootstrap camera will
not reliably frame the complete traversal. Each frame it reads only the ball's
world translation, follows at a fixed behind-and-above offset, looks a modest
distance ahead along positive Z, and retains Y-up orientation. Smooth only the
camera translation. It does not read or inherit physical rotation, velocity,
or angular velocity.

Add a small contrasting child marker or narrow ring to the rendered ball so
the physics-driven orientation is visible. It is generated primitive geometry,
not production art. Keep physics debug drawing off in standard runs; enable it
only through the documented development feature if needed to inspect colliders.

## Testing approach

Do not attempt to re-test Avian's gravity and contact solver. Test only
project-owned deterministic helpers if implementation introduces them, such as
spawn-clearance or track-transition geometry invariants. Run the existing Cargo
quality suite. Manually observe a desktop run: the ball moves within five
seconds, rotates visibly, remains on the slope/run-out/end-stop system, and the
upright camera keeps it readable until the application closes normally.
