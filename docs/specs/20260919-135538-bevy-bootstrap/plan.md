# Implementation Plan: Project Foundation and Bevy Bootstrap

**Branch**: feature/bevy-bootstrap | **Date**: 2026-09-19 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from docs/specs/20260919-135538-bevy-bootstrap/spec.md

## Summary

Create the smallest trustworthy Shifty Balls desktop foundation: one direct Bevy application that
opens a titled window and renders a lit, static ground plane and ball sphere. Use Bevy 0.19.1 with
its default native features and Rust 1.95.0 or newer on the stable channel. Keep the root as the
only application crate and use generated primitives; introduce neither physics nor gameplay input,
architecture, assets, or future-facing dependencies.

## Technical Context

**Language/Version**: Rust edition 2024; stable Rust 1.95.0 or newer. The currently installed
Rust 1.92.0 is insufficient and must be upgraded before implementation validation.

**Primary Dependencies**: Bevy 0.19.1 only, using its default native-window and 3D rendering
features.

**Storage**: N/A

**Testing**: cargo test for ordinary Rust logic; manual desktop launch, scene, responsiveness, and
shutdown validation. No screenshot, GPU, or headless rendering test infrastructure.

**Target Platform**: Native Linux, macOS, and Windows developer desktops with a usable graphics and
display environment. Web and headless targets are out of scope.

**Project Type**: Single-crate desktop application in a minimal Cargo workspace.

**Performance Goals**: A static scene remains normally responsive to desktop lifecycle events; no
frame-rate benchmark is required for this foundation.

**Constraints**: cargo run from the repository root; title Shifty Balls; fixed perspective view;
one lit flat reference surface and one stationary sphere; no external assets, physics, gameplay
input, camera controls, or additional Bevy ecosystem crates.

**Scale/Scope**: One developer, one application crate, one static scene, and no persistent data,
network interface, or public API.

## Constitution Check

**Pre-design gate: PASS**

- **Fun and playable progress**: Pass. The work is the constitution's required rendering foundation
  before the first rolling-ball experiment, and stops at the deliberately minimal visual proof.
- **Physics as control / momentum / collisions**: Pass. No physics model, steering surrogate,
  collision system, or control is selected or implemented.
- **Code coherence and purposeful workspace boundaries**: Pass. The design retains one application
  crate and direct Bevy setup; a root Cargo workspace contains that single crate without creating
  a future-facing boundary.
- **Presentation boundary**: Pass. Bevy types are used directly for the purely visual scene; no
  framework-hiding abstraction is introduced.
- **Dependencies**: Pass. Bevy 0.19.1 is the only new dependency. Default features are justified
  by the concrete window, renderer, mesh, material, camera, and lighting requirements. Physics
  middleware and all unrelated dependencies remain excluded.
- **Determinism and testing**: Pass. No simulation exists. Standard unit-test conventions and
  manual visual validation are proportionate; future deterministic gameplay remains separately
  testable when it exists.
- **Scope, roadmap, and timestamps**: Pass. The feature remains bounded to foundation work, keeps
  later roadmap ideas unchecked, normalizes the roadmap to docs/roadmap.md, and uses the
  timestamped specification convention.
- **Quality and documentation**: Pass. The plan requires documented toolchain and validation
  commands plus compilation, tests, formatting, and Clippy without disabled checks.

**Post-design gate: PASS** — Phase 1 introduces only documentation artifacts and the
human-facing desktop-scene contract. It adds no complexity, dependencies, or scope expansion.

## Project Structure

### Documentation (this feature)

~~~text
docs/specs/20260919-135538-bevy-bootstrap/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── desktop-scene.md
└── tasks.md             # Created later by the task workflow
~~~

### Source Code (repository root)

~~~text
Cargo.toml             # Root package and minimal workspace; Bevy dependency
src/
└── main.rs            # Direct application setup and static scene startup
README.md              # Setup, run, validation, and project-status guidance
.gitignore             # Build, editor, and platform artifacts
docs/
├── roadmap.md         # Long-lived backlog; normalized from ROADMAP.md
└── specs/
    └── 20260919-135538-bevy-bootstrap/
        ├── spec.md
        ├── plan.md
        ├── research.md
        ├── data-model.md
        ├── quickstart.md
        └── contracts/
            └── desktop-scene.md
~~~

**Structure Decision**: Retain the root binary package as the single workspace member. Keep scene
creation in src/main.rs because one startup path does not justify modules or plugins. Add a tests
directory only when testable project logic warrants it.
