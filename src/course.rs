//! One disposable, hand-authored course for the first control kill test.
//!
//! This is deliberately a short list of primitive surfaces, not a track
//! language. Every rendered surface owns an identical static collider so the
//! player can trust what they are looking at.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::physics::{
    CourseRoute, CourseTrack, FinishRegion, ProgressionGate, TRACK_FRICTION, TRACK_RESTITUTION,
};

const SURFACE_THICKNESS: f32 = 0.5;
const SURFACE_OVERLAP: f32 = 0.3;
const RETAINING_WALL_THICKNESS: f32 = 0.4;
const RETAINING_WALL_HEIGHT: f32 = 1.5;
const COURSE_WIDTH: f32 = 12.0;
const RACING_SECTION_WIDTH: f32 = 16.0;
const RACING_SECTION_GATE_HALF_WIDTH: f32 = 7.5;
const DEFAULT_GATE_HALF_WIDTH: f32 = 5.0;

#[derive(Clone, Copy)]
struct SurfaceSpec {
    width: f32,
    logical_length: f32,
    rotation: Quat,
    retaining_walls: bool,
}

impl SurfaceSpec {
    fn new(width: f32, logical_length: f32, rotation: Quat) -> Self {
        Self {
            width,
            logical_length,
            rotation,
            retaining_walls: true,
        }
    }
}

pub fn setup_course(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let opening = materials.add(Color::srgb(0.08, 0.30, 0.38));
    let gentle = materials.add(Color::srgb(0.07, 0.40, 0.46));
    let bank = materials.add(Color::srgb(0.11, 0.36, 0.56));
    let momentum = materials.add(Color::srgb(0.16, 0.46, 0.42));
    let boundary = materials.add(Color::srgb(0.04, 0.11, 0.18));
    let finish = materials.add(Color::srgb(0.85, 0.82, 0.18));

    let mut cursor = Vec3::new(0.0, 5.0, -22.0);
    let mut gates = Vec::new();
    cursor = spawn_surface(
        &mut commands,
        &mut meshes,
        opening.clone(),
        boundary.clone(),
        cursor,
        SurfaceSpec::new(COURSE_WIDTH, 8.0, Quat::IDENTITY),
    );
    cursor = spawn_surface(
        &mut commands,
        &mut meshes,
        opening,
        boundary.clone(),
        cursor,
        SurfaceSpec::new(COURSE_WIDTH, 16.0, surface_rotation(0.0, 12.0, 0.0)),
    );
    gates.push(progression_gate(cursor, surface_rotation(0.0, 12.0, 0.0)));

    // The forgiving bend establishes that changing mass position changes line
    // before the stronger bank asks the player to anticipate it.
    for (yaw, bank_degrees) in [(4.0, 2.0), (8.0, 4.0), (12.0, 5.0), (16.0, 6.0)] {
        cursor = spawn_surface(
            &mut commands,
            &mut meshes,
            gentle.clone(),
            boundary.clone(),
            cursor,
            SurfaceSpec::new(COURSE_WIDTH, 5.0, surface_rotation(yaw, 3.0, bank_degrees)),
        );
    }
    gates.push(progression_gate(cursor, surface_rotation(16.0, 3.0, 6.0)));

    // This deliberately broad bank and run-out are the physical-racing
    // experiment. The centre-seeking AI leaves a high/low line for the player,
    // while continuous walls make a traffic displacement consequential but
    // recoverable without a reset or collision-specific assistance.
    for (yaw, bank_degrees) in [
        (8.0, -6.0),
        (0.0, -9.0),
        (-8.0, -12.0),
        (-16.0, -14.0),
        (-24.0, -12.0),
        (-32.0, -9.0),
    ] {
        cursor = spawn_surface(
            &mut commands,
            &mut meshes,
            bank.clone(),
            boundary.clone(),
            cursor,
            SurfaceSpec::new(
                RACING_SECTION_WIDTH,
                4.0,
                surface_rotation(yaw, 2.0, bank_degrees),
            ),
        );
    }
    gates.push(progression_gate(cursor, surface_rotation(-32.0, 2.0, -9.0)));

    cursor = spawn_surface(
        &mut commands,
        &mut meshes,
        momentum.clone(),
        boundary.clone(),
        cursor,
        SurfaceSpec::new(
            RACING_SECTION_WIDTH,
            12.0,
            surface_rotation(-32.0, 6.0, 0.0),
        ),
    );
    cursor = spawn_surface(
        &mut commands,
        &mut meshes,
        momentum.clone(),
        boundary.clone(),
        cursor,
        SurfaceSpec::new(
            RACING_SECTION_WIDTH,
            12.0,
            surface_rotation(-32.0, -9.0, 0.0),
        ),
    );
    cursor = spawn_surface(
        &mut commands,
        &mut meshes,
        momentum,
        boundary.clone(),
        cursor,
        SurfaceSpec::new(
            RACING_SECTION_WIDTH,
            8.0,
            surface_rotation(-32.0, -2.0, 0.0),
        ),
    );
    gates.push(progression_gate_with_width(
        cursor,
        surface_rotation(-32.0, -2.0, 0.0),
        RACING_SECTION_GATE_HALF_WIDTH,
    ));
    let finish_start = cursor;
    let finish_rotation = surface_rotation(-32.0, 4.0, 0.0);
    let finish_end = spawn_surface(
        &mut commands,
        &mut meshes,
        finish.clone(),
        boundary,
        finish_start,
        SurfaceSpec::new(10.0, 18.0, finish_rotation),
    );

    let finish_center = finish_start + finish_rotation * Vec3::Z * 13.0 + Vec3::Y * 1.0;
    let finish_region = FinishRegion {
        center: finish_center,
        half_extents: Vec3::splat(4.0),
    };
    commands.spawn(finish_region);
    commands.insert_resource(CourseRoute {
        gates,
        finish: Some(finish_region),
    });
    spawn_finish_gate(
        &mut commands,
        &mut meshes,
        finish,
        finish_center,
        finish_rotation,
    );

    // Retain this endpoint calculation in the source to make the final apron
    // route explicit while avoiding an accidental visual-only course extension.
    let _course_end = finish_end;
}

fn progression_gate(center: Vec3, rotation: Quat) -> ProgressionGate {
    progression_gate_with_width(center, rotation, DEFAULT_GATE_HALF_WIDTH)
}

fn progression_gate_with_width(center: Vec3, rotation: Quat, half_width: f32) -> ProgressionGate {
    ProgressionGate {
        center: center + Vec3::Y,
        forward: (rotation * Vec3::Z).with_y(0.0).normalize_or_zero(),
        half_width,
        half_height: 4.0,
        half_depth: 2.5,
    }
}

fn spawn_surface(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: Handle<StandardMaterial>,
    boundary_material: Handle<StandardMaterial>,
    start_top: Vec3,
    spec: SurfaceSpec,
) -> Vec3 {
    let transform = surface_transform(start_top, spec.logical_length, spec.rotation);
    let collider_length = spec.logical_length + SURFACE_OVERLAP;

    commands.spawn((
        CourseTrack,
        RigidBody::Static,
        Collider::cuboid(spec.width, SURFACE_THICKNESS, collider_length),
        Friction::new(TRACK_FRICTION),
        Restitution::new(TRACK_RESTITUTION),
        Mesh3d(meshes.add(Cuboid::new(spec.width, SURFACE_THICKNESS, collider_length))),
        MeshMaterial3d(material),
        transform,
    ));

    if spec.retaining_walls {
        for side in [-1.0, 1.0] {
            let local_position = Vec3::new(
                side * (spec.width * 0.5 + RETAINING_WALL_THICKNESS * 0.5),
                RETAINING_WALL_HEIGHT * 0.5,
                spec.logical_length * 0.5,
            );
            commands.spawn((
                CourseTrack,
                RigidBody::Static,
                Collider::cuboid(
                    RETAINING_WALL_THICKNESS,
                    RETAINING_WALL_HEIGHT,
                    collider_length,
                ),
                Friction::new(TRACK_FRICTION),
                Restitution::new(TRACK_RESTITUTION),
                Mesh3d(meshes.add(Cuboid::new(
                    RETAINING_WALL_THICKNESS,
                    RETAINING_WALL_HEIGHT,
                    collider_length,
                ))),
                MeshMaterial3d(boundary_material.clone()),
                Transform::from_translation(start_top + spec.rotation * local_position)
                    .with_rotation(spec.rotation),
            ));
        }
    }

    advance_surface_start(start_top, spec.logical_length, spec.rotation)
}

fn spawn_finish_gate(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: Handle<StandardMaterial>,
    centre: Vec3,
    rotation: Quat,
) {
    for side in [-1.0, 1.0] {
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.35, 3.5, 0.35))),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(centre + rotation * Vec3::X * side * 4.0 + Vec3::Y * 0.75)
                .with_rotation(rotation),
        ));
    }
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(8.35, 0.35, 0.35))),
        MeshMaterial3d(material),
        Transform::from_translation(centre + Vec3::Y * 2.5).with_rotation(rotation),
    ));
}

pub fn advance_surface_start(start_top: Vec3, logical_length: f32, rotation: Quat) -> Vec3 {
    start_top + rotation * Vec3::Z * logical_length
}

pub fn surface_transform(start_top: Vec3, logical_length: f32, rotation: Quat) -> Transform {
    Transform::from_translation(
        start_top
            + rotation * (Vec3::Z * (logical_length * 0.5) - Vec3::Y * (SURFACE_THICKNESS * 0.5)),
    )
    .with_rotation(rotation)
}

fn surface_rotation(yaw_degrees: f32, pitch_degrees: f32, bank_degrees: f32) -> Quat {
    Quat::from_rotation_y(yaw_degrees.to_radians())
        * Quat::from_rotation_x(pitch_degrees.to_radians())
        * Quat::from_rotation_z(bank_degrees.to_radians())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_advances_along_the_surface_travel_axis() {
        assert_eq!(
            advance_surface_start(Vec3::new(1.0, 2.0, 3.0), 4.0, Quat::IDENTITY),
            Vec3::new(1.0, 2.0, 7.0)
        );
    }

    #[test]
    fn surface_transform_places_the_top_at_the_logical_start() {
        let transform = surface_transform(Vec3::new(0.0, 5.0, -2.0), 10.0, Quat::IDENTITY);
        assert_eq!(transform.translation, Vec3::new(0.0, 4.75, 3.0));
    }

    #[test]
    fn racing_section_gate_accepts_a_full_width_line_without_accepting_a_shortcut() {
        let gate =
            progression_gate_with_width(Vec3::ZERO, Quat::IDENTITY, RACING_SECTION_GATE_HALF_WIDTH);
        assert!(gate.contains(Vec3::new(7.25, 1.0, 0.5)));
        assert!(!gate.contains(Vec3::new(7.6, 1.0, 0.0)));
        assert!(!gate.contains(Vec3::new(0.0, 1.0, 3.0)));
    }
}
