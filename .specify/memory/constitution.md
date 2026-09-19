<!--
Sync Impact Report
- Version change: 1.1.0 -> 2.0.0
- Modified principles: I. Fun Over Realism; II. Emergent Gameplay Through Simple Systems;
  V. Deterministic and Testable Simulation; VI. Presentation Must Not Own the Game;
  VII. Playable Progress Over Infrastructure; XV. The Game Must Remain Fun to Build.
- Added principles: II. Physics Is the Control Scheme; III. Momentum Creates the Gameplay;
  V. Collisions Are Gameplay.
- Added sections: none.
- Removed sections: none.
- Follow-up TODOs: none.
-->

# Shifty Balls Constitution

## Core Principles

### I. Fun Over Realism

Shifty Balls is a game first. Physics, terrain, movement, collisions, and related systems MUST
primarily serve enjoyable, understandable gameplay. Real-world physics provides inspiration and
intuition, but values and behaviours MAY be simplified or exaggerated when doing so improves
responsiveness, readability, racing skill, recovery, spectacle, or fun. The game MUST favour
internally consistent, learnable behaviour over scientific accuracy, and MUST NOT sacrifice
satisfying control solely to preserve physical exactness. Deliberate approximations MUST still
allow players to develop useful intuition rather than producing arbitrary behaviour.

### II. Physics Is the Control Scheme

Players MUST influence the ball through understandable physical interactions, not conventional
vehicle-style steering. The primary intended model is manipulation of an internal mass or centre
of mass: player input changes a meaningful physical state, physical balance changes, and the ball
responds through the simulation. Implementations MUST NOT reduce this to arbitrary steering force
or an equivalent spherical-car control scheme disguised as physics. The exact model MAY be
simplified or exaggerated and remains subject to experimentation and playtesting, but it MUST
preserve this causal relationship so players feel they are influencing a physical object.

### III. Momentum Creates the Gameplay

Momentum MUST be a central source of player skill. Speed, gravity, rotation, banking, elevation,
collisions, airborne orientation, surface interaction, and recovery SHOULD combine to create
meaningful racing decisions. Tracks SHOULD let players preserve and redirect momentum, trade
height for speed, climb and descend banks, prepare jumps, recover from poor landings, respond to
collisions, and find faster physical lines. Optimal play MUST NOT primarily be following prescribed
racing lines, activating arbitrary boosts, or reacting to scripted events.

### IV. Emergent Gameplay Through Simple Systems

The game MUST prefer a small set of understandable, composable systems over bespoke gameplay
rules. Gravity, rigid-body motion, rotation, internal-mass position, inertia, friction, surface
properties, collisions, track geometry, and airborne motion SHOULD combine to produce emergent
play. New simulation complexity MUST create observable gameplay possibilities and MUST NOT exist
solely for technical sophistication; prefer composition of existing mechanics to special cases.

### V. Collisions Are Gameplay

Ball-to-ball collisions SHOULD eventually be meaningful physical events rather than cosmetic
contact. Players SHOULD be able to disrupt another ball's line, be moved up or down a bank, lose
or gain useful momentum, recover from contact, and use positioning to survive crowded track
sections. Collision mechanics MUST remain subordinate to proving the fundamental single-ball
control model: neither multiplayer racing nor sophisticated collision play is required for the
initial prototype.

### VI. Code Coherence Over Cleverness

Code MUST optimise for readability, understandability, maintainability, and ease of modification.
Prefer straightforward, explicit Rust. Abstractions MUST earn their existence through real project
requirements: do not introduce speculative abstractions, unjustified trait hierarchies, generics
that only remove small duplication, macros where ordinary Rust is clear, conventional game
patterns without need, excessive module or crate fragmentation, or premature generalisation. A
developer SHOULD be able to understand a subsystem without architectural archaeology.

### VII. Workspace With Purposeful Boundaries

Shifty Balls MUST remain a Rust Cargo workspace. Crates SHOULD exist only when they create a
concrete architectural, dependency, testing, ownership, or reuse boundary; they MUST NOT merely
organise source files or anticipate hypothetical requirements. Prefer a small set of coherent
crates, and extract, combine, split, or remove boundaries when experience demonstrates that it
improves coherence. Individual specifications MAY evolve the workspace without this constitution
fixing crate names or topology.

### VIII. Deterministic and Testable Simulation

Deterministic pure gameplay calculations SHOULD be tested deterministically, and control-model
calculations SHOULD be independently testable where practical. Physics and game-domain behaviour
SHOULD remain separable from rendering, audio, input, and other presentation concerns when that
creates practical testing value. Important gameplay invariants SHOULD have readable tests targeting
observable behaviour, and important bugs SHOULD receive regression coverage when an automated test
can represent them. Physics middleware MAY make perfect cross-platform determinism impractical;
physics behaviour MUST instead be reproducible enough for development and regression testing where
practical. The project MUST NOT build a custom physics engine solely for theoretical determinism.

### IX. Presentation Must Not Own the Game

Bevy MAY naturally own substantial application and rendering state, and framework-specific types
MAY be used where they make implementation clearer. Gameplay and physical concepts SHOULD remain
understandable and testable independently where that has practical value. Contributors MUST NOT
manufacture generic engine interfaces or renderer abstractions solely to hide Bevy, nor couple
every game concept unnecessarily to presentation technology. A direct, understandable integration
boundary is preferred over theoretical framework independence.

### X. Playable Progress Over Infrastructure

Shifty Balls MUST become playable early and remain playable throughout development. After basic
project and rendering foundations exist, work MUST prioritise the smallest interactive test:
ball rolls downhill -> player shifts internal mass -> ball responds physically -> player learns to
exploit the response. Work SHOULD favour visible experiments—ball, slope, gravity, roll, shift
mass, observe response—over infrastructure. The project MUST NOT delay this test for elaborate
asset pipelines, generic physics frameworks, race architecture, networking, saves, configuration,
plugins, procedural tracks, AI, menus, progression, modding, telemetry, replays, or sophisticated
ECS architecture. The project MUST NOT become a physics engine, general-purpose game engine,
architecture experiment, or technology demonstration at the expense of becoming a game.

### XI. Scope Discipline

Ideas are encouraged; implementation scope is constrained. Multiple racers, ball collisions,
large luge tracks, half-pipes, bowls, jumps, funnels, split routes, surface materials, ball and
internal-mass variants, rotational-inertia manipulation, airborne control, AI, multiplayer, track
generation, racing modes, cosmetics, and progression are ideas, not commitments. Interesting
future work SHOULD be recorded in the roadmap rather than implemented immediately, and roadmap
placement MUST NOT be treated as an implementation commitment. Each specification MUST select
coherent, bounded work and MUST NOT opportunistically implement unrelated roadmap items. Work
discovered during implementation SHOULD be recorded for later unless required by the active
specification.

### XII. Roadmap Discipline

`docs/roadmap.md` MUST be the long-lived project backlog. It MUST use Markdown checkboxes for
actionable work, group related work understandably, capture known features, experiments, and ideas,
and distinguish near-term work from speculation where useful. Specifications that satisfy roadmap
acceptance criteria MUST update the corresponding items. Newly discovered ideas SHOULD normally be
added to the roadmap instead of silently expanding the active specification.

### XIII. Timestamped Specifications

Specifications MUST live beneath `docs/specs/` and use timestamp identifiers, not sequential
feature numbers. The directory format is `YYYYMMDD-HHMMSS-feature-name`, using the project's local
development time; for example, `docs/specs/20260905-091846-workspace-setup/`. SpecKit workflow,
feature-creation tooling, templates, and project guidance MUST preserve this convention.
Specifications MUST be independently understandable, explicitly name meaningful dependencies, and
MUST NOT rely on chronological order as an implicit dependency mechanism.

### XIV. Quality Is Part of Completion

Completed work MUST leave the repository healthy unless its specification documents a justified
exception. The workspace MUST compile; relevant tests, formatting, and Clippy MUST pass without
unjustified warnings; affected documentation MUST be accurate; and completed roadmap work MUST be
updated. Warnings, failing tests, TODO implementations, disabled checks, or commented-out code
MUST NOT be used merely to make work appear complete.

### XV. Dependencies Must Earn Their Place

Dependencies SHOULD be added deliberately. Before adding one, contributors MUST consider whether
it solves a meaningful problem better than a small, clear local implementation; this MUST NOT be
used to justify reimplementing mature libraries unnecessarily. Established libraries are preferred
for substantial solved problems, but trivial conveniences do not justify dependencies. Bevy is the
selected game technology, but that decision MUST NOT be used to introduce every available Bevy
ecosystem crate. Physics middleware MUST be chosen deliberately when the first rolling-ball
specification establishes concrete requirements; it MUST NOT be selected automatically merely
because the game needs rigid-body physics. Dependencies that no longer provide value SHOULD be
removed.

### XVI. Comments Explain Intent

Code SHOULD explain mechanics through clear naming and structure. Comments MUST explain intent:
why behaviour exists, non-obvious gameplay decisions, mathematical reasoning, important
invariants, deliberate approximations, or surprising constraints; they MUST NOT merely translate
obvious code into English. Physics code SHOULD document its gameplay model and assumptions enough
for another developer to understand intended behaviour without reconstructing the mathematics.

### XVII. Refactoring Is Normal

Architecture is not sacred. Refactoring SHOULD occur when it materially improves coherence,
readability, testability, or the ability to implement gameplay. Contributors MUST NOT preserve a
poor abstraction merely because an earlier specification created it. A specification MAY include
focused refactoring needed for its feature; unrelated large-scale cleanup SHOULD become separate
roadmap work.

### XVIII. The Game Must Remain Fun to Build

Shifty Balls is a recreational project. Technical decisions SHOULD preserve experimentation,
visible progress, and enjoyment of development. Between otherwise reasonable choices, prefer the
approach that keeps the project understandable and makes gameplay experimentation easier. The
project MUST leave room for spectacular crashes, chaotic multi-ball collisions, ridiculous track
geometry, huge banks and bowls, enormous jumps, near misses, desperate recoveries, and unexpected
but understandable physics interactions; it MUST NOT optimise this personality out in pursuit of
simulation purity or architectural sophistication.

## Project Constraints

Shifty Balls is a 3D physics-driven racing game written in Rust using Bevy. Players race rolling
balls through large downhill courses inspired by luge tracks, half-pipes, bowls, banked surfaces,
ramps, drops, jumps, funnels, and other terrain. Bevy is the current selected game technology;
specifications MUST NOT investigate Bevy against `wgpu`, other engines, or alternative rendering
stacks unless future experience provides a concrete reason to reconsider that decision. These
constraints describe the project context, not a mandate for a particular physics model, physics
library, ECS architecture, or crate topology. The initial kill test is whether manipulating one
ball through a simple downhill environment is satisfying. If it is not, contributors MUST improve
or replace the control model before expanding scope; adding opponents, race rules, menus,
progression, procedural tracks, power-ups, weapons, cosmetics, AI, multiplayer, championship
systems, or polished assets will not resolve the fundamental problem.

## Development Workflow and Quality Gates

Specifications MUST define bounded, independently understandable work and document meaningful
dependencies explicitly. Before creating a feature specification, contributors MUST create and
switch to a dedicated feature branch. Specification, planning, tasking, implementation, and
feature-specific documentation work MUST occur only on that feature branch until the feature is
integrated. Before considering a specification complete, contributors MUST review it and its
implementation for compliance with this constitution, update roadmap items whose acceptance
criteria were satisfied, and record newly discovered nonessential work in `docs/roadmap.md`.
Quality checks MUST be proportionate to the change but may not be bypassed merely for convenience.
Exceptions require an explicit, justified statement in the relevant specification.

## Governance

This constitution governs all Shifty Balls specifications and implementation work and supersedes
conflicting project practices. A proposed implementation that conflicts with it MUST change,
unless the project's principles themselves have genuinely changed and the constitution is
intentionally amended and documented. Compliance review is required when creating specifications,
reviewing implementation plans, and declaring work complete.

Amendments MUST state the reason and update the Sync Impact Report. Governance changes use semantic
versioning: MAJOR for backward-incompatible removals or redefinitions of principles, MINOR for new
principles or materially expanded guidance, and PATCH for clarifications, wording, or other
non-semantic refinements. When principles conflict, apply them in this order: fun and satisfying
physical control; understandable and learnable physical behaviour; playable progress; code
coherence and maintainability; scope discipline; correctness and reproducibility where practical;
architectural elegance; then physical realism. The simplest interpretation consistent with these
principles prevails; the constitution guides engineering judgement rather than creating
bureaucracy.

**Version**: 2.0.0 | **Ratified**: 2026-09-05 | **Last Amended**: 2026-09-19
