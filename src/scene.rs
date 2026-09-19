//! Primitive geometry for observing the first gravity-driven rolling ball.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::attempt::DevelopmentDisplay;
use crate::mass_shift::{InternalMassState, MassShiftTuning};
use crate::physics::{
    BALL_FRICTION, BALL_MASS_KILOGRAMS, BALL_RADIUS_METRES, BALL_RESTITUTION, PlayerBall,
    course_start_transform,
};

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ball_material = materials.add(Color::srgb(0.9, 0.28, 0.06));
    let marker_material = materials.add(Color::srgb(1.0, 0.92, 0.55));

    commands.spawn((
        DirectionalLight {
            illuminance: 12_000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, -0.7, 0.0)),
    ));
    commands.spawn((
        PointLight {
            intensity: 500_000.0,
            range: 35.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-4.0, 10.0, -4.0),
    ));

    let ball = commands
        .spawn((
            PlayerBall,
            RigidBody::Dynamic,
            Collider::sphere(BALL_RADIUS_METRES),
            Mass(BALL_MASS_KILOGRAMS),
            // The mass-shift experiment updates this explicit local property
            // before physics preparation; ZERO is the neutral starting balance.
            CenterOfMass::ZERO,
            Friction::new(BALL_FRICTION),
            Restitution::new(BALL_RESTITUTION),
            TransformInterpolation,
            Mesh3d(meshes.add(Sphere::new(BALL_RADIUS_METRES).mesh().uv(32, 18))),
            MeshMaterial3d(ball_material),
            course_start_transform(),
        ))
        .id();

    // This marker travels around the sphere with its physical rotation, making
    // rolling observable without introducing an art asset or gameplay state.
    commands.entity(ball).with_children(|parent| {
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.09).mesh().uv(12, 8))),
            MeshMaterial3d(marker_material),
            Transform::from_xyz(0.0, BALL_RADIUS_METRES, 0.0),
        ));
    });
}

/// Draws an intentionally plain, world-stable explanation of the experimental
/// mass state. These gizmos are not children of the rolling mesh, so the
/// player's directions do not become visually scrambled by shell rotation.
pub fn draw_mass_shift_display(
    mut gizmos: Gizmos,
    ball: Single<(&Transform, &LinearVelocity), With<PlayerBall>>,
    state: Res<InternalMassState>,
    tuning: Res<MassShiftTuning>,
    display: Res<DevelopmentDisplay>,
) {
    if !display.visible {
        return;
    }

    let (ball, velocity) = ball.into_inner();
    let centre = ball.translation;
    let current = centre + state.current_world;
    let target = centre + state.requested_world;
    let boundary_rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);

    gizmos.sphere(Isometry3d::new(centre, Quat::IDENTITY), 0.055, Color::WHITE);
    gizmos.sphere(
        Isometry3d::new(current, Quat::IDENTITY),
        0.10,
        Color::srgb(1.0, 0.15, 0.15),
    );
    gizmos.sphere(
        Isometry3d::new(target, Quat::IDENTITY),
        0.065,
        Color::srgba(0.15, 0.95, 1.0, 0.55),
    );
    gizmos.circle(
        Isometry3d::new(centre + Vec3::Y * 0.015, boundary_rotation),
        tuning.displacement_radius_metres,
        Color::srgb(0.3, 0.8, 1.0),
    );
    gizmos.line(centre, current, Color::srgb(1.0, 0.15, 0.15));
    gizmos.line(centre, target, Color::srgba(0.15, 0.95, 1.0, 0.55));
    let velocity_cue = velocity.0.clamp_length_max(4.0);
    gizmos.line(centre, centre + velocity_cue, Color::srgb(0.35, 1.0, 0.35));
}
