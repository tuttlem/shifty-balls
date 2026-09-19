# Shifty Balls

Shifty Balls is a 3D physics-driven racing game prototype. The eventual player control mechanic
will influence a rolling ball by shifting its internal mass rather than steering it like a vehicle.

The project has completed its first-course kill test: one ball rolls through a compact,
hand-authored course while the player moves a visible conceptual weight inside it. The weight
changes the ball's centre of mass; gravity and collision contact, not vehicle steering, determine
the response. The initial result is promising, although the first course is currently too difficult
to serve as a gentle introduction. There is no race system or production track yet.

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

The application window is titled **Shifty Balls**. It shows one ball, a compact course with bends,
banks, a crest, and a finish gate, plus a development display for mass state, speed, and timing.
Gravity and rigid-body contact move the ball; closing the window exits normally.

## Temporary Experiment Controls

- `W`: shift the internal mass down-track (+Z)
- `S`: shift it up-track (-Z)
- `A`: shift it track-left (-X)
- `D`: shift it track-right (+X)
- Combined keys request a normalised diagonal; releasing every key returns the target to centre.
- `R`: immediately restart the current course attempt while retaining the session-best time.
- `F3`: toggle the development mass, velocity, and timing display.

The control model is experimental. It does not set velocity, turn the ball, or apply steering
forces. See [the mass-shift experiment](docs/mass-shift.md) for the current approximation,
tuning values, display legend, and trial procedure. See [the first course](docs/course.md) for the
kill-test layout, timing, and observation record.

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
- Current internal-mass experiment: [docs/mass-shift.md](docs/mass-shift.md)
- First-course kill test: [docs/course.md](docs/course.md)

The next decision comes from the course playtest: improve the control model if it is weak, rather
than adding racing content to compensate.
