# Data Model: First Course Kill Test

## PrototypeCourse

| Property | Meaning | Validation |
|---|---|---|
| start position and orientation | Defined fair state for every attempt | Supports ball at rest without immediate obstruction |
| connected surfaces | One sequence of opening, bends, height choice, momentum crest, and finish approach | Adjacent geometry remains visually and physically continuous enough to roll |
| readable boundaries | Retaining edges where recovery is intended and visible exit risk where it is not | Boundaries match collision geometry |
| finish region | Obvious final area | Contains the ball centre only at the intended end of a run |

The course has no serialised format, reusable catalogue, or external authoring
model. Its layout is deliberately local to this experiment.

## AttemptState

| Field | Meaning | Validation |
|---|---|---|
| phase | Running or completed | Completion is one-way until restart |
| elapsed | Duration since the active attempt began | Advances only while running |
| completed time | Captured duration for a completed attempt | Set once on first finish entry |
| session best | Fastest completed duration in this application session | Survives restart; only decreases after completion |

State transitions:

```text
initial/restart -> Running -> Completed
                      ^           |
                      +--- R -----+
```

Reaching the finish while already completed has no effect. Restart clears the
active/completed state and restores a running attempt without clearing session
best.

## FinishRegion

| Property | Meaning |
|---|---|
| centre | Course-world centre of the final apron region |
| half extents | Broad enough to tolerate the ball radius and normal finish line variation |
| visual marker | High-contrast geometry readable before arrival |

The finish is an inclusive axis-aligned bounds check against the player-ball
centre. It is a one-course completion signal, not a checkpoint or race rule.

## DevelopmentReadout

| Displayed value | Source | Purpose |
|---|---|---|
| current or completed time | AttemptState | Shows the immediate result of an attempt |
| session best | AttemptState | Makes practice measurable |
| speed | Current ball motion magnitude | Helps explain momentum outcomes |
| mass display | Existing InternalMassState visualisation | Keeps cause and response visible |
| restart/debug hint | Fixed controls | Supports fast experimentation |

The readout is development-facing and session-local. It does not own course,
attempt, or physical behaviour.
