# Desktop Control Comparison Contract

This is a development-facing experiment, not a production menu or rebinding scheme.

## Session selection

| Input | Required result |
|---|---|
| `1` | Start a normalised single-ball SHIFT attempt. |
| `2` | Start a normalised single-ball TORQUE attempt. |
| `3` | Start a normalised single-ball FORCE attempt. |
| `4` | Enter/reset First Race with established SHIFT human and AI field. |
| `R` | Reset the active comparison or race session. |
| `F3` | Toggle development-facing control/movement visualisation. |

Selection must not mix models in a moving attempt. It normalises movement, centre-of-mass/internal-mass, progression, timer, and camera before the new run.

## Shared directional input

| Input | Stable world request |
|---|---|
| `W` | Down-track (`+Z`) |
| `S` | Up-track (`-Z`) |
| `A` | Track-left (`-X`) |
| `D` | Track-right (`+X`) |

Combined input is normalised and never follows the rotating ball's local axes.

## Model behaviour

| Model | Directional request changes | Neutral | Prohibited |
|---|---|---|---|
| SHIFT | Existing internal mass/centre of mass. | Mass returns to centre under smoothing. | Direct movement, force, torque, path, or traction steering. |
| TORQUE | World-relative roll torque. | No player torque. | Direct orientation/velocity/path changes or translational steering force. |
| FORCE | Horizontal force at ball centre of mass. | No player force. | Direct velocity/orientation/path changes, off-centre force, or traction help. |

All models retain ordinary gravity, collision, friction, restitution, rotation, and momentum. TORQUE/FORCE stay active in air for this experiment; their differences must be documented rather than hidden.

The comparison readout shows active model before input, current time, model-specific best time, and controls. F3 shows shared intent and model-specific state. The normal First Race readout remains unchanged.
