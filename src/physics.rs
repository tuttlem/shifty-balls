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

pub const SLOPE_ANGLE_RADIANS: f32 = 0.314_159_27;
pub const SLOPE_CENTER: Vec3 = Vec3::new(0.0, 1.664, -2.0);
pub const BALL_START_POSITION: Vec3 = Vec3::new(0.0, 3.65, -6.0);

#[derive(Component)]
pub struct PlayerBall;

#[derive(Component)]
pub struct PhysicsTestTrack;

#[derive(Component)]
pub struct ObservationCamera;

pub fn configure(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec3::NEG_Y * GRAVITY_METRES_PER_SECOND_SQUARED));
}
