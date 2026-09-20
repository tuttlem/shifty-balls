# Quickstart: Validate Half-Pipe Racing Section

## Automated validation

From the repository root:

```sh
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

Tests cover project-owned strip/gate geometry and current route/reset invariants; they do not duplicate rolling or multi-ball physics.

## Manual three-race procedure

1. Run `cargo run`, press `4`, and confirm the unchanged four-racer countdown and fair release.
2. In race one use the low half-pipe line; exit, progress, and finish without `R`.
3. In race two climb a higher line; record its visible height, momentum, exit, traffic, or place difference.
4. In race three try an in-bounds poor entry, wall climb, low-speed, or traffic displacement. Use only normal SHIFT input to find the exit/recovery route.
5. Across all races observe traffic, side-by-side running, or an order change near the pipe; confirm valid ordered progression and finish.
6. Check camera/readout at the high line and after contact. Press `R` after each race to confirm a fair rematch.
7. Record affirmative, negative, or unresolved usefulness, line, momentum, recovery, traffic, camera, and replay findings in `docs/course.md` and `docs/race.md`.

**Evidence threshold**: three successful exits/finishes, one low/high outcome, two traffic/side-by-side situations, two normal-control recoveries, and an explicit owner decision on half-pipe usefulness.
