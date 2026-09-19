# Desktop Scene Contract

This is a human-facing application contract, not a network or library API.

## Launch

- From the repository root, cargo run starts the Shifty Balls desktop application after the
  documented prerequisites are installed.
- The primary window title is exactly Shifty Balls.

## Initial View

- The initial view is a readable, lit, fixed-perspective 3D scene.
- It contains one flat ground/reference surface and one sphere representing the future player ball.
- The sphere and surface use generated presentation geometry; external art assets are not required.

## Lifecycle

- The window remains responsive under normal desktop operation.
- A normal desktop close request ends the application cleanly.

## Explicit Absences

- There is no gameplay input or camera control.
- The sphere has no physics, gravity, velocity, rolling, collision, friction, restitution,
  centre-of-mass, or internal-mass behaviour.
- The application exposes no network, file-format, command-line, or persistent-data interface
  beyond Cargo's normal development commands.
