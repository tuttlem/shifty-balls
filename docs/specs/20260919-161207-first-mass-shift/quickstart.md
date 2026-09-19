# Quickstart: Validate the First Internal Mass Shift

## Prerequisites

- Rust 1.95.0 and repository-requested components.
- A supported native desktop graphics environment.

Read the [desktop contract](contracts/desktop-mass-shift.md), [data model](data-model.md), and [research decision](research.md).

## Automated validation

~~~text
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
~~~

Tests cover only project-owned intent, clamp, interpolation, ratio, and frame
conversion. They do not reproduce third-party physics.

## Manual experiment

1. Run cargo run and allow normal downhill rolling.
2. Observe centre, current mass, target mass, and boundary.
3. Hold W, A, S, and D separately; verify documented world directions.
4. Hold at least four diagonals; verify normalised, bounded targets.
5. Release input; verify target and mass return continuously toward centre.
6. Run ten paired downhill trials from the same start: hold A in one and D in
   the other. Record whether at least eight pairs have distinguishable,
   directionally opposite influence.
7. Compare W and S with no input; document whether each response is observable
   and useful.
8. Confirm shell rotation does not make the display unreadable and close
   normally.

## Boundary review

Confirm no direct velocity write, desired orientation, steering force/torque,
trajectory snap, traction aid, second ball, or non-goal system has appeared.
Record weak or unintuitive results rather than hiding them with steering.
