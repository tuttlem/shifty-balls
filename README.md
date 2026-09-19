# Shifty Balls

Shifty Balls is a 3D physics-driven racing game prototype. The eventual player control mechanic
will influence a rolling ball by shifting its internal mass rather than steering it like a vehicle.

The project is currently at its first physics experiment: one uncontrolled ball rolls down a
primitive static test track under gravity. There is no player input, steering, internal-mass
mechanic, race system, or production track yet.

## Prerequisites

- A native Linux, macOS, or Windows desktop with a working graphics and display environment.
- Rust 1.95.0 or newer on the stable channel.

Update the stable toolchain when needed:

```sh
rustup update stable
```

## Build and Run

```sh
cargo build
cargo run
```

The application window is titled **Shifty Balls**. It shows one marked ball, primitive slope and
run-out geometry, and a simple follow view. Gravity and rigid-body contact move the ball; closing
the window exits normally.

## Validate

```sh
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

The initial rigid-body simulation is manually observed rather than unit-tested as a duplicate of
physics middleware. Future deterministic gameplay calculations should receive unit tests where
practical, and useful regression tests should be added for gameplay bugs.

CI is deliberately deferred for this single-developer prototype. Local validation is sufficient
until automation provides demonstrated value.

## Project Documents

- Long-lived backlog: [docs/roadmap.md](docs/roadmap.md)
- Feature specifications: [docs/specs](docs/specs)
- Initial physics decision and world conventions: [docs/physics.md](docs/physics.md)

The next gameplay experiment will investigate the internal movable-mass / centre-of-mass control
model without reducing it to conventional steering.
