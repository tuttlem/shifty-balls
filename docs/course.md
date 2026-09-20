# First Course Kill Test

This is one compact, hand-authored physical course, not the beginning of a
track system. It first established whether Shifty Balls was worth another run;
it now also hosts the deliberately small first-race traffic experiment and the
post-race human-only control comparison.

## Layout intent

The route is assembled from matching visible and collision primitive surfaces.
Each surface overlaps its neighbour slightly and changes direction or bank
gradually, so the ball sees one readable physical route rather than a set of
unrelated objects.

1. A level start deck and forgiving downhill establish momentum and lateral
   control.
2. A broad gentle bend acts as an implicit tutorial: shift weight, change line.
3. A 16-metre-wide, softened bank and its continuous run-out form the physical
   racing section. It offers a high/low line around centre-seeking opponents;
   retaining boundaries keep a poor traffic displacement recoverable without
   hiding it with a reset or collision aid.
4. A compression and uphill crest expose whether the player preserved enough
   momentum through the preceding line.
5. A final descent and bright gate form one obvious finish apron.

There is deliberately no jump or drop in this version. Grounded mass control
is the kill-test priority; the crest gives a momentum question without
confusing the result with airborne behaviour or adding mid-air steering.

## Controls and timing

For the current control comparison, `1`/`2`/`3` restart this same course with
SHIFT/TORQUE/FORCE respectively, while `4` restores First Race. The comparison
removes opponents physically and keeps the ball/course/camera baseline common;
see [the comparison record](control-model-comparison.md). The original course
and its physical-racing section remain a possible playtest confounder, not
proof that one model is superior.

| Input | Behaviour |
|---|---|
| W/A/S/D | Shift the existing internal mass in world-relative directions |
| R | Reset every racer, motion, mass position, camera, and race state to a fresh countdown |
| F3 | Toggle the development readout and mass/velocity display |

The first-race display now shows countdown, position, and current time. It
remains deliberately small: there are no laps, persistent records, or
leaderboard.

## Tuning and camera

The existing mass ratio, displacement, movement speed, gravity, friction, and
restitution remain the initial baseline. The physical-racing pass widens and
softens the known traffic bottleneck but adds no direct steering, propulsion,
traction assistance, velocity cap, collision-specific assistance, or artificial
acceleration.

The camera remains world-up and independent from ball rotation. It now starts
behind and above the ball with an early down-track look target, and snaps to
the same view on R before normal smoothing resumes. It intentionally avoids
camera roll, impact shake, dramatic field of view, and a velocity-facing view.

## Kill-test record

Run the manual procedure in the feature [quickstart](specs/20260919-165535-first-course/quickstart.md)
before completing this record. An unresolved or negative result is valid
evidence; do not use additional content to conceal it.

| Question | Observation | Result |
|---|---|---|
| Can the player predict how shifting mass affects the ball? | Input focus worked; after waking the sleeping body on mass shifts, the response was immediately legible to the project owner. | Promising; needs broader testing |
| Does improved technique produce visibly improved results? | The opening course is currently too difficult to make a fair comparison. | Unresolved |
| Can the player intentionally climb a bank? | Not established in this first owner session. | Unresolved |
| Can the player intentionally descend a bank? | Not established in this first owner session. | Unresolved |
| Can height be traded for speed? | Not established in this first owner session. | Unresolved |
| Can momentum be preserved through turns? | Not established in this first owner session. | Unresolved |
| Can mistakes be recovered from? | Not established in this first owner session. | Unresolved |
| Does control remain interesting across repeated attempts? | The owner described the mechanic as “almost there” and the game idea as strong, despite the steep learning curve. | Promising; needs sustained testing |
| Is getting faster satisfying? | The initial course difficulty prevented a useful timing evaluation. | Unresolved |
| Does the player feel responsible for success and failure? | The owner’s response indicates the physical control has compelling potential, but this was not tested rigorously. | Promising; needs broader testing |

The project owner accepted this initial kill-test pass on 19 September 2026.
The evidence supports continuing Shifty Balls, but it does **not** validate this
course as appropriately learnable. The immediate follow-up should soften and
broaden the opening course before any racing systems are considered.

That follow-up is now signed off as the Physical Racing Playtest: the widened,
softened bank supported successful repeated races with ordinary traffic. It is
one validated racing section, not a general track-language decision.

## Known limits and next questions

- The course intentionally has no automatic failure detector; R is the fast
  recovery path when the ball falls, stalls, or becomes trapped.
- Surface seams are deliberately gradual and slightly overlapping, but course
  playtesting must still look for snags at joins.
- The first pass does not claim an ideal duration, speed range, camera feel, or
  final tuning. Record those findings before considering Milestone E.
