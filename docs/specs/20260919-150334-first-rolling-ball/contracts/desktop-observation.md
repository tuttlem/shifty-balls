# Desktop Observation Contract

## Purpose

This is the user-visible contract for the first rolling-ball physics experiment.
It describes observable behaviour, not a public API or a final gameplay loop.

## Invocation

~~~text
cargo run
~~~

The command launches one native desktop application titled Shifty Balls. Closing
the window exits the application cleanly.

## Scene contract

On launch, the application presents:

1. one perspective, world-upright camera view;
2. readable lighting;
3. a visually obvious spherical player ball, including a generated orientation
   detail;
4. a primitive start deck, downhill surface, run-out, and containment geometry;
5. no menu, HUD, opponent, timer, or control prompt.

## Motion contract

After startup, within five seconds under normal desktop conditions:

1. the single ball moves only because gravity acts on its dynamic rigid body;
2. it accelerates down the surface in the documented positive-Z down-track
   direction;
3. its visual orientation detail changes as physical contact produces rotation;
4. it remains supported by and transitions between the matching static slope
   and run-out collision geometry rather than falling through or escaping; and
5. it can lose energy and settle when contact geometry and material values make
   that physically appropriate.

The camera follows translation enough to keep the traversal readable, but
remains upright and does not rotate with the ball.

## Explicit exclusions

No keyboard, mouse, gamepad, or other gameplay input influences the ball. The
application contains no steering, propulsion, gravity toggle, internal mass,
centre-of-mass manipulation, multiple balls, ball-to-ball interaction, race
rules, production track, or final camera system.

## Development debug mode

If implementation adds the optional physics-debug Cargo feature, the following
may be used to inspect colliders during development:

~~~text
cargo run --features physics-debug
~~~

This is not required for normal use and must not change the ball's physics.
