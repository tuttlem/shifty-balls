//! Primitive geometry for observing the first gravity-driven rolling ball.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::mass_shift::{InternalMassState, MassShiftTuning};
use crate::physics::{
    BALL_FRICTION, BALL_MASS_KILOGRAMS, BALL_RADIUS_METRES, BALL_RESTITUTION, BALL_START_POSITION,
    PhysicsTestTrack, PlayerBall, SLOPE_ANGLE_RADIANS, SLOPE_CENTER, TRACK_FRICTION,
    TRACK_RESTITUTION,
};

const TRACK_WIDTH: f32 = 8.0;
const TRACK_THICKNESS: f32 = 0.4;

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let track_material = materials.add(Color::srgb(0.08, 0.28, 0.34));
    let edge_material = materials.add(Color::srgb(0.05, 0.12, 0.18));
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

    spawn_track_piece(
        &mut commands,
        &mut meshes,
        track_material.clone(),
        Vec3::new(TRACK_WIDTH, TRACK_THICKNESS, 5.0),
        Transform::from_xyz(0.0, 3.508, -10.5),
    );
    spawn_track_piece(
        &mut commands,
        &mut meshes,
        track_material.clone(),
        Vec3::new(TRACK_WIDTH, TRACK_THICKNESS, 12.0),
        Transform::from_translation(SLOPE_CENTER)
            .with_rotation(Quat::from_rotation_x(SLOPE_ANGLE_RADIANS)),
    );
    spawn_track_piece(
        &mut commands,
        &mut meshes,
        track_material,
        Vec3::new(TRACK_WIDTH, TRACK_THICKNESS, 18.0),
        Transform::from_xyz(0.0, -TRACK_THICKNESS * 0.5, 12.0),
    );

    for x in [-4.25, 4.25] {
        spawn_track_piece(
            &mut commands,
            &mut meshes,
            edge_material.clone(),
            Vec3::new(0.5, 1.5, 18.0),
            Transform::from_xyz(x, 0.55, 12.0),
        );
    }
    spawn_track_piece(
        &mut commands,
        &mut meshes,
        edge_material,
        Vec3::new(TRACK_WIDTH, 4.0, 0.5),
        Transform::from_xyz(0.0, 1.8, 21.0),
    );

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
            Transform::from_translation(BALL_START_POSITION),
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
    ball: Single<&Transform, With<PlayerBall>>,
    state: Res<InternalMassState>,
    tuning: Res<MassShiftTuning>,
) {
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
}

fn spawn_track_piece(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: Handle<StandardMaterial>,
    size: Vec3,
    transform: Transform,
) {
    commands.spawn((
        PhysicsTestTrack,
        RigidBody::Static,
        Collider::cuboid(size.x, size.y, size.z),
        Friction::new(TRACK_FRICTION),
        Restitution::new(TRACK_RESTITUTION),
        Mesh3d(meshes.add(Cuboid::new(size.x, size.y, size.z))),
        MeshMaterial3d(material),
        transform,
    ));
}
