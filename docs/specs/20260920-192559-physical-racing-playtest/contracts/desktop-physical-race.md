# Desktop Physical Racing Contract

This contract defines observable behaviour for the Physical Racing Playtest. It
extends the accepted First Race desktop path; it is not a network, multiplayer,
or public programming interface.

## Entering a race

1. From the application, the player selects `4` to enter a fresh four-racer
   SHIFT race.
2. The field appears at the existing fair start and displays the normal
   3–2–1–GO countdown, player place, and elapsed time.
3. At GO, all racers are released together. Human and AI mass control are
   active only through the established SHIFT physical model.

## Traffic and passing

- Ordinary ball-to-ball and ball-to-course contact remains active throughout a
  race. It may alter line, speed, or order, but it never triggers a combat move,
  scripted pass, stun state, catch-up speed, direct pose change, or teleport.
- The designated wide racing section visibly offers at least two physical lines
  within the one valid route. A player may gain or lose position through line,
  momentum, or traffic.
- A player displaced within course bounds can see a normal-control route toward
  the next gate or finish. Falling behind is valid; being silently returned to a
  preferred line is not.

## Readability and rematch

- The camera continues to follow the human racer closely enough for the player
  to identify a forward or recovery direction after ordinary traffic contact.
- The race display continues to show countdown/GO, place, and time, followed by
  the fixed finish result when the player finishes validly.
- `R` at countdown, while racing, or after a result restores all four racers to
  the common start, clears race state, and begins a fresh countdown.

## Explicit exclusions

This feature does not add alternate-route progression, laps, a track editor,
new race modes, local/network multiplayer, controller rebinding, collision
weapons, automatic recovery, rubber-banding, AI overtaking strategy, AI
avoidance, new control models, persistent results, replay/telemetry, or a
general collision framework.
