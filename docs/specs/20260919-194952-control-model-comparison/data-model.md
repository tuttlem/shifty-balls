# Data Model: Control Model Comparison

## ControlModel

The mutually exclusive human control selection: `Shift`, `Torque`, or `Force`.
SHIFT alone may update internal mass; TORQUE alone may submit rotational input;
FORCE alone may submit centre-of-mass force. Changing selection normalises and
restarts the attempt before the new effect acts.

## ControlIntent

| Field | Meaning | Validation |
|---|---|---|
| `world_direction` | Player request in stable world coordinates. | Horizontal; magnitude is zero or one; diagonal input is normalised. |

One human comparison ball consumes this intent through exactly one selected
`ControlModel`.

## ControlModelTuning

| Value | Meaning | Validation |
|---|---|---|
| SHIFT values | Existing mass ratio, displacement, and movement speed. | Existing physical bounds apply. |
| Torque strength | Maximum world torque for unit intent. | Finite, non-negative, TORQUE-only. |
| Force strength | Maximum world force for unit intent. | Finite, non-negative, FORCE-only. |

Ball baseline, gravity, course, and camera remain common across models.

## SessionMode

| Value | Participants | Lifecycle |
|---|---|---|
| `Comparison` | Human ball only. | Select, running, finished, reset; per-model session best time. |
| `Race` | Existing human and three AI racers. | Existing countdown, progression, positions, result, rematch. |

Entering comparison removes race-only physical interference. Entering race
restores the established four-racer start. A reset clears movement, centre of
mass, mass state, intent, progression, timer, and camera state as applicable.

## ComparisonAttempt

| Field | Meaning | Validation |
|---|---|---|
| Selected model | Sole model for the attempt. | Immutable while running. |
| Phase | Ready/running/finished lifecycle. | Timer runs only while running. |
| Elapsed | Current or completed duration. | Starts at release and stops at valid finish. |
| Best by model | Lowest valid session completion per model. | Changes only for a lower completed time. |
| Progress | Reused ordered gate state. | Every gate is required before finish. |

## DevelopmentReadout

Shows active model and shared world intent in every mode. SHIFT adds current and
target mass plus its movement disk; TORQUE adds applied axis/magnitude; FORCE
adds applied force/magnitude. Existing velocity context remains useful. Display
is observational, world-stable, and cannot alter simulation.
