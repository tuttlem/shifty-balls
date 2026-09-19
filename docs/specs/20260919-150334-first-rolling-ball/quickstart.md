# Quickstart: Validate the First Rolling Ball

## Prerequisites

- Rust 1.95.0 installed through rustup; the repository toolchain file selects
  it and requests rustfmt and Clippy.
- Native desktop graphics drivers supported by Bevy.

The expected scene and behavioural boundaries are defined in the
[desktop observation contract](contracts/desktop-observation.md). Prototype
entities and values are in the [data model](data-model.md).

## Validate the repository

From the repository root, run:

~~~text
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
~~~

All commands must complete successfully with no warnings accepted by the lint
policy.

## Validate the desktop experiment

1. Run cargo run from the repository root.
2. Confirm a window named Shifty Balls opens and stays responsive.
3. Confirm the camera shows the primitive cuboid test world and one visibly
   marked ball.
4. Without touching a gameplay control, observe the ball begin moving downhill
   within five seconds.
5. Confirm its marker rotates, it passes from slope to run-out without falling
   through a seam, and containment geometry prevents an immediate escape.
6. Confirm the view follows without rolling/banking with the ball.
7. Close the window and confirm the application exits normally.

## Optional collider inspection

Only if the implementation provides the non-default feature, run:

~~~text
cargo run --features physics-debug
~~~

Confirm the displayed ball and static-track collider bounds agree with their
visible primitive geometry. Return to plain cargo run for the normal scene.

## Review boundaries

Confirm the implementation has not introduced physics input, conventional
steering, internal mass, custom forces, multiple balls, race systems, track
frameworks, production assets, or a final camera. Update roadmap entries only
after the listed observation and quality checks succeed.
