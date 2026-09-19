//! Intentionally small, course-aware opponents for the first race.
//!
//! AI has privileged knowledge of the current hand-authored route, but it
//! only requests the same stable world-horizontal internal-mass displacement
//! as the player. Physics still decides whether that request makes a useful
//! line, a collision, or a recoverable mistake.

use bevy::prelude::*;

use crate::{
    mass_shift::{InternalMassState, MassShiftTuning, clamp_to_horizontal_disk},
    physics::{AiRacer, CourseRoute},
    race::{RaceState, RacerProgress},
};

pub fn set_ai_mass_intent(
    race: Res<RaceState>,
    route: Res<CourseRoute>,
    tuning: Res<MassShiftTuning>,
    mut racers: Query<(&Transform, &RacerProgress, &mut InternalMassState), With<AiRacer>>,
) {
    for (transform, progress, mut mass) in &mut racers {
        if !race.is_racing() {
            mass.requested_world = Vec3::ZERO;
            continue;
        }

        let target = route
            .gates
            .get(progress.next_gate)
            .map(|gate| gate.center)
            .or_else(|| route.finish.map(|finish| finish.center));
        let direction = target.map_or(Vec3::ZERO, |point| {
            (point - transform.translation)
                .with_y(0.0)
                .normalize_or_zero()
        });
        mass.requested_world = clamp_to_horizontal_disk(
            direction * tuning.displacement_radius_metres,
            tuning.displacement_radius_metres,
        );
    }
}
