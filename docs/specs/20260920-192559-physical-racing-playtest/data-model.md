# Data Model: Physical Racing Playtest

This feature reuses the existing four physical racers and one connected,
ordered route. It adds no new controller hierarchy, persistent storage, or
general track graph.

## Racing section

| Property | Meaning | Validation |
|---|---|---|
| approach | Connected course surface before the final bank | Delivers the field into the section without a new start or teleport |
| usable width | Surface area between visible retaining boundaries | Fits multiple ball-widths and leaves distinct valid physical lines |
| bank/grade | Visible slope that makes high/low positioning affect momentum and risk | Remains readable and recoverable under normal SHIFT control |
| route gate | Existing broad ordered progression region covering the section exit | Accepts valid lateral lines while rejecting obvious shortcuts |
| recovery space | Contiguous playable surface and boundaries following a traffic displacement | Lets a displaced racer use normal control to rejoin the next route leg |

The racing section is course data authored directly with the current surface
helpers. It is not an alternate path: every valid line advances through the
same ordered gate sequence.

## Racer and physical baseline

| Property | Meaning | Validation |
|---|---|---|
| participant | One human and three route-targeting AI racers | Four distinct fair starts; all release together |
| control request | Existing SHIFT internal-mass target in stable world coordinates | No force, direct velocity, transform, path, or collision-specific assistance |
| contact baseline | Existing ball/track friction and restitution | Ordinary contact remains active; changes are central, explicit, and manually observed |
| progress | Existing next required gate and optional immutable finish record | Contact can change position but cannot bypass the route |
| order | Existing finished place, gate count, in-leg advance, stable-ID comparison | Relative order can change after the start without flicker from invalid shortcuts |

## Passing event

| Field | Meaning | Validation |
|---|---|---|
| before/after order | The two racers' relative display order around the racing section | Change occurs after GO and is not assigned by a scripted pass |
| cause | Observed line choice, momentum difference, or ordinary contact | No catch-up speed, teleport, forced lane, or collision attack |
| route validity | Both participants continue through their required gates | Neither participant receives a progression exemption |

Passing events are owner-observed playtest evidence, not a simulation command or
a persistent gameplay record.

## Recovery observation

| Field | Meaning | Validation |
|---|---|---|
| trigger | Ordinary contact that visibly displaces a racer from its intended line | Occurs during an active race |
| recovery state | Racer remains in playable course space and can identify a forward/rejoin direction | No invisible correction or developer-only reset |
| outcome | Racer rejoins the ordered route, finishes, or yields an explicit negative/unresolved finding | Result is recorded honestly in the race document |

## Race lifecycle

The existing `Countdown → Racing → Player finished → rematch` lifecycle is
unchanged. All mutable racer motion, internal mass, centre of mass, progress,
and finish data return to the fair start state on rematch. The feature only
changes the playable course conditions and the evidence collected while racing.
