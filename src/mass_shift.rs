//! The first deliberately small internal-mass experiment.
//!
//! Input moves a conceptual weight in stable world X/Z coordinates. The only
//! coupling to the rolling shell is Avian's local [`CenterOfMass`] property;
//! gravity and contact decide every resulting movement.

use avian3d::prelude::{CenterOfMass, Rotation, WakeBody};
use bevy::prelude::*;

use crate::physics::{
    BALL_MASS_KILOGRAMS, BALL_SHELL_MASS_KILOGRAMS, HumanRacer,
    INTERNAL_MASS_DISPLACEMENT_RADIUS_METRES, INTERNAL_MASS_KILOGRAMS,
    INTERNAL_MASS_MOVEMENT_SPEED_METRES_PER_SECOND, Racer,
};
use crate::race::RaceState;

/// Deliberately centralised prototype values, kept as a resource rather than a
/// configuration framework so the first control experiment remains quick to tune.
#[derive(Resource, Debug, Clone, Copy)]
pub struct MassShiftTuning {
    pub shell_mass_kilograms: f32,
    pub inner_mass_kilograms: f32,
    pub total_mass_kilograms: f32,
    pub displacement_radius_metres: f32,
    pub movement_speed_metres_per_second: f32,
}

impl Default for MassShiftTuning {
    fn default() -> Self {
        Self {
            shell_mass_kilograms: BALL_SHELL_MASS_KILOGRAMS,
            inner_mass_kilograms: INTERNAL_MASS_KILOGRAMS,
            total_mass_kilograms: BALL_MASS_KILOGRAMS,
            displacement_radius_metres: INTERNAL_MASS_DISPLACEMENT_RADIUS_METRES,
            movement_speed_metres_per_second: INTERNAL_MASS_MOVEMENT_SPEED_METRES_PER_SECOND,
        }
    }
}

impl MassShiftTuning {
    /// The conceptual shell and inner weights intentionally account for the
    /// one-kilogram Avian body as a useful tuning invariant.
    pub fn inner_mass_ratio(self) -> f32 {
        debug_assert!(
            ((self.shell_mass_kilograms + self.inner_mass_kilograms) - self.total_mass_kilograms)
                .abs()
                < f32::EPSILON
        );
        self.inner_mass_kilograms / self.total_mass_kilograms
    }
}

/// Project-owned state for the conceptual mass. Both positions are world
/// horizontal offsets from the ball centre, never ball-local directions.
#[derive(Component, Debug, Default)]
pub struct InternalMassState {
    pub requested_world: Vec3,
    pub current_world: Vec3,
}

/// Converts the four temporary keyboard directions into the stable experiment
/// frame: W is down-track (+Z), A is track-left (-X), S is up-track (-Z), and
/// D is track-right (+X). Diagonals are normalised rather than made stronger.
pub fn input_direction(forward: bool, left: bool, backward: bool, right: bool) -> Vec3 {
    let x = f32::from(right) - f32::from(left);
    let z = f32::from(forward) - f32::from(backward);
    Vec3::new(x, 0.0, z).normalize_or_zero()
}

/// Keeps a requested or current conceptual mass in its physically sensible
/// horizontal disk, independent of any rotation of the rendered shell.
pub fn clamp_to_horizontal_disk(position: Vec3, radius: f32) -> Vec3 {
    let horizontal = Vec2::new(position.x, position.z).clamp_length_max(radius);
    Vec3::new(horizontal.x, 0.0, horizontal.y)
}

/// Moves continuously toward an inner-mass target without overshoot.
pub fn move_toward(current: Vec3, target: Vec3, maximum_distance: f32) -> Vec3 {
    let offset = target - current;
    let distance = offset.length();
    if distance <= maximum_distance || distance == 0.0 {
        target
    } else {
        current + offset / distance * maximum_distance
    }
}

/// Calculates the combined body's world COM displacement caused by the inner
/// conceptual mass while keeping the existing one-kilogram rigid body intact.
pub fn combined_com_world(inner_world: Vec3, tuning: MassShiftTuning) -> Vec3 {
    inner_world * tuning.inner_mass_ratio()
}

/// Avian stores a centre of mass in the body's rotating local frame. Player
/// intent remains world-relative, so invert the shell rotation only at this
/// narrow physics boundary.
pub fn com_world_to_local(com_world: Vec3, body_rotation: Quat) -> Vec3 {
    body_rotation.inverse() * com_world
}

pub fn read_keyboard_intent(
    keys: Res<ButtonInput<KeyCode>>,
    tuning: Res<MassShiftTuning>,
    race: Res<RaceState>,
    mut state: Single<&mut InternalMassState, With<HumanRacer>>,
) {
    if !race.is_racing() {
        state.requested_world = Vec3::ZERO;
        return;
    }

    let direction = input_direction(
        keys.pressed(KeyCode::KeyW),
        keys.pressed(KeyCode::KeyA),
        keys.pressed(KeyCode::KeyS),
        keys.pressed(KeyCode::KeyD),
    );

    // Releasing every key deliberately returns the target to neutral instead
    // of leaving an invisible control state behind.
    state.requested_world = clamp_to_horizontal_disk(
        direction * tuning.displacement_radius_metres,
        tuning.displacement_radius_metres,
    );
}

pub fn advance_internal_mass(
    time: Res<Time>,
    tuning: Res<MassShiftTuning>,
    race: Res<RaceState>,
    mut states: Query<&mut InternalMassState, With<Racer>>,
) {
    for mut state in &mut states {
        if !race.is_racing() {
            state.requested_world = Vec3::ZERO;
            state.current_world = Vec3::ZERO;
            continue;
        }

        let maximum_distance = tuning.movement_speed_metres_per_second * time.delta_secs();
        state.current_world = clamp_to_horizontal_disk(
            move_toward(state.current_world, state.requested_world, maximum_distance),
            tuning.displacement_radius_metres,
        );
    }
}

/// The sole gameplay-to-physics coupling. This intentionally does not touch
/// velocities, orientation, forces, torques, impulses, or traction: Avian's
/// gravity and contact solver produce the ball's response after this update.
pub fn apply_center_of_mass(
    mut commands: Commands,
    tuning: Res<MassShiftTuning>,
    mut racers: Query<(Entity, &Rotation, &InternalMassState, &mut CenterOfMass), With<Racer>>,
) {
    for (entity, rotation, state, mut center_of_mass) in &mut racers {
        let world_offset = combined_com_world(state.current_world, *tuning);
        let local_offset = com_world_to_local(world_offset, **rotation);
        **center_of_mass = local_offset;

        // Avian does not treat a changed centre of mass as an automatic wake
        // condition. Without this, a resting racer can ignore a player-visible
        // weight shift indefinitely. Waking is not a movement force: contact
        // and gravity still create every resulting motion.
        if world_offset != Vec3::ZERO {
            commands.queue(WakeBody(entity));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cardinal_input_uses_the_documented_world_frame() {
        assert_eq!(input_direction(true, false, false, false), Vec3::Z);
        assert_eq!(input_direction(false, true, false, false), Vec3::NEG_X);
        assert_eq!(input_direction(false, false, true, false), Vec3::NEG_Z);
        assert_eq!(input_direction(false, false, false, true), Vec3::X);
    }

    #[test]
    fn diagonal_input_is_normalised() {
        assert_eq!(
            input_direction(true, false, false, true),
            Vec3::new(1.0, 0.0, 1.0).normalize()
        );
    }

    #[test]
    fn displacement_stays_in_the_horizontal_disk() {
        assert!(
            clamp_to_horizontal_disk(Vec3::new(0.6, 4.0, 0.8), 0.3)
                .abs_diff_eq(Vec3::new(0.18, 0.0, 0.24), 0.000_01)
        );
    }

    #[test]
    fn inner_mass_moves_without_overshoot() {
        assert_eq!(
            move_toward(Vec3::ZERO, Vec3::X, 0.25),
            Vec3::new(0.25, 0.0, 0.0)
        );
        assert_eq!(
            move_toward(Vec3::new(0.9, 0.0, 0.0), Vec3::X, 0.25),
            Vec3::X
        );
    }

    #[test]
    fn mass_ratio_derives_combined_com_offset() {
        let tuning = MassShiftTuning::default();
        assert_eq!(
            combined_com_world(Vec3::new(0.3, 0.0, 0.0), tuning),
            Vec3::new(0.135, 0.0, 0.0)
        );
    }

    #[test]
    fn world_com_is_inverse_rotated_into_body_space() {
        assert_eq!(com_world_to_local(Vec3::Z, Quat::IDENTITY), Vec3::Z);

        let quarter_turn = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        assert!(com_world_to_local(Vec3::Z, quarter_turn).abs_diff_eq(Vec3::NEG_X, 0.000_01));
    }
}
