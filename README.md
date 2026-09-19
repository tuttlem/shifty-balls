# Shifty Balls

Shifty Balls is a 3D physics-driven racing game prototype. The eventual player control mechanic
will influence a rolling ball by shifting its internal mass rather than steering it like a vehicle.

The project is currently at its rendering foundation: it opens a 3D scene containing a reference
surface and a visual-only future player ball. There is no physics, input, rolling, or gameplay
control yet.

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

The application window is titled **Shifty Balls**. It shows a lit ground/reference surface and one
stationary sphere, then exits normally when closed.

## Validate

```sh
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

There is intentionally little ordinary game logic to test at this stage. Future deterministic
gameplay calculations should receive unit tests where practical, and useful regression tests should
be added for gameplay bugs.

CI is deliberately deferred for this single-developer prototype. Local validation is sufficient
until automation provides demonstrated value.

## Project Documents

- Long-lived backlog: [docs/roadmap.md](docs/roadmap.md)
- Feature specifications: [docs/specs](docs/specs)

The next feature will deliberately choose a physics approach before adding gravity, rolling, or the
internal-mass control model.
