# Quickstart: Validate Physical Racing Playtest

## Prerequisites

- Rust 1.95.0 or newer with the repository-requested components.
- A native Linux, macOS, or Windows desktop graphics environment.

Read the [feature specification](spec.md), [data model](data-model.md), and
[desktop contract](contracts/desktop-physical-race.md) before validation.

## Automated validation

From the repository root, run:

```sh
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

Focused tests cover project-owned course/gate geometry and any changed
route/order/rematch invariants. They do not reproduce third-party multi-body
physics contacts.

## Manual three-race procedure

1. Run `cargo run`, press `4`, and confirm four distinct racers at a common
   staged start. During 3–2–1, hold movement input and confirm no meaningful
   early launch occurs.
2. At GO, drive through opening traffic and observe at least one ordinary
   ball-to-ball contact. Confirm it visibly changes line or momentum without a
   special collision control.
3. Enter the designated wide racing section. Try both available physical lines
   across the three races. Confirm the player can identify a forward direction,
   a recovery direction after contact, and the displayed position.
4. In each race, deliberately accept or create one ordinary traffic
   displacement while remaining in course bounds. Continue using normal SHIFT
   controls and record whether the player can rejoin the ordered route and
   finish without `R`.
5. Record every post-start order change observed in the racing section. Identify
   whether it arose from line choice, momentum, or contact; do not count a
   countdown/grid ordering change as a pass.
6. Complete three consecutive races. After each finish, press `R` and verify
   that the start, motion, progression, timer, camera, and result state reset
   before the next countdown.
7. Record affirmative, negative, or unresolved findings in `docs/race.md` for:
   collision readability, recovery, overtaking, line choice, camera readability,
   fairness, and player-skill influence. Negative findings are a valid result.

## Expected evidence threshold

- Each of the three races contains at least one ordinary contact and finishes
  without a developer-only reset.
- At least two races show a post-start order change in the racing section.
- At least two traffic displacements are recovered through normal controls and
  lead to a valid finish.
- The observation record explicitly states whether the physical-racing slice
  advances Milestone F or needs another bounded tuning pass.
