//! Shared physical conventions for the first rolling-ball experiment.
//!
//! These values are intentionally visible rather than hidden in a controller:
//! gravity and contact with static geometry are the only sources of the ball's
//! initial motion.

use avian3d::prelude::{Gravity, PhysicsPlugins};
use bevy::prelude::*;

pub const GRAVITY_METRES_PER_SECOND_SQUARED: f32 = 9.81;
pub const BALL_RADIUS_METRES: f32 = 0.5;
pub const BALL_MASS_KILOGRAMS: f32 = 1.0;
pub const BALL_SHELL_MASS_KILOGRAMS: f32 = 0.55;
pub const INTERNAL_MASS_KILOGRAMS: f32 = 0.45;
pub const INTERNAL_MASS_DISPLACEMENT_RADIUS_METRES: f32 = 0.30;
pub const INTERNAL_MASS_MOVEMENT_SPEED_METRES_PER_SECOND: f32 = 1.5;
pub const BALL_FRICTION: f32 = 0.8;
pub const BALL_RESTITUTION: f32 = 0.1;
pub const TRACK_FRICTION: f32 = 0.9;
pub const TRACK_RESTITUTION: f32 = 0.05;

pub const RACER_COUNT: usize = 4;
pub const COUNTDOWN_SECONDS: f32 = 3.0;

#[derive(Component)]
pub struct Racer {
    pub id: u8,
}

#[derive(Component)]
pub struct HumanRacer;

#[derive(Component)]
pub struct AiRacer;

/// Racers that participate only in the accepted multi-ball race. Comparison
/// attempts remove their collider and body rather than hiding visual traffic
/// that could still change the human experiment.
#[derive(Component)]
pub struct RaceOnly;

#[derive(Component, Clone, Copy)]
pub struct RacerStart {
    pub transform: Transform,
}

#[derive(Component)]
pub struct CourseTrack;

#[derive(Component, Debug, Clone, Copy)]
pub struct FinishRegion {
    pub center: Vec3,
    pub half_extents: Vec3,
}

#[derive(Clone, Copy, Debug)]
pub struct ProgressionGate {
    pub center: Vec3,
    pub forward: Vec3,
    pub half_width: f32,
    pub half_height: f32,
    pub half_depth: f32,
}

impl ProgressionGate {
    pub fn contains(self, position: Vec3) -> bool {
        let forward = self.forward.normalize_or_zero();
        let right = Vec3::new(forward.z, 0.0, -forward.x);
        let offset = position - self.center;

        offset.dot(right).abs() <= self.half_width
            && offset.y.abs() <= self.half_height
            && offset.dot(forward).abs() <= self.half_depth
    }
}

#[derive(Resource, Default)]
pub struct CourseRoute {
    pub gates: Vec<ProgressionGate>,
    pub finish: Option<FinishRegion>,
}

#[derive(Component)]
pub struct ObservationCamera;

pub fn configure(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec3::NEG_Y * GRAVITY_METRES_PER_SECOND_SQUARED));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broad_gate_accepts_outer_racing_lines_but_rejects_a_bypass() {
        let gate = ProgressionGate {
            center: Vec3::ZERO,
            forward: Vec3::Z,
            half_width: 7.5,
            half_height: 2.0,
            half_depth: 2.5,
        };
        assert!(gate.contains(Vec3::new(7.25, 0.0, 0.0)));
        assert!(!gate.contains(Vec3::new(7.6, 0.0, 0.0)));
        assert!(!gate.contains(Vec3::new(0.0, 0.0, 2.6)));
    }
}
