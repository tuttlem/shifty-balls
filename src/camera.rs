//! A narrow observation camera for the unattended physics experiment.

use bevy::prelude::*;

use crate::physics::{ObservationCamera, PlayerBall};

const CAMERA_OFFSET: Vec3 = Vec3::new(7.0, 5.0, -9.0);
const LOOK_AHEAD: Vec3 = Vec3::new(0.0, 0.0, 4.0);
const FOLLOW_RATE: f32 = 5.0;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        ObservationCamera,
        Camera3d::default(),
        Transform::from_translation(CAMERA_OFFSET).looking_at(LOOK_AHEAD, Vec3::Y),
    ));
}

pub fn follow_ball(
    time: Res<Time>,
    ball: Single<&Transform, With<PlayerBall>>,
    mut camera: Single<&mut Transform, (With<ObservationCamera>, Without<PlayerBall>)>,
) {
    let desired_position = ball.translation + CAMERA_OFFSET;
    let smoothing = 1.0 - (-FOLLOW_RATE * time.delta_secs()).exp();

    camera.translation = camera.translation.lerp(desired_position, smoothing);
    camera.look_at(ball.translation + LOOK_AHEAD, Vec3::Y);
}
