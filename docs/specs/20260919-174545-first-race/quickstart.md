# Quickstart: Validate the First Race

## Prerequisites

- Rust 1.95.0 or newer, with the repository-requested components.
- A supported native desktop graphics environment.

Read the [feature specification](spec.md), [race data model](data-model.md), and [desktop race contract](contracts/desktop-race.md) before validation.

## Automated validation

    cargo check
    cargo test
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings

Focused tests cover project-owned lifecycle/countdown rules, progression order, finish eligibility and order, race position comparison, timing, and rematch reset. They do not reproduce third-party physics contacts.

## Manual race validation

1. Run cargo run. Confirm the start contains one recognisable human ball and three visibly different opponents without overlap.
2. Observe 3–2–1–GO. Hold W/A/S/D during the countdown and confirm neither the player nor opponents gain a meaningful early launch; confirm time starts at GO.
3. Race through the opening and banks. Confirm the human mass display still explains the player’s requested/current mass when F3 is enabled.
4. Observe at least one ball-to-ball collision. Check that it changes real lines or momentum without a combat control, and that continued internal-mass control can attempt recovery.
5. Watch the player position while racers pass ordered course regions. Deliberately reach the finish area before completing the route if practical and confirm it does not produce a finish.
6. Complete a valid player finish. Confirm fixed place, elapsed time, and win/loss result agree with the observed racer order.
7. Press R during countdown, active racing, and after a player result. Confirm each rematch restores four fair starts, clears state, and begins a fresh countdown.
8. Repeat at least three starts. Record whether AI creates useful traffic, handles ordinary contact credibly, and makes the player care about position. Record failures honestly for the next collision-focused milestone.

## Expected first-race observations

- Opponent control remains visibly tied to moving an internal mass; no opponent receives direct steering or artificial catch-up speed.
- Traffic can create line disruption and overtaking opportunities, but collision tuning and advanced AI remain out of scope.
- The existing course may reveal width, bottleneck, camera, or recovery problems. Record them; do not hide them with extra content.
