# Data Model: Half-Pipe Racing Section

## Half-pipe section

| Property | Meaning | Validation |
|---|---|---|
| low channel | Central flat physical lane | Connected forward path and recoverable low-speed state |
| left/right banks | Two symmetric strips on each side | Higher physical lines with visible height/momentum difference |
| outer lips | Only exposed section edges | Visible retaining walls contain climbs without blocking line changes |
| exit gate | Broad next ordered region | Accepts valid lateral lines and rejects route bypasses |

## Race integration

The existing four racers, SHIFT mass requests, contact/material baseline, ordered gate progress, position comparison, finish, and rematch are unchanged. The added broad exit gate is one more required gate in the same connected route; the centre-targeting AI treats it like every current gate.

## Owner observation

| Field | Meaning |
|---|---|
| line result | Visible low/high difference in height, momentum, exit, or outcome |
| traffic result | Ordinary contact, order change, or side-by-side situation |
| recovery result | Normal-control rejoin after poor entry, wall climb, or low speed |
| readability result | Player can see an exit/recovery route and wants to replay |
