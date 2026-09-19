//! Primitive scene setup and racer visuals for the first physical race.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    mass_shift::{InternalMassState, MassShiftTuning},
    physics::{
        AiRacer, BALL_FRICTION, BALL_MASS_KILOGRAMS, BALL_RADIUS_METRES, BALL_RESTITUTION,
        HumanRacer, Racer, RacerStart,
    },
    race::{DevelopmentDisplay, RacerProgress},
};

const STARTS: [Vec3; 4] = [
    Vec3::new(-2.0, 5.5, -20.0),
    Vec3::new(2.0, 5.5, -20.0),
    Vec3::new(-2.0, 5.5, -18.0),
    Vec3::new(2.0, 5.5, -18.0),
];

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    let racer_mesh = meshes.add(Sphere::new(BALL_RADIUS_METRES).mesh().uv(32, 18));
    let marker_mesh = meshes.add(Sphere::new(0.09).mesh().uv(12, 8));
    let colours = [
        Color::srgb(0.9, 0.28, 0.06),
        Color::srgb(0.18, 0.75, 0.95),
        Color::srgb(0.78, 0.18, 0.76),
        Color::srgb(0.28, 0.9, 0.38),
    ];
    let racer_materials: Vec<_> = colours
        .into_iter()
        .map(|colour| materials.add(colour))
        .collect();
    let marker_material = materials.add(Color::srgb(1.0, 0.92, 0.55));

    for (index, start) in STARTS.into_iter().enumerate() {
        let transform = Transform::from_translation(start);
        let mut entity = commands.spawn((
            Racer { id: index as u8 },
            RacerStart { transform },
            RigidBody::Kinematic,
            Collider::sphere(BALL_RADIUS_METRES),
            Mass(BALL_MASS_KILOGRAMS),
            CenterOfMass::ZERO,
            Friction::new(BALL_FRICTION),
            Restitution::new(BALL_RESTITUTION),
            TransformInterpolation,
            InternalMassState::default(),
            RacerProgress::default(),
            Mesh3d(racer_mesh.clone()),
            MeshMaterial3d(racer_materials[index].clone()),
            transform,
        ));

        if index == 0 {
            entity.insert(HumanRacer).with_children(|parent| {
                parent.spawn((
                    Mesh3d(marker_mesh.clone()),
                    MeshMaterial3d(marker_material.clone()),
                    Transform::from_xyz(0.0, BALL_RADIUS_METRES, 0.0),
                ));
            });
        } else {
            entity.insert(AiRacer);
        }
    }
}

/// Draws only the human racer’s mass state. Showing every opponent’s target
/// would conceal the player-facing experiment rather than explain it.
pub fn draw_mass_shift_display(
    mut gizmos: Gizmos,
    human: Single<(&Transform, &LinearVelocity, &InternalMassState), With<HumanRacer>>,
    tuning: Res<MassShiftTuning>,
    display: Res<DevelopmentDisplay>,
) {
    if !display.visible {
        return;
    }

    let (ball, velocity, state) = human.into_inner();
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
    gizmos.line(
        centre,
        centre + velocity.0.clamp_length_max(4.0),
        Color::srgb(0.35, 1.0, 0.35),
    );
}
