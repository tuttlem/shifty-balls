//! Focused human control alternatives for the post-First-Race experiment.
//!
//! All models consume the same world-horizontal intent. Only the selected model
//! may affect a comparison attempt: SHIFT changes centre of mass elsewhere,
//! TORQUE supplies a roll torque, and FORCE supplies a centre-of-mass force.

use avian3d::prelude::{Forces, WriteRigidBodyForces};
use bevy::prelude::*;

use crate::{comparison::SessionMode, physics::HumanRacer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ControlModel {
    #[default]
    Shift,
    Torque,
    Force,
}

impl ControlModel {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Shift => "SHIFT",
            Self::Torque => "TORQUE",
            Self::Force => "FORCE",
        }
    }

    pub const fn index(self) -> usize {
        match self {
            Self::Shift => 0,
            Self::Torque => 1,
            Self::Force => 2,
        }
    }
}

#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct ControlIntent {
    pub world_direction: Vec3,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct ControlModelTuning {
    pub torque_newton_metres: f32,
    pub force_newtons: f32,
}

impl Default for ControlModelTuning {
    fn default() -> Self {
        Self {
            torque_newton_metres: 3.0,
            force_newtons: 3.0,
        }
    }
}

/// The shared, spin-independent experiment frame: W is +Z, A is -X, S is -Z,
/// and D is +X. This is intentionally not camera- or ball-local.
pub fn input_direction(forward: bool, left: bool, backward: bool, right: bool) -> Vec3 {
    let x = f32::from(right) - f32::from(left);
    let z = f32::from(forward) - f32::from(backward);
    Vec3::new(x, 0.0, z).normalize_or_zero()
}

pub fn torque_for_intent(intent: Vec3, tuning: ControlModelTuning) -> Vec3 {
    Vec3::Y.cross(intent) * tuning.torque_newton_metres
}

pub fn force_for_intent(intent: Vec3, tuning: ControlModelTuning) -> Vec3 {
    intent * tuning.force_newtons
}

pub fn read_keyboard_intent(keys: Res<ButtonInput<KeyCode>>, mut intent: ResMut<ControlIntent>) {
    intent.world_direction = input_direction(
        keys.pressed(KeyCode::KeyW),
        keys.pressed(KeyCode::KeyA),
        keys.pressed(KeyCode::KeyS),
        keys.pressed(KeyCode::KeyD),
    );
}

/// Applies exactly one selected non-SHIFT control effect for this physics step.
/// Avian clears these one-step inputs after integration; no velocity or transform
/// is written here.
pub fn apply_selected_physics(
    session: Res<SessionMode>,
    intent: Res<ControlIntent>,
    tuning: Res<ControlModelTuning>,
    mut human: Query<Forces, With<HumanRacer>>,
) {
    let Some(model) = session.running_comparison_model() else {
        return;
    };
    let request = intent.world_direction;
    if request == Vec3::ZERO {
        return;
    }

    for mut forces in &mut human {
        match model {
            ControlModel::Shift => {}
            ControlModel::Torque => forces.apply_torque(torque_for_intent(request, *tuning)),
            ControlModel::Force => forces.apply_force(force_for_intent(request, *tuning)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_uses_the_stable_documented_frame() {
        assert_eq!(input_direction(true, false, false, false), Vec3::Z);
        assert_eq!(input_direction(false, true, false, false), Vec3::NEG_X);
        assert_eq!(input_direction(false, false, true, false), Vec3::NEG_Z);
        assert_eq!(input_direction(false, false, false, true), Vec3::X);
        assert_eq!(
            input_direction(true, false, false, true),
            Vec3::new(1.0, 0.0, 1.0).normalize()
        );
    }

    #[test]
    fn torque_roll_axis_matches_the_requested_direction() {
        let tuning = ControlModelTuning::default();
        assert_eq!(torque_for_intent(Vec3::Z, tuning), Vec3::X * 3.0);
        assert_eq!(torque_for_intent(Vec3::ZERO, tuning), Vec3::ZERO);
    }

    #[test]
    fn force_is_horizontal_and_neutral_when_input_is_neutral() {
        let tuning = ControlModelTuning::default();
        assert_eq!(force_for_intent(Vec3::X, tuning), Vec3::X * 3.0);
        assert_eq!(force_for_intent(Vec3::ZERO, tuning), Vec3::ZERO);
    }

    #[test]
    fn labels_and_indices_are_unique() {
        assert_eq!(ControlModel::Shift.label(), "SHIFT");
        assert_eq!(ControlModel::Torque.index(), 1);
        assert_eq!(ControlModel::Force.index(), 2);
    }
}
