# Quickstart: Validate the Bevy Bootstrap

## Prerequisites

- A native Linux, macOS, or Windows desktop with a working graphics/display environment.
- Rust 1.95.0 or newer on the stable channel. The repository's currently observed Rust 1.92.0 is
  not sufficient for Bevy 0.19.1.

Install or update the stable toolchain with Rustup if needed:

~~~sh
rustup update stable
~~~

## Validate the Project

From the repository root, run:

~~~sh
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
~~~

Each command must exit successfully. The initial project may have little or no ordinary Rust logic
to test; that is expected and does not justify adding graphical test infrastructure.

## Run the Application

~~~sh
cargo run
~~~

Verify the [desktop scene contract](contracts/desktop-scene.md):

1. A desktop window opens with the exact title **Shifty Balls**.
2. The initial perspective view visibly contains a lit flat ground/reference surface and exactly one
   ball sphere.
3. The sphere remains stationary and no gameplay input changes the scene.
4. The application remains responsive.
5. Close the window normally and confirm the process exits cleanly.

## Scope Review

Review Cargo.toml and the source before completion:

- Bevy 0.19.1 is the only new runtime dependency.
- No physics library or other Bevy ecosystem crate is present.
- No physics, ball control, track system, gameplay camera, external assets, or race system has been
  introduced.

Then update the root README and normalize the roadmap from docs/ROADMAP.md to docs/roadmap.md,
preserving its content and checking only items evidenced by this feature.
