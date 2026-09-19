# Desktop Mass-Shift Experiment Contract

## Invocation

~~~text
cargo run
~~~

The existing Shifty Balls window opens into the primitive rolling-ball
environment. The ball continues its gravity-driven no-input roll.

## Temporary controls

| Input | Requested mass direction |
|---|---|
| W | Down-track, positive Z |
| S | Up-track, negative Z |
| A | Track-left, negative X |
| D | Track-right, positive X |
| Combined W/A/S/D | Normalised diagonal request |
| No directional input | Target returns to neutral centre |

## Observable contract

1. The development display identifies ball centre, current mass, target mass,
   and allowed horizontal movement region.
2. Input moves current mass toward its target and never outside the boundary.
3. The display remains world-stable while the shell rotates.
4. Input affects only the inner-mass request. The outer ball receives no direct
   velocity, orientation, path, lateral force, torque, traction, or steering
   command.
5. Under grounded rolling, left/right displacement can influence observed path;
   forward/backward behaviour is available for comparison with no input.
6. Releasing keys returns the target toward centre without resetting or
   teleporting the ball.

## Exclusions

No race, gamepad, production UI, track system, airborne control, final camera,
or final-tuning claim is supplied. This is an experiment, not proof the game is
fun.
