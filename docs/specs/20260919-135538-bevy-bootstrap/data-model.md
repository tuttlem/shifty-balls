# Data Model: Project Foundation and Bevy Bootstrap

This feature has no persistent data model and introduces no gameplay-domain state. These entities
describe the observable presentation state used to validate the bootstrap; they do not require
separate Rust types, modules, or abstractions.

## Application Session

| Property | Description | Validation |
|---|---|---|
| Title | The desktop window identifies itself as “Shifty Balls”. | The displayed native window title matches exactly. |
| Lifecycle | The session starts, remains responsive, and ends by normal desktop close. | Closing the window ends the process cleanly. |

**State transitions**: starting → running → closed. There are no saved states or recovery flows.

## Reference Scene

| Property | Description | Validation |
|---|---|---|
| Perspective view | A fixed view frames the reference surface and future player ball. | Both objects are visible at launch. |
| Lighting | A basic light makes both objects readable. | The scene is not unlit or indistinguishable. |
| Ground surface | One flat primitive provides visual reference only. | It is visibly flat and contains no collision behaviour. |

## Future Player Ball

| Property | Description | Validation |
|---|---|---|
| Appearance | One generated sphere identifies the future player ball. | Exactly one visible sphere appears in the initial scene. |
| Transform | A fixed initial placement keeps it clearly visible above or on the reference surface. | Its initial relation to the ground is readable. |
| Behaviour | Presentation-only and stationary. | It has no gravity, velocity, angular velocity, collision, rolling, control, or internal-mass state. |

## Relationships and Invariants

- One application session presents one reference scene.
- One reference scene contains one ground surface and one future player ball.
- The camera and light exist solely to make the reference scene readable.
- No entity in this feature represents a rigid body, collider, input mapping, track, race, or
  mutable gameplay state.
