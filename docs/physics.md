# Initial Physics Experiment

Shifty Balls uses [Avian 3D 0.7.0](https://docs.rs/avian3d/0.7.0/avian3d/)
with Bevy 0.19.1 for its first rolling-ball experiment. Avian was selected
deliberately: it provides dynamic and static rigid bodies, spherical and cuboid
colliders, gravity, friction, restitution, angular motion, forces and impulses,
collision events, transform interpolation, and optional collider debug drawing
through Bevy-native ECS components.

Rapier was the mature fallback considered for the same requirements. Avian is
the current choice because it keeps this small experiment direct in Bevy's ECS
and exposes mass-property components that make a later centre-of-mass
experiment practical to investigate. This does not choose or implement the
future internal-mass control model. That model must still prove the causal chain
of mass shift, balance change, and ball response through playtesting.

## World conventions

| Convention | Value |
|---|---|
| World up | Positive Y |
| Initial down-track direction | Positive Z |
| Track-right | Positive X |
| Scale | One Bevy world unit is one metre; meshes and colliders share transforms and scale |
| Gravity | (0.0, -9.81, 0.0) metres per second squared |
| Linear velocity | Metres per second |
| Initial ball radius | 0.5 metres |
| Initial ball mass | 1.0 kilogram |
| Angular measurement | Radians per second |

The first test track has a level start deck, a roughly 18-degree slope that
descends towards positive Z, an overlapping level run-out, retaining edges, and
an end-stop. It is disposable test geometry, not a track system.

## Prototype contact values

| Object | Friction | Restitution | Damping |
|---|---:|---:|---|
| Ball | 0.8 | 0.1 | None initially |
| Track | 0.9 | 0.05 | N/A for static geometry |

There is no linear or angular damping in the first run. If repeatable jitter
prevents a ball from settling, a later tuning change must document its reason
rather than hiding it as an unexplained gameplay force.

## Debugging and current boundary

Avian can draw collision geometry, but normal cargo run stays clean and this
prototype does not add a permanent debug overlay. The visible primitive track
and a contrasting marker on the ball are enough to inspect the first experiment.

The ball has no player input, steering, propulsion, force, impulse, movable
internal mass, centre-of-mass control, additional ball, or race rule. Gravity
and normal physical contact own its motion.
