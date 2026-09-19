# Shifty Balls Roadmap

Shifty Balls is a 3D physics-driven racing game built around rolling balls, momentum, gravity, collisions, unusual track geometry, and an unconventional control system.

The player does **not** directly steer the ball like a vehicle.

Instead, the player influences the physical behaviour of the ball by shifting an internal mass / centre of mass. The resulting imbalance changes how the ball rolls, climbs banks, changes direction, rotates, behaves in the air, lands, recovers, and responds to the environment.

The central question behind the project is deliberately simple:

**Is influencing a rolling ball through its internal physics fun enough to build a racing game around?**

The roadmap is intentionally much broader than committed implementation scope.

Items recorded here represent ideas, experiments, milestones, possible features, and areas worth investigating. Inclusion does **not** imply a commitment to implement every item.

Individual specifications should select coherent, bounded pieces of work from this roadmap.

Completed work should be checked off only when its acceptance criteria have genuinely been satisfied.

---

# Near-Term Milestones

These milestones represent the current intended development path.

* [ ] Establish the Rust/Bevy project foundation.
* [ ] Open a Bevy game window.
* [ ] Render a minimal 3D scene.
* [ ] Render a sphere representing the player ball.
* [ ] Select an appropriate initial rigid-body physics approach.
* [ ] Place the ball under gravity.
* [ ] Create a simple slope or half-pipe.
* [ ] Allow the ball to roll freely through physical simulation.
* [ ] Establish useful follow-camera behaviour.
* [ ] Implement an initial internal movable mass / centre-of-mass model.
* [ ] Allow player input to shift the internal mass.
* [ ] Produce an observable physical response from shifting the mass.
* [ ] Tune the response until influencing the ball is understandable.
* [ ] Build a small prototype course requiring deliberate mass shifting.
* [ ] Determine whether the core mechanic is genuinely enjoyable.
* [ ] Only then introduce multiple racing balls.

The first major gameplay target is:

**ball rolls downhill → player shifts internal mass → ball responds physically → player learns to exploit the response**

This is the project's initial kill test.

Do not bury it beneath racing infrastructure.

---

# 1. Project Foundation

## Repository and Workspace

* [ ] Establish Shifty Balls as a Rust Cargo project/workspace.
* [ ] Establish the minimum useful crate structure.
* [ ] Keep workspace boundaries based on demonstrated needs.
* [ ] Ensure the repository is easy to understand and navigate.
* [ ] Add appropriate `.gitignore`.
* [ ] Add a useful root `README.md`.
* [ ] Document how to build and run the project.
* [ ] Document how to run tests and quality checks.
* [ ] Establish workspace-wide development commands where useful.

## Rust Tooling

* [ ] Establish supported Rust toolchain policy.
* [ ] Configure `rustfmt`.
* [ ] Configure Clippy.
* [ ] Establish workspace-wide build validation.
* [ ] Establish workspace-wide test validation.
* [ ] Establish workspace-wide lint validation.
* [ ] Treat warnings intentionally rather than ignoring them.
* [ ] Avoid unnecessary dependencies during foundation work.

## Bevy Bootstrap

Bevy is already the selected game technology.

A separate engine-selection exercise is not required.

* [ ] Add the appropriate Bevy dependency.
* [ ] Establish the smallest useful Bevy application.
* [ ] Open a desktop game window.
* [ ] Configure an appropriate application/window title.
* [ ] Establish a basic 3D scene.
* [ ] Add a perspective camera.
* [ ] Add basic lighting.
* [ ] Render a ground/reference surface.
* [ ] Render a sphere representing the future player ball.
* [ ] Ensure the application exits cleanly.
* [ ] Document how to launch the game.

Do not introduce physics merely as part of the Bevy bootstrap unless required by a separate specification.

## Testing Foundation

* [ ] Establish unit-testing conventions.
* [ ] Establish integration-testing conventions where useful.
* [ ] Establish deterministic testing patterns for pure gameplay calculations.
* [ ] Introduce shared testing infrastructure only when duplication demonstrates a need.
* [ ] Establish the expectation that discovered gameplay bugs receive regression tests where practical.

## Documentation

* [ ] Maintain `docs/roadmap.md`.
* [ ] Maintain `docs/specs/`.
* [ ] Preserve timestamp-based specification naming.
* [ ] Document relevant developer workflow guidance.
* [ ] Keep architectural documentation lightweight and current.

## Automation / CI

* [ ] Decide whether CI currently provides sufficient value.
* [ ] Add automated build validation if justified.
* [ ] Add automated tests if justified.
* [ ] Add formatting validation if justified.
* [ ] Add Clippy validation if justified.

Do not create elaborate delivery infrastructure for a recreational prototype.

---

# 2. Physics Technology

Physics is central to Shifty Balls and deserves an explicit decision.

The objective is not maximum physical realism.

The objective is sufficiently robust rigid-body behaviour that makes rolling, banking, jumping, landing, collisions, friction, and centre-of-mass manipulation enjoyable and understandable.

## Requirements

* [ ] Define immediate rigid-body requirements.
* [ ] Support dynamic spherical rigid bodies.
* [ ] Support gravity.
* [ ] Support collision with static track geometry.
* [ ] Support ball-to-ball collision eventually.
* [ ] Support friction.
* [ ] Support restitution/bounce.
* [ ] Support angular velocity and rotational motion.
* [ ] Support impulses/forces where required.
* [ ] Determine how an offset centre of mass can be represented.
* [ ] Determine whether compound colliders or equivalent mechanisms are useful.
* [ ] Determine whether runtime mass-property modification is practical.
* [ ] Consider physics debugging/visualisation support.
* [ ] Consider Bevy integration quality.
* [ ] Consider maintainability and dependency cost.

## Physics Approach Evaluation

* [ ] Evaluate suitable Bevy-compatible physics libraries.
* [ ] Evaluate whether an existing physics library provides the required mass-property behaviour.
* [ ] Evaluate whether any small amount of custom physical modelling is required for the internal-mass mechanic.
* [ ] Avoid implementing a general-purpose physics engine.
* [ ] Record the selected approach and reasoning.

The physics library is infrastructure for the game.

It must not dictate the game design.

---

# 3. World and Coordinate Conventions

Establish a consistent world model before track and movement systems become complicated.

* [ ] Define world axes.
* [ ] Define which axis represents vertical.
* [ ] Define forward direction conventions.
* [ ] Define world units.
* [ ] Define velocity units.
* [ ] Define angular conventions where relevant.
* [ ] Define gravity direction and magnitude conventions.
* [ ] Define track-local versus world-space concepts where useful.
* [ ] Define ball radius conventions.
* [ ] Define reasonable world scale.
* [ ] Ensure rendering and physics agree about scale and orientation.
* [ ] Document conventions sufficiently for future systems.

Avoid creating unnecessary coordinate abstraction layers.

---

# 4. First Rolling Ball

The first physics milestone should prove that a ball behaves satisfyingly before player control exists.

## Ball

* [ ] Create a dynamic spherical rigid body.
* [ ] Give it appropriate mass.
* [ ] Give it an appropriate collider.
* [ ] Apply gravity.
* [ ] Allow natural rotational motion.
* [ ] Allow the ball to come to rest where appropriate.
* [ ] Ensure visual orientation reflects physical rotation where visible/useful.

## Test Track

* [ ] Create a simple inclined surface.
* [ ] Allow the ball to roll down it.
* [ ] Add a simple curved/banked section if useful.
* [ ] Prevent immediate accidental escape from the test area.
* [ ] Keep geometry deliberately primitive.
* [ ] Avoid building a production track system.

## Physical Feel

* [ ] Establish initial friction.
* [ ] Establish initial restitution.
* [ ] Establish initial damping values if required.
* [ ] Observe acceleration under gravity.
* [ ] Observe rolling behaviour.
* [ ] Observe sliding versus rolling.
* [ ] Observe transition onto different slopes.
* [ ] Establish a reasonable initial speed range.

The goal is not final tuning.

The goal is:

**put ball on hill → ball convincingly becomes somebody else's problem**

---

# 5. Internal Mass Model

This is the defining gameplay system.

It should receive focused experimentation rather than being buried inside general player-control code.

## Conceptual Model

* [ ] Represent an internal movable mass.
* [ ] Define its relationship to the outer ball.
* [ ] Define allowed movement range.
* [ ] Define how quickly the mass can move.
* [ ] Define whether movement is continuous or constrained.
* [ ] Define whether the mass has meaningful inertia.
* [ ] Determine how the shifted mass changes the ball's physical behaviour.
* [ ] Keep the relationship understandable to the player.

## Centre-of-Mass Behaviour

Investigate approaches such as:

* [ ] Runtime centre-of-mass offset.
* [ ] Compound rigid body with internal mass representation.
* [ ] Physically modelled internal body/constraint.
* [ ] Controlled torque resulting from mass displacement.
* [ ] Simplified gameplay approximation preserving understandable physical causality.

The implementation does not need to be physically exact.

It MUST preserve the conceptual relationship:

**mass moves → balance changes → ball responds**

Avoid disguising conventional steering as centre-of-mass physics.

---

# 6. Player Control

Player control should influence physics rather than directly command movement.

## Basic Input

* [ ] Read directional player input.
* [ ] Map input into desired internal-mass displacement.
* [ ] Visualise current mass displacement during development.
* [ ] Clamp displacement to allowed physical range.
* [ ] Return or transition the mass appropriately when input changes.
* [ ] Tune input responsiveness.

## Control Behaviour

* [ ] Shift mass forward/backward.
* [ ] Shift mass left/right.
* [ ] Support combined directional displacement.
* [ ] Evaluate whether vertical/internal radial movement is useful.
* [ ] Evaluate whether control should be camera-relative.
* [ ] Evaluate whether control should be ball-relative.
* [ ] Evaluate whether control should be track-relative.
* [ ] Ensure control remains understandable while the ball rotates.

## Input Devices

Initially:

* [ ] Keyboard support.
* [ ] Mouse only where genuinely useful.

Later:

* [ ] Gamepad analogue-stick support.
* [ ] Controller vibration/haptics if useful.
* [ ] Input rebinding.

Analogue control may eventually become the preferred control method if the mechanic benefits from continuous mass positioning.

---

# 7. Core Control Kill Test

Before substantial racing systems are built, answer whether Shifty Balls is actually fun.

Create a deliberately small prototype environment that requires the player to influence the ball.

Potential elements:

* [ ] Straight downhill section.
* [ ] Gentle bank.
* [ ] Strong bank.
* [ ] Direction change.
* [ ] Small half-pipe.
* [ ] Rise requiring momentum.
* [ ] Drop.
* [ ] Small jump.
* [ ] Recovery after poor positioning.

Evaluate:

* [ ] Can the player predict how shifting mass affects the ball?
* [ ] Does improved technique produce visibly improved results?
* [ ] Can the player intentionally climb a bank?
* [ ] Can the player intentionally descend a bank?
* [ ] Can height be traded for speed?
* [ ] Can momentum be preserved through turns?
* [ ] Can mistakes be recovered from?
* [ ] Does control remain interesting after several minutes?
* [ ] Is getting faster satisfying?
* [ ] Does the player feel responsible for success and failure?

If the answer is no, iterate on the control model.

Do not solve an unsatisfying core mechanic by adding content.

---

# 8. Camera

The camera is particularly important because the player must understand speed, slope, banking, and lateral position.

## Basic Follow Camera

* [ ] Follow the player ball.
* [ ] Maintain useful visibility ahead.
* [ ] Avoid excessive camera rotation caused by ball rotation.
* [ ] Smooth movement appropriately.
* [ ] Preserve a strong sense of speed.
* [ ] Preserve track readability.
* [ ] Handle steep downhill sections.
* [ ] Handle steep uphill sections.
* [ ] Handle strong banks.
* [ ] Handle jumps.
* [ ] Handle landings.

## Camera Orientation

Investigate:

* [ ] World-relative horizon.
* [ ] Track-relative horizon.
* [ ] Velocity-relative orientation.
* [ ] Hybrid orientation.
* [ ] Look-ahead based on velocity.
* [ ] Look-ahead based on expected track direction.

## Camera Dynamics

Later possibilities:

* [ ] Speed-dependent distance.
* [ ] Speed-dependent field of view.
* [ ] Impact shake.
* [ ] Landing response.
* [ ] Collision response.
* [ ] Airborne framing.
* [ ] Recovery camera.
* [ ] Spectator/replay cameras.

Camera effects must preserve readability and avoid unnecessary nausea.

---

# 9. Track Geometry

Tracks should exploit rolling-ball physics rather than resemble ordinary roads.

## Basic Track Elements

* [ ] Straight downhill section.
* [ ] Gentle curve.
* [ ] Banked curve.
* [ ] Half-pipe.
* [ ] Bowl.
* [ ] Funnel.
* [ ] Ramp.
* [ ] Jump.
* [ ] Drop.
* [ ] Crest.
* [ ] Compression.
* [ ] Narrow channel.
* [ ] Wide open section.
* [ ] Wall ride.
* [ ] Corkscrew.
* [ ] Loop only if physics and camera make it enjoyable.

## Track Philosophy

Track geometry should create choices involving:

* [ ] momentum preservation;
* [ ] height versus speed;
* [ ] high versus low lines;
* [ ] risk versus stability;
* [ ] collision exposure;
* [ ] jump positioning;
* [ ] landing preparation;
* [ ] overtaking opportunities;
* [ ] alternate routes.

Avoid simply recreating conventional car-racing circuits inside a tube.

The track itself should be a physical playground.

---

# 10. Track Representation

Do not build a sophisticated track editor prematurely.

## Initial Representation

* [ ] Establish a simple way to construct prototype tracks.
* [ ] Support connected track sections.
* [ ] Ensure collision geometry matches rendered geometry sufficiently.
* [ ] Support banked and curved surfaces.
* [ ] Support broad half-pipe-style geometry.
* [ ] Establish start position.
* [ ] Establish finish region.
* [ ] Establish track bounds where appropriate.

## Later Track Construction

Potential future work:

* [ ] Reusable track segments.
* [ ] Parametric curves.
* [ ] Spline-based track generation.
* [ ] Procedural mesh generation.
* [ ] Track validation.
* [ ] Authoring tools.
* [ ] External track format.
* [ ] In-game track editor.

Introduce these only when manually building tracks becomes a real bottleneck.

---

# 11. Surface Physics

Different surfaces could create substantial gameplay variety without changing the core control scheme.

## Surface Properties

Potential properties include:

* [ ] friction;
* [ ] rolling resistance;
* [ ] restitution;
* [ ] grip;
* [ ] damping;
* [ ] roughness as a gameplay parameter.

## Example Surfaces

* [ ] Smooth concrete.
* [ ] Rough concrete.
* [ ] Metal.
* [ ] Rubber.
* [ ] Grass.
* [ ] Dirt.
* [ ] Sand.
* [ ] Ice.
* [ ] Wet/slippery surface.
* [ ] High-grip synthetic surface.

## Gameplay

Surface changes may affect:

* [ ] braking behaviour;
* [ ] wall climbing;
* [ ] sliding;
* [ ] turning response;
* [ ] bounce;
* [ ] landing stability;
* [ ] momentum preservation;
* [ ] route choice.

Keep surface behaviour readable.

Do not create dozens of materials that differ only numerically.

---

# 12. Airborne Behaviour

Jumps should preserve the physical-control identity of the game.

## Initial Airborne Behaviour

* [ ] Allow natural ballistic flight.
* [ ] Preserve angular momentum appropriately.
* [ ] Detect landing.
* [ ] Make landing orientation matter where useful.
* [ ] Allow poor landings to lose momentum.
* [ ] Allow skilled landings to preserve momentum.

## Internal Mass While Airborne

Investigate whether shifting the internal mass can:

* [ ] alter rotational behaviour;
* [ ] influence pitch;
* [ ] influence roll;
* [ ] influence yaw indirectly;
* [ ] prepare landing orientation;
* [ ] affect stability.

Do not introduce arbitrary mid-air steering.

Any airborne control should remain consistent with the project's physics-as-control philosophy.

---

# 13. Rotational Inertia Experiments

A possible secondary mechanic is manipulation of the ball's rotational inertia.

This is speculative and should remain subordinate to the centre-of-mass mechanic.

Potential model:

* [ ] Pull internal mass inward.
* [ ] Push internal mass outward.
* [ ] Change rotational inertia.
* [ ] Influence spin rate.
* [ ] Influence stability.
* [ ] Influence airborne rotation.
* [ ] Influence recovery.

Potential controls:

* [ ] Hold to contract mass inward.
* [ ] Release to expand.
* [ ] Separate analogue control.
* [ ] Context-sensitive behaviour.

Evaluate whether this adds skill or merely complexity.

Remove it if it obscures the cleaner centre-of-mass mechanic.

---

# 14. Speed and Momentum

Speed should feel earned rather than granted by arbitrary acceleration.

## Momentum

* [ ] Preserve momentum through skilled lines.
* [ ] Lose momentum through poor lines.
* [ ] Gain speed through elevation loss.
* [ ] Trade speed for height.
* [ ] Recover speed through good positioning.
* [ ] Make impacts meaningfully affect momentum.
* [ ] Avoid hidden rubber-band forces during normal movement.

## Speed Limits

* [ ] Determine practical maximum simulation speed.
* [ ] Ensure collision detection remains reliable.
* [ ] Ensure camera remains usable.
* [ ] Ensure tracks remain readable.
* [ ] Prevent numerical instability.

Artificial speed caps should be avoided where physics and track design can solve the problem naturally.

---

# 15. Recovery and Failure

The player should not immediately lose because of every mistake.

Interesting recovery can itself become part of the skill ceiling.

## Recovery

* [ ] Recover from climbing too high on a wall.
* [ ] Recover from sideways motion.
* [ ] Recover from poor landings.
* [ ] Recover from collisions.
* [ ] Recover from low-speed states.
* [ ] Recover from entering a bowl poorly.
* [ ] Allow skilled players to convert mistakes into survivable lines.

## Failure

Possible failure states:

* [ ] Leave the track.
* [ ] Become physically trapped.
* [ ] Fall into unrecoverable geometry.
* [ ] Stop in an invalid location.
* [ ] Miss a required track section.

Potential response:

* [ ] Reset to recent checkpoint.
* [ ] Apply time penalty.
* [ ] Respawn with retained race state.
* [ ] Preserve understandable consequences.

Avoid frustrating long periods where the player knows the run is irrecoverably ruined but must continue.

---

# 16. Multiple Balls

Only introduce multiple racers once the single-ball mechanic is enjoyable.

## Basic Multiple-Ball Simulation

* [ ] Spawn multiple balls.
* [ ] Simulate them simultaneously.
* [ ] Support ball-to-ball collision.
* [ ] Maintain independent physical state.
* [ ] Identify balls visually.
* [ ] Maintain stable simulation with several balls.

## Collision Gameplay

* [ ] Balls can bump one another.
* [ ] Collisions transfer momentum.
* [ ] Players can be knocked higher or lower on banks.
* [ ] Collisions can disrupt racing lines.
* [ ] Players can recover from collisions.
* [ ] Positioning before bottlenecks matters.
* [ ] Heavier/faster impacts feel meaningfully different.

Avoid turning collision into conventional combat unless future playtesting strongly supports it.

---

# 17. Ball Characteristics

Different balls could eventually create different handling characteristics.

Do not introduce these before the baseline ball is satisfying.

Potential characteristics:

* [ ] total mass;
* [ ] internal movable mass;
* [ ] internal-mass ratio;
* [ ] mass movement speed;
* [ ] shell friction;
* [ ] restitution;
* [ ] rotational inertia;
* [ ] aerodynamic behaviour if relevant;
* [ ] visual size where gameplay permits.

Potential archetypes:

* [ ] balanced;
* [ ] heavy;
* [ ] agile;
* [ ] high-grip;
* [ ] low-grip;
* [ ] bouncy;
* [ ] stable;
* [ ] chaotic.

Differences should create distinct physical handling rather than simple statistical upgrades.

---

# 18. Player Identity and Ball Presentation

Balls need to remain readable during crowded races.

* [ ] Distinct player colours/materials.
* [ ] Clear player marker where useful.
* [ ] Readable orientation if orientation matters.
* [ ] Visual indication of internal mass during development.
* [ ] Decide whether internal mass remains visible in final presentation.
* [ ] Distinguish local player from opponents.
* [ ] Preserve readability during collisions.

Potential visual concepts:

* [ ] transparent/semi-transparent shell showing internal mechanism;
* [ ] glowing internal weight;
* [ ] mechanical gyroscope-like interior;
* [ ] stylised shifting core;
* [ ] exterior indicator showing current weight bias.

Presentation should help players understand the mechanic.

---

# 19. Race Structure

Once multiple balls exist, establish actual racing rules.

## Basic Race

* [ ] Starting grid/positions.
* [ ] Countdown.
* [ ] Race start.
* [ ] Track progression.
* [ ] Checkpoints.
* [ ] Finish detection.
* [ ] Finishing order.
* [ ] Race results.
* [ ] Restart/rematch.

## Position Tracking

* [ ] Determine progress through track.
* [ ] Handle alternate routes.
* [ ] Handle players temporarily travelling backwards.
* [ ] Handle falls/resets.
* [ ] Display race position accurately.

## Race Formats

Potential later formats:

* [ ] Point-to-point downhill race.
* [ ] Multi-lap circuit.
* [ ] Elimination.
* [ ] Time trial.
* [ ] Knockout rounds.
* [ ] Survival track.
* [ ] Trick/skill course.
* [ ] Endurance descent.

Point-to-point downhill racing should remain the initial conceptual default.

---

# 20. Starts

Race starts could become a small skill opportunity.

Potential approaches:

* [ ] All balls released simultaneously.
* [ ] Player controls initial internal-mass position.
* [ ] Player influences launch timing.
* [ ] Gravity-only release.
* [ ] Starting gate.
* [ ] Initial push/throw mechanic.

Avoid making starts disproportionately important relative to the rest of the race.

---

# 21. AI Racers

AI should interact with the same physical control model where practical.

It should not cheat by directly steering its ball.

## Basic AI

* [ ] Observe track direction.
* [ ] Determine desired physical line.
* [ ] Manipulate internal mass.
* [ ] Navigate basic track geometry.
* [ ] Recover from mistakes.
* [ ] Complete a race.

## AI Racing

* [ ] Respond to nearby racers.
* [ ] Choose high/low lines.
* [ ] Handle jumps.
* [ ] Handle collisions.
* [ ] Attempt overtakes.
* [ ] Avoid obvious hazards.

## AI Skill

Potential differences:

* [ ] reaction time;
* [ ] control precision;
* [ ] track anticipation;
* [ ] recovery ability;
* [ ] collision awareness;
* [ ] risk tolerance.

AI should make understandable mistakes rather than simply receive slower physics.

---

# 22. Local Multiplayer

Potential forms:

* [ ] Split-screen.
* [ ] Shared-screen where practical.
* [ ] Multiple controllers.
* [ ] Local race setup.
* [ ] Player identification.
* [ ] Independent camera behaviour for split-screen.

Evaluate rendering and performance implications before committing.

---

# 23. Network Multiplayer — Future

Networking is intentionally not an early requirement.

Physics-heavy multiplayer can create substantial synchronisation complexity.

Do not let hypothetical networking requirements distort the initial architecture.

Potential future work:

* [ ] Define networking requirements.
* [ ] Evaluate authoritative-server simulation.
* [ ] Evaluate physics synchronisation strategy.
* [ ] Handle prediction/interpolation if required.
* [ ] Handle collision disagreement.
* [ ] Handle race-state synchronisation.
* [ ] Handle reconnection.
* [ ] Direct/private races.
* [ ] Matchmaking only if project scale warrants it.

Networking should only be investigated once local racing is genuinely worth sharing.

---

# 24. HUD

The HUD should remain minimal during active racing.

Potential information:

* [ ] race position;
* [ ] speed;
* [ ] checkpoint/progress;
* [ ] race time;
* [ ] lap where applicable;
* [ ] nearby racer indicators;
* [ ] internal-mass position where useful;
* [ ] reset/recovery state.

Avoid filling the screen with telemetry.

The player should primarily read the track and physical behaviour of the ball.

---

# 25. Input Feel

Input tuning may be one of the most important areas of the entire project.

Investigate:

* [ ] analogue response curve;
* [ ] dead zone;
* [ ] mass movement speed;
* [ ] acceleration of mass movement;
* [ ] return-to-centre behaviour;
* [ ] damping;
* [ ] maximum displacement;
* [ ] input smoothing;
* [ ] rapid direction changes;
* [ ] control sensitivity versus speed.

Different contexts may require different tuning, but avoid hidden behaviour that makes controls inconsistent.

The player should gradually develop physical intuition.

---

# 26. Feedback and Game Feel

The underlying mechanic may require strong sensory feedback to become readable and satisfying.

## Visual Feedback

* [ ] Ball rotation readability.
* [ ] Speed cues.
* [ ] Internal-mass movement.
* [ ] Impact effects.
* [ ] Landing effects.
* [ ] Surface response.
* [ ] Dust/sparks/debris where appropriate.
* [ ] Speed streaks only if useful and not distracting.

## Camera Feedback

* [ ] Acceleration response.
* [ ] Impact response.
* [ ] Landing response.
* [ ] High-speed response.
* [ ] Airborne response.

## Haptic Feedback

Later:

* [ ] collision vibration;
* [ ] surface vibration;
* [ ] landing impact;
* [ ] internal-mass movement cues;
* [ ] high-speed rumble.

Feedback should communicate physics, not merely decorate it.

---

# 27. Audio

Audio should reinforce weight, speed, surfaces, collisions, and mechanical movement.

Potential sounds:

* [ ] rolling.
* [ ] sliding.
* [ ] scraping.
* [ ] impacts.
* [ ] ball-to-ball collisions.
* [ ] landing.
* [ ] airborne wind.
* [ ] internal mechanism movement.
* [ ] surface transitions.
* [ ] checkpoint.
* [ ] race start.
* [ ] finish.
* [ ] crowd/environmental ambience where appropriate.

Rolling audio should respond meaningfully to speed and surface.

The internal shifting mechanism may benefit from a distinctive mechanical sound that helps players feel what the ball is doing.

---

# 28. Visual Style

Readability and personality matter more than realism.

Potential direction:

* [ ] stylised 3D.
* [ ] clean track geometry.
* [ ] exaggerated scale.
* [ ] strong silhouettes.
* [ ] highly readable ball colours.
* [ ] clear surface differentiation.
* [ ] visible internal mechanisms.
* [ ] strong sense of depth and speed.
* [ ] dramatic environments without obscuring the track.

Possible aesthetic directions:

* [ ] giant abstract sports arena;
* [ ] futuristic mechanical course;
* [ ] toy-like physical playground;
* [ ] impossible architectural structures;
* [ ] enormous real-world-inspired environments;
* [ ] surreal floating tracks.

Do not allow asset production to delay gameplay experimentation.

Primitive geometry is acceptable for as long as it remains useful.

---

# 29. Track Environments

Once the physical track language is established, environments can provide spectacle and identity.

Potential environments:

* [ ] giant sports arena.
* [ ] mountain descent.
* [ ] industrial facility.
* [ ] futuristic city.
* [ ] enormous drainage system.
* [ ] canyon.
* [ ] ice world.
* [ ] desert.
* [ ] space station.
* [ ] low-gravity environment.
* [ ] impossible abstract geometry.
* [ ] toy-room scale environment.
* [ ] gigantic household environment.

Environmental art should not interfere with reading track geometry.

---

# 30. Environmental Physics — Experimental

Later environments could modify physical conditions.

Potential experiments:

* [ ] different gravity strengths.
* [ ] low gravity.
* [ ] high gravity.
* [ ] wind.
* [ ] directional airflow.
* [ ] water currents.
* [ ] moving track surfaces.
* [ ] conveyor belts.
* [ ] magnetic fields.
* [ ] low-friction environments.
* [ ] variable gravity sections.

These should remain understandable gameplay modifiers rather than arbitrary chaos.

---

# 31. Dynamic Track Elements

Only after static track racing is strong.

Potential elements:

* [ ] moving platforms.
* [ ] rotating cylinders.
* [ ] swinging obstacles.
* [ ] gates.
* [ ] pistons.
* [ ] moving walls.
* [ ] collapsing sections.
* [ ] seesaws.
* [ ] rotating bowls.
* [ ] conveyor surfaces.
* [ ] launch mechanisms.
* [ ] crushers.
* [ ] pendulums.

Dynamic obstacles should create physical decisions rather than simple timing-game interruptions.

---

# 32. Route Choice

Tracks may eventually support multiple viable lines.

* [ ] High route versus low route.
* [ ] Safe route versus fast route.
* [ ] Jump shortcut.
* [ ] Wall-ride shortcut.
* [ ] Narrow high-speed route.
* [ ] Wide slower route.
* [ ] Routes favouring different ball characteristics.
* [ ] Routes affected by race traffic.

Avoid obvious fake choices where one route is always superior.

---

# 33. Drafting and Traffic

Investigate whether groups of balls create useful race dynamics beyond collisions.

Potential ideas:

* [ ] aerodynamic drafting only if it produces understandable gameplay;
* [ ] slipstream effect;
* [ ] wake turbulence;
* [ ] collision shielding;
* [ ] strategic positioning before narrow sections.

Do not introduce aerodynamic complexity merely because conventional racing games have drafting.

---

# 34. Competitive Collision Behaviour

Collision should create excitement without making races feel arbitrary.

Investigate:

* [ ] side impacts.
* [ ] rear impacts.
* [ ] high-speed impacts.
* [ ] wall pinching.
* [ ] bank displacement.
* [ ] airborne collision.
* [ ] landing onto another ball.
* [ ] using another ball to redirect momentum.
* [ ] deliberate blocking.

Avoid:

* [ ] easy griefing.
* [ ] permanent stun states.
* [ ] excessive loss of player agency.
* [ ] collisions that routinely decide races regardless of skill.

Chaotic moments are desirable.

Arbitrary outcomes are not.

---

# 35. Checkpoints and Reset

A robust recovery system will be important for large tracks.

* [ ] Define checkpoint volumes.
* [ ] Record valid progression.
* [ ] Detect leaving playable track.
* [ ] Detect unrecoverable states.
* [ ] Reset to appropriate checkpoint.
* [ ] Apply fair time penalty if required.
* [ ] Prevent checkpoint exploits.
* [ ] Preserve race ordering correctly after reset.

Reset should return the player to racing quickly.

---

# 36. Time Trial

Time trial may be useful even before AI or multiplayer.

* [ ] Start timer.
* [ ] Finish timer.
* [ ] Record best time.
* [ ] Show previous best.
* [ ] Show checkpoint splits.
* [ ] Restart quickly.
* [ ] Support repeated attempts.

Potential later features:

* [ ] ghost ball.
* [ ] personal-best ghost.
* [ ] downloadable ghosts.
* [ ] leaderboard.

Time trial could become an excellent environment for tuning the control system and track design.

---

# 37. Ghosts and Replays

Potential future functionality:

* [ ] Record player inputs.
* [ ] Record/reconstruct physical state where necessary.
* [ ] Replay completed runs.
* [ ] Personal-best ghost.
* [ ] Slow-motion collision replay.
* [ ] Finish-line replay.
* [ ] Spectator camera.
* [ ] Share interesting runs.

Physics determinism requirements should be investigated before assuming input-only replay is sufficient.

---

# 38. Race Setup

Eventually:

* [ ] Select track.
* [ ] Select player count.
* [ ] Select human/AI participants.
* [ ] Select ball type where applicable.
* [ ] Select laps where applicable.
* [ ] Select race mode.
* [ ] Select physics/environment modifiers where supported.
* [ ] Start race.

Do not build substantial setup UI before there is a race worth configuring.

---

# 39. Menus and Application Flow

Later:

* [ ] Title screen.
* [ ] Main menu.
* [ ] Race setup.
* [ ] Options.
* [ ] Pause menu.
* [ ] Results.
* [ ] Restart.
* [ ] Return to menu.
* [ ] Exit cleanly.

Early development should launch directly into the current gameplay experiment.

---

# 40. Configuration and Settings

When meaningful:

* [ ] resolution.
* [ ] fullscreen/windowed mode.
* [ ] graphics quality.
* [ ] audio levels.
* [ ] input sensitivity.
* [ ] controller configuration.
* [ ] accessibility options.
* [ ] camera options.

Avoid building configuration infrastructure before meaningful settings exist.

---

# 41. Accessibility

Potential considerations:

* [ ] colour-blind-friendly player identification.
* [ ] alternatives to colour-only information.
* [ ] adjustable camera shake.
* [ ] adjustable motion effects.
* [ ] input sensitivity.
* [ ] remappable controls.
* [ ] readable UI scaling.
* [ ] reduced visual effects.
* [ ] clear audio/visual race cues.

Accessibility decisions should be considered as relevant systems appear rather than postponed entirely until the end.

---

# 42. Performance

Physics-heavy multi-ball racing may eventually require optimisation.

Performance work should remain evidence-driven.

* [ ] Establish target frame rate.
* [ ] Measure physics cost.
* [ ] Measure multi-ball simulation cost.
* [ ] Measure track collision cost.
* [ ] Measure rendering cost.
* [ ] Measure particles/effects cost.
* [ ] Profile before optimising.
* [ ] Avoid premature ECS optimisation.
* [ ] Avoid premature track chunking/LOD systems.
* [ ] Preserve physical behaviour during optimisation.

---

# 43. Debugging Tools

Introduce debugging facilities when they directly aid gameplay development.

Potential tools:

* [ ] collider visualisation.
* [ ] centre-of-mass visualisation.
* [ ] internal-mass position visualisation.
* [ ] velocity vector.
* [ ] angular velocity.
* [ ] contact points.
* [ ] friction state.
* [ ] current surface.
* [ ] speed.
* [ ] physics timestep.
* [ ] camera target.
* [ ] checkpoint state.

Avoid creating a general-purpose developer-console framework before it is needed.

---

# 44. Physics Tuning

Centralise gameplay-relevant physical tuning sufficiently that experimentation remains easy.

Potential tuneables:

* [ ] gravity.
* [ ] ball mass.
* [ ] ball radius.
* [ ] internal-mass ratio.
* [ ] internal-mass movement speed.
* [ ] maximum mass offset.
* [ ] shell friction.
* [ ] restitution.
* [ ] damping.
* [ ] rotational behaviour.
* [ ] surface friction.
* [ ] camera smoothing.
* [ ] control response.

Do not prematurely build a generic configuration system.

Constants or focused resources/components are acceptable while the model is evolving rapidly.

---

# 45. Telemetry for Development

Lightweight development telemetry may eventually help tune the mechanic.

Potential measurements:

* [ ] speed through track sections.
* [ ] peak speed.
* [ ] time spent airborne.
* [ ] collisions.
* [ ] reset count.
* [ ] average internal-mass displacement.
* [ ] route choices.
* [ ] checkpoint splits.
* [ ] momentum loss at specific corners.

Only introduce telemetry when there is an actual tuning question to answer.

Do not build analytics infrastructure for hypothetical future use.

---

# 46. Progression — Future

Only consider progression once repeated racing is intrinsically enjoyable.

Potential ideas:

* [ ] track unlocks.
* [ ] cosmetic ball shells.
* [ ] visual internal mechanisms.
* [ ] championship series.
* [ ] medals.
* [ ] time targets.
* [ ] challenge tracks.
* [ ] skill challenges.

Avoid stat upgrades that undermine physics-based competitive fairness unless a specific game mode justifies them.

---

# 47. Cosmetics — Future

Potential cosmetic customisation:

* [ ] shell colour.
* [ ] shell material appearance.
* [ ] trails.
* [ ] internal mechanism appearance.
* [ ] decals.
* [ ] collision effects.
* [ ] finish effects.

Cosmetics should preserve ball readability.

---

# 48. Procedural Tracks — Future / Experimental

Procedural generation could eventually create replayability but should not be attempted until good track design is understood manually.

Potential work:

* [ ] Identify reusable track grammar.
* [ ] Identify valid transition rules.
* [ ] Generate connected track sections.
* [ ] Validate physical traversability.
* [ ] Ensure sufficient momentum.
* [ ] Avoid impossible jumps.
* [ ] Avoid unrecoverable traps.
* [ ] Generate alternate routes.
* [ ] Seed generation.
* [ ] Reproduce generated tracks.

Do not automate track design before understanding what makes a hand-built track fun.

---

# 49. Track Editor — Future

Only consider once track creation itself becomes a genuine bottleneck or desirable player feature.

Potential capabilities:

* [ ] place segments;
* [ ] manipulate splines;
* [ ] adjust banking;
* [ ] adjust width;
* [ ] place jumps;
* [ ] place obstacles;
* [ ] assign surfaces;
* [ ] define checkpoints;
* [ ] test immediately;
* [ ] save/load tracks;
* [ ] validate connectivity.

A track editor is a separate product-sized feature.

Treat it accordingly.

---

# 50. Experimental Gameplay Backlog

These ideas are deliberately speculative.

They should remain here until playtesting demonstrates a reason to pursue them.

* [ ] Pull internal mass inward to increase spin rate.
* [ ] Push internal mass outward for stability.
* [ ] Multiple independently movable internal weights.
* [ ] Gyroscopic internal mechanism.
* [ ] Temporarily lock internal mass.
* [ ] Free-swinging internal pendulum.
* [ ] Fluid-filled ball.
* [ ] Deformable ball.
* [ ] Extremely bouncy ball.
* [ ] Extremely heavy ball.
* [ ] Tiny high-speed ball.
* [ ] Oversized slow ball.
* [ ] Variable-radius ball.
* [ ] Ball that changes mass during a race.
* [ ] Magnetic ball.
* [ ] Magnetic track sections.
* [ ] Low-gravity tracks.
* [ ] High-gravity tracks.
* [ ] Zero-gravity transition.
* [ ] Wind tunnels.
* [ ] Water sections.
* [ ] Vertical drops.
* [ ] Enormous ski-jump-style launches.
* [ ] Full loops.
* [ ] Corkscrews.
* [ ] Spirals.
* [ ] Huge bowls with multiple exits.
* [ ] Funnel sections where racers converge.
* [ ] Narrow collision-heavy chutes.
* [ ] Giant open half-pipes.
* [ ] Wall riding.
* [ ] Ceiling riding if physically achievable.
* [ ] Breakable track elements.
* [ ] Destructible obstacles.
* [ ] Moving track geometry.
* [ ] Track sections rotating during the race.
* [ ] Cannons/launchers.
* [ ] Pinball-like bumpers.
* [ ] Giant fans.
* [ ] Conveyors.
* [ ] Seesaws.
* [ ] Pendulums.
* [ ] Crushers.
* [ ] Trapdoors.
* [ ] Temporary surface changes.
* [ ] Race through gigantic household objects.
* [ ] Race through industrial machinery.
* [ ] Race down a mountain.
* [ ] Race through an impossible megastructure.
* [ ] Race inside a space station.
* [ ] Completely ridiculous track ideas discovered during development.

---

# 51. Development Milestones

These are conceptual milestones rather than fixed specification sequences.

Specifications remain timestamped and should be created according to coherent work rather than milestone numbering.

## Milestone A — Foundation

* [ ] Rust project/workspace exists.
* [ ] Bevy is integrated.
* [ ] Game window opens.
* [ ] Basic 3D scene renders.
* [ ] Sphere renders.
* [ ] Repository builds cleanly.
* [ ] Tests and quality checks pass.
* [ ] Documentation and roadmap exist.
* [ ] SpecKit workflow is established.

**At this point Shifty Balls is ready for gameplay experimentation.**

## Milestone B — First Roll

* [ ] Physics approach selected.
* [ ] Ball is a rigid body.
* [ ] Gravity affects the ball.
* [ ] Ball collides with track.
* [ ] Ball rolls downhill.
* [ ] Ball rotation is physically represented.
* [ ] Camera follows the ball sufficiently for testing.

**At this point we have a ball, not yet a game.**

## Milestone C — First Shift

* [ ] Internal mass exists conceptually and technically.
* [ ] Player can move internal mass.
* [ ] Centre-of-mass change affects physical behaviour.
* [ ] Player can intentionally influence the ball's path.
* [ ] Control response is understandable.
* [ ] Debug visualisation explains what the internal mass is doing.

**At this point we discover whether the central idea works.**

## Milestone D — First Course

* [ ] Small prototype course exists.
* [ ] Course includes meaningful banks/curves.
* [ ] Momentum matters.
* [ ] Player can improve through practice.
* [ ] Mistakes have understandable consequences.
* [ ] Recovery is possible.
* [ ] Completing the course feels satisfying.
* [ ] Repeating the course to improve feels worthwhile.

**At this point decide whether Shifty Balls deserves to become a full game.**

## Milestone E — First Race

* [ ] Multiple balls exist.
* [ ] Ball-to-ball collisions work.
* [ ] Starting sequence exists.
* [ ] Track progression works.
* [ ] Finish detection works.
* [ ] Race positions work.
* [ ] AI or another local player can race.
* [ ] A complete race can be won.

**At this point Shifty Balls is a racing game.**

## Milestone F — Physical Racing

* [ ] Collision behaviour is tuned.
* [ ] Overtaking is possible.
* [ ] Track geometry creates racing decisions.
* [ ] Traffic affects line choice.
* [ ] Recovery after collisions is satisfying.
* [ ] Different lines through major track elements are viable.
* [ ] Race outcomes reflect meaningful player skill.

## Milestone G — Track Language

* [ ] Several track-element types exist.
* [ ] Half-pipes are useful.
* [ ] Banks are useful.
* [ ] Jumps are useful.
* [ ] Bowls/funnels are useful.
* [ ] Surface differences exist where beneficial.
* [ ] Track construction workflow is practical.
* [ ] At least one complete polished track exists.

## Milestone H — Complete Race Experience

* [ ] Race setup exists.
* [ ] AI racers provide useful competition.
* [ ] Camera supports racing cleanly.
* [ ] HUD communicates necessary race state.
* [ ] Audio reinforces speed and physical behaviour.
* [ ] Results are presented.
* [ ] Race can be restarted.
* [ ] Full game loop works without developer tooling.

## Milestone I — Variety

* [ ] Multiple tracks.
* [ ] Multiple environments.
* [ ] Additional track elements.
* [ ] Ball characteristics explored if worthwhile.
* [ ] Additional race modes if worthwhile.
* [ ] Improved AI.
* [ ] Improved visual identity.
* [ ] Improved audio identity.

## Milestone J — Polish

* [ ] Controls thoroughly tuned.
* [ ] Camera thoroughly tuned.
* [ ] Physics edge cases addressed.
* [ ] Collision behaviour polished.
* [ ] UI polished.
* [ ] Accessibility options added where appropriate.
* [ ] Performance measured and tuned.
* [ ] Visual/audio presentation polished.

## Milestone K — Optional Expansion

* [ ] Local multiplayer evaluation.
* [ ] Network multiplayer evaluation.
* [ ] Ghost/replay sharing.
* [ ] Procedural track evaluation.
* [ ] Track editor evaluation.
* [ ] Progression.
* [ ] Cosmetics.
* [ ] Community-oriented features if the project reaches that stage.

---

# 52. Questions the Project Must Answer

These questions are more important than completing the roadmap.

## Core Mechanic

* [ ] Is shifting internal mass fun?
* [ ] Is the resulting movement understandable?
* [ ] Can players intentionally improve?
* [ ] Does the ball feel physical rather than directly steered?
* [ ] Is there enough control to avoid frustration?
* [ ] Is there enough inertia to preserve the game's identity?
* [ ] Does analogue control materially improve the experience?

## Racing

* [ ] Is racing other balls more fun than time trialling alone?
* [ ] Are collisions exciting rather than irritating?
* [ ] Can skilled players recover from contact?
* [ ] Can overtaking emerge naturally from track geometry and momentum?
* [ ] Does racing remain readable with many balls together?
* [ ] What is the right number of simultaneous racers?

## Track Design

* [ ] What geometry best exploits centre-of-mass control?
* [ ] Are half-pipes actually fun?
* [ ] Are bowls fun?
* [ ] Are wall rides controllable?
* [ ] Are jumps interesting with the available airborne control?
* [ ] How wide should tracks be?
* [ ] How much freedom should players have laterally?
* [ ] How much recovery space is needed?
* [ ] Should tracks generally contain walls?
* [ ] When should falling off be possible?

## Physics

* [ ] How realistic should rolling behaviour be?
* [ ] How exaggerated should mass shifting be?
* [ ] How large should the internal mass be relative to the shell?
* [ ] How quickly should the mass move?
* [ ] Should internal mass have inertia of its own?
* [ ] Should rotational inertia manipulation become a second mechanic?
* [ ] How much bounce is fun?
* [ ] How much friction is fun?
* [ ] What speeds remain controllable?

## Game Identity

* [ ] Is Shifty Balls primarily a serious skill racer with a ridiculous premise?
* [ ] Is it a chaotic party racer?
* [ ] Can it successfully be both?
* [ ] Should collisions be strategic or mostly incidental?
* [ ] Should ball differences exist?
* [ ] Should tracks become fantastical?
* [ ] How much spectacle can be added without weakening physical readability?

These questions should be answered through prototypes and play rather than architecture discussions wherever possible.

---

# Guiding Test

When deciding whether an item from this roadmap should become active work, ask:

**Will this make controlling the ball more satisfying, make racing more interesting, make the project more fun to build, or materially improve our ability to understand and maintain it?**

If not, it can stay on the roadmap.

For early development, an even simpler test applies:

**Does this help us discover whether shifting the mass inside a rolling ball is fun?**

If not, it probably does not belong in the next specification.

The project does not need every feature recorded here.

It needs a ball.

It needs a hill.

It needs gravity.

And then it needs us to move that bloody weight and see what happens.
