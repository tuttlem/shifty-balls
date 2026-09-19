//! A narrow observation camera for the unattended physics experiment.

use bevy::prelude::*;

use crate::physics::{ObservationCamera, PlayerBall, course_start_transform};

const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 6.0, -10.0);
const LOOK_AHEAD: Vec3 = Vec3::new(0.0, 0.8, 7.0);
const FOLLOW_RATE: f32 = 5.0;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        ObservationCamera,
        Camera3d::default(),
        course_camera_transform(course_start_transform().translation),
    ));
}

pub fn snap_to_course_start(camera: &mut Transform) {
    *camera = course_camera_transform(course_start_transform().translation);
}

fn course_camera_transform(ball_position: Vec3) -> Transform {
    Transform::from_translation(ball_position + CAMERA_OFFSET)
        .looking_at(ball_position + LOOK_AHEAD, Vec3::Y)
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
