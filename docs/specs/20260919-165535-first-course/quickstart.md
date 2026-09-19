# Quickstart: Validate the First Course Kill Test

## Prerequisites

- Rust 1.95.0 and the repository-requested components.
- A supported native desktop graphics environment.

Read the [desktop course contract](contracts/desktop-course.md), [data
model](data-model.md), and [research decisions](research.md) before testing.

## Automated validation

```text
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

Tests cover project-owned timing, best-time, finish, restart, and any extracted
course helper calculations. They do not duplicate third-party physics contact
simulation.

## Manual kill test

1. Run `cargo run` and identify the current run time, best time, speed, mass
   display, route, and finish before moving.
2. Let the opening descent establish momentum; try W/A/S/D deliberately and
   verify the gentle bend teaches a useful lateral line change.
3. Take intentionally high and low lines through the stronger bank in at least
   three paired attempts; note path, height, speed, and time differences.
4. Try to preserve momentum through the compression and crest, then repeat
   after a deliberately poor bank exit.
5. Make one common error—such as climbing too high or entering off-centre—and
   attempt recovery without R before deciding it is unrecoverable.
6. Press R while rolling, stalled, after a finish, and if the ball leaves the
   useful course; confirm each case restores one fair start and retains best.
7. Complete at least two runs; verify completion is obvious, time freezes, and
   best changes only for the faster run.
8. Confirm the camera maintains a readable world-up horizon and enough early
   view for every course decision. If F3 exists, confirm it does not alter
   gameplay.
9. Record all ten kill-test questions in the course documentation as yes, no,
   or unresolved. Do not proceed to racing solely because the course completes.
