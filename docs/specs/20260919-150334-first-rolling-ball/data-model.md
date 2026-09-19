# Data Model: First Physically Simulated Rolling Ball

This feature uses direct Bevy and Avian ECS components. These are conceptual
models and small project-owned markers/constants, not a request for a generic
entity abstraction.

## PhysicsConventions

**Purpose**: One authoritative set of prototype constants shared by spawning,
physics configuration, and documentation.

| Field | Value / validation |
|---|---|
| up axis | Positive Y |
| down-track axis | Positive Z |
| metres per unit | 1.0; no rendering-to-physics conversion |
| gravity | (0.0, -9.81, 0.0) metres per second squared |
| ball radius | 0.5 metres; positive |
| ball mass | 1.0 kilogram; positive and explicit |
| ball material | friction 0.8; restitution 0.1 |
| track material | friction 0.9; restitution 0.05 |
| damping | Initially zero; only changed with recorded physical reason |

**Relationships**: Static track dimensions, player-ball collider, visible mesh,
camera offsets, and spawn clearance all use these conventions.

## SimulatedPlayerBall

**Purpose**: The one sphere whose physical motion is observed. It replaces the
bootstrap scene's decorative player sphere.

| Aspect | Representation / rule |
|---|---|
| Identity | Project-owned PlayerBall marker; exactly one startup entity |
| Body | Avian Dynamic rigid body |
| Shape | Sphere collider with the documented 0.5 metre radius |
| Mass | Explicit 1.0 kilogram mass property, not an implicit visual convention |
| Contact | Friction and restitution from PhysicsConventions |
| Motion | Gravity and collision only; no initial velocity, steering, force, impulse, or propulsion |
| Rendering | Generated sphere mesh at the same transform/scale as collider |
| Orientation marker | Generated contrasting child mesh or marker that follows physical rotation |
| Smoothing | Physics transform interpolation where Avian requires it for visible fixed-step motion |

**States**:

1. Spawned clear of the start deck and slope.
2. Accelerating and rolling under gravity.
3. Crossing the overlapping slope-to-run-out transition.
4. Contacting the end-stop and potentially settling/sleeping.

No player-controlled state exists in this specification.

## PrimitiveTestTrack

**Purpose**: Static, deliberately disposable geometry for observing natural
rolling; it is not a production track model.

| Piece | Rules |
|---|---|
| Start deck | Static visible cuboid with matching cuboid collider; supports the clear ball spawn. |
| Downhill slope | Static, 8 metre wide cuboid about 12 metres long, rotated to fall 15–20 degrees towards positive Z. |
| Run-out | Static, 8 metre wide, level cuboid at least 16 metres long. It slightly overlaps the slope. |
| Retaining edges | Static narrow cuboids where needed to prevent trivial lateral escape. |
| End-stop | Static low cuboid to contain the first traversal. |

Every rendered piece has a matching collider transform and dimensions. No
spline, mesh collider, procedural generation, segment type, checkpoint, or
race metadata may be added.

## ObservationCamera

**Purpose**: Lets a developer see the one uncontrolled ball without defining
the final gameplay camera.

| Aspect | Rule |
|---|---|
| Identity | One perspective Camera3d entity with a project-owned observation marker if needed. |
| Follow target | PlayerBall world translation only. |
| Position | Fixed behind-and-above offset, smoothed for readability. |
| Look target | Ball position plus a modest positive-Z look-ahead. |
| Up direction | Positive Y; never inherited from ball rotation. |
| Exclusions | No velocity look-ahead, FOV response, shake, orbit input, jump framing, or track-relative banking. |

## DebugVisibility

**Purpose**: Optional development-only way to inspect collider placement.

| Aspect | Rule |
|---|---|
| Normal run | No permanent wireframe/debug overlay. |
| Optional mode | A non-default Cargo feature named physics-debug may register Avian's debug renderer. |
| Scope | Collider inspection only; no telemetry, replay, or debugging framework. |
