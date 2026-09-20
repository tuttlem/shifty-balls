# Shifty Balls

Shifty Balls is a 3D physics-driven racing game prototype. SHIFT—the original control model—moves
an internal mass rather than steering like a vehicle. The project now also has a deliberately
bounded post-race comparison between SHIFT, rotational TORQUE, and translational FORCE.

The accepted First Race remains available: one human ball and three physical SHIFT AI balls share
a compact hand-authored course. Its current Physical Racing Playtest widens and softens the known
traffic bottleneck into a high/low-line recovery section, while retaining ordinary physics and
SHIFT-only human/AI control. By default, the application starts a human-only comparison attempt
on that same course so its control alternatives can be evaluated without traffic contamination.

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

The application window is titled **Shifty Balls**. It starts a single-ball comparison attempt on
the compact course. Select `4` to restore the accepted four-ball race, with its countdown,
progression, position, finish result, and rematch loop.

## Temporary Experiment Controls

- `1`: start a normalised single-ball SHIFT comparison attempt.
- `2`: start a normalised single-ball TORQUE comparison attempt.
- `3`: start a normalised single-ball FORCE comparison attempt.
- `4`: enter/reset the accepted First Race path.
- `W`/`A`/`S`/`D`: request down-track/left/up-track/right in stable world coordinates; diagonals are normalised.
- `R`: reset the active comparison attempt or start a fresh race countdown.
- `F3`: toggle development-facing model and velocity visualisation.

The control comparison is experimental. No model sets a desired velocity, orientation, or path:
SHIFT changes centre of mass, TORQUE applies rotational physics input, and FORCE applies a
centre-of-mass physical force. See [the comparison record](docs/control-model-comparison.md) for
the temporary controls, tuning, procedure, and current decision state. See [the first race](docs/race.md)
for the preserved SHIFT race lifecycle.

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
- Post-First-Race control comparison: [docs/control-model-comparison.md](docs/control-model-comparison.md)

The next decision comes from the three-race physical-racing playtest: establish whether traffic,
recovery, and passing are readable and skillful before adding broader track language or AI racing
behaviour.

The owner has signed off this physical-racing pass after successful repeated
races. The next work should continue through bounded track-language or race
experience experiments rather than adding hidden traffic assistance.
