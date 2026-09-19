# Shifty Balls

Shifty Balls is a 3D physics-driven racing game prototype. The eventual player control mechanic
will influence a rolling ball by shifting its internal mass rather than steering it like a vehicle.

The project is now at its first-race vertical slice: one human ball and three physical AI balls
share a compact, hand-authored course. Every racer moves by shifting an internal mass; gravity and
collision contact, not vehicle steering, determine the response. The initial single-ball result
was promising, although the course remains demanding and first-race traffic observations are still
awaiting playtest.

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

The application window is titled **Shifty Balls**. It stages four distinct balls, counts down,
then releases them through a compact course with bends, banks, a crest, progression regions, and a
finish gate. The race display shows countdown, player position, and time; gravity and rigid-body
contact move every ball.

## Temporary Experiment Controls

- `W`: shift the internal mass down-track (+Z)
- `S`: shift it up-track (-Z)
- `A`: shift it track-left (-X)
- `D`: shift it track-right (+X)
- Combined keys request a normalised diagonal; releasing every key returns the target to centre.
- `R`: reset every racer and begin a fresh race countdown.
- `F3`: toggle the human-only development mass and velocity display.

The control model is experimental. It does not set velocity, turn the ball, or apply steering
forces. See [the mass-shift experiment](docs/mass-shift.md) for the current approximation,
tuning values, display legend, and trial procedure. See [the first course](docs/course.md) for the
kill-test layout, timing, and observation record, and [the first race](docs/race.md) for the race
lifecycle, progression model, and current traffic observations.

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
- First-race vertical slice: [docs/race.md](docs/race.md)

The next decision comes from repeated physical races: improve traffic, control, or course evidence
if it is weak, rather than adding racing content to compensate.
