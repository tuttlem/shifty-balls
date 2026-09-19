# Phase 0 Research: Project Foundation and Bevy Bootstrap

## Bevy Version and Rust Toolchain

**Decision**: Use Bevy 0.19.1 and support stable Rust 1.95.0 or newer.

**Rationale**: Bevy 0.19.1 is the current stable patch release. Its version-specific manifest
declares Rust 1.95.0 and edition 2024, which matches the repository edition and gives a concrete
minimum toolchain. The locally installed Rust 1.92.0 must therefore be upgraded before validation.

**Alternatives considered**:

- Bevy 0.20.0-rc.1: rejected because it is a release candidate, not the requested stable choice.
- Bevy's main branch: rejected because its API is not a stable release target.
- An older Bevy release: rejected because it omits current stable fixes without a project reason.

**Sources**:

- [Bevy releases](https://github.com/bevyengine/bevy/releases)
- [Bevy 0.19.1 Cargo manifest](https://raw.githubusercontent.com/bevyengine/bevy/v0.19.1/Cargo.toml)
- [Bevy setup guide](https://bevy.org/learn/quick-start/getting-started/setup/)

## Native 3D Bootstrap

**Decision**: Use Bevy's default plugins and a direct startup setup for the window, one fixed
perspective camera, one light, a generated plane, and a generated sphere.

**Rationale**: The default plugin group provides the concrete native-window, rendering, mesh,
material, camera, light, and lifecycle functionality this feature needs. Generated primitives meet
the visual requirement without external assets. One startup path is clearer than a custom plugin or
module hierarchy.

**Alternatives considered**:

- A custom Bevy feature selection: rejected as premature tuning that can accidentally omit required
  native 3D facilities.
- Custom rendering or engine abstractions: rejected because they hide a simple direct integration
  without satisfying a current requirement.
- External models or art assets: rejected because primitive geometry is sufficient.

**Sources**:

- [Bevy getting started](https://github.com/bevyengine/bevy#getting-started)
- [Version-matched 3D examples](https://github.com/bevyengine/bevy/tree/v0.19.1/examples/3d)
- [Bevy 0.19.1 feature manifest](https://raw.githubusercontent.com/bevyengine/bevy/v0.19.1/Cargo.toml)

## Physics and Scope Boundary

**Decision**: Add no physics library, physics model, gameplay input, or camera-control system.

**Rationale**: The constitution requires deliberate physics selection only after concrete
rolling-ball requirements exist. This feature proves the project and rendering path; a stationary
presentation-only ball does that without prejudging the control model.

**Alternatives considered**:

- Add a Bevy physics integration now: rejected because it makes an irreversible dependency decision
  before the next specification's evaluation.
- Add gravity, rolling, or input now: rejected because those are the next coherent experiment, not
  foundation work.

## Validation and CI

**Decision**: Use local standard Cargo validation and manual scene/lifecycle checks; defer CI.

**Rationale**: cargo check, cargo test, formatting validation, and Clippy provide immediate,
proportionate confidence for a single-developer visual bootstrap. Manual inspection is sufficient
for the intentionally small graphical requirement. CI has no demonstrated incremental value yet and
would add delivery infrastructure before the gameplay experiment.

**Alternatives considered**:

- Screenshot, GPU, or headless rendering tests: rejected because they add disproportionate
  infrastructure to a static visual smoke test.
- CI now: rejected pending a demonstrated need; the roadmap decision item is completed by recording
  this rationale, while CI implementation items remain unchecked.
