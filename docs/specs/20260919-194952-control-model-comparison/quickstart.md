# Control Model Comparison Quickstart

## Automated validation

From the repository root:

```sh
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

Tests cover project-owned selection, intent conversion, TORQUE/FORCE mappings, neutral state, comparison timing/best time, reset, and progression/finish invariants. They do not duplicate rigid-body solver behaviour.

## Manual procedure

1. Run `cargo run` in a native desktop environment.
2. Select `1` for SHIFT and confirm its label before input. Use `F3` to inspect model-specific state.
3. Make three normalised attempts, including a gentle turn, direction change, bank/recovery situation, and input release. Record completion/time if valid.
4. Repeat with `2` TORQUE and `3` FORCE. Rotate model order by round (S/T/F, T/F/S, F/S/T) so learning is not credited to the final model.
5. Verify `R` and changing model reset ball movement, control state, timing, progress, and camera before the new attempt.
6. Record low/high-speed response, predictability, banks, recovery, airborne behaviour, surprises, enjoyment, and whether the difficult course distorted results.
7. Select `4`, complete one First Race through countdown, traffic, finish, and `R` rematch to verify the accepted SHIFT race regression path.

Record a provisional preferred model—or an honest inconclusive result—in `docs/control-model-comparison.md` during implementation.
