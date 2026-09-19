# Data Model: First Internal Mass Shift

## MassShiftTuning

| Field | Initial value | Validation |
|---|---:|---|
| shell mass | 0.55 kilograms | Positive |
| inner mass | 0.45 kilograms | Positive |
| total mass | 1.00 kilogram | Equals shell plus inner |
| inner displacement radius | 0.30 metres | Positive and less than ball radius |
| maximum combined COM offset | 0.135 metres | Radius multiplied by inner / total |
| target and return speed | 1.5 metres per second | Positive |
| reference frame | World horizontal X/Z | Stable while shell rolls |
| neutral position | World zero offset | Release target |

## InternalMassState

| Field | Meaning |
|---|---|
| requested_world | Bounded player intent in world X/Z |
| current_world | Smoothed bounded inner-mass position |
| derived_com_world | current_world multiplied by inner / total |
| derived_com_local | world COM transformed by inverse shell rotation |
| return mode | Target becomes zero on release |

States: Neutral; Directed; Moving; Returning; and Coupled before each physics
preparation step. No second rigid body or collider exists.

## PlayerBall relationship

The existing PlayerBall remains the only Dynamic rigid body with total Mass one
kilogram. It gains CenterOfMass from derived_com_local; collider, gravity,
friction, restitution, and ordinary physics remain unchanged.

## DevelopmentMassDisplay

| Element | Relationship |
|---|---|
| Centre marker | PlayerBall world position |
| Current marker | Ball centre plus current_world |
| Target marker | Ball centre plus requested_world |
| Boundary | Horizontal disk at PlayerBall, radius 0.30 metres |
| Lines | Centre to current and target |

The display is not parented to the rolling shell and never owns physical state.

## InputIntent

W, A, S, and D request positive-Z, negative-X, negative-Z, and positive-X.
Diagonals are normalised before disk clamping. Input has no direct relation to
ball translation or rotation.
