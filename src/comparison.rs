//! Human-only, repeatable control-model attempts on the existing course.

use std::time::Duration;

use avian3d::prelude::{
    AngularVelocity, CenterOfMass, Collider, LinearVelocity, RigidBody, Rotation,
};
use bevy::prelude::*;

use crate::{
    camera,
    control_model::{ControlIntent, ControlModel},
    mass_shift::InternalMassState,
    physics::{CourseRoute, HumanRacer, ObservationCamera, RaceOnly, RacerStart},
    race::{self, RaceState, RacerProgress},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveSession {
    Comparison(ControlModel),
    Race,
}

#[derive(Resource, Debug)]
pub struct SessionMode {
    pub active: ActiveSession,
}

impl Default for SessionMode {
    fn default() -> Self {
        Self {
            active: ActiveSession::Comparison(ControlModel::Shift),
        }
    }
}

impl SessionMode {
    pub fn select_comparison(&mut self, model: ControlModel) {
        self.active = ActiveSession::Comparison(model);
    }

    pub fn enter_race(&mut self) {
        self.active = ActiveSession::Race;
    }
    pub fn comparison_model(&self) -> Option<ControlModel> {
        match self.active {
            ActiveSession::Comparison(model) => Some(model),
            ActiveSession::Race => None,
        }
    }

    pub fn comparison_is_running(&self) -> bool {
        self.comparison_model().is_some()
    }

    pub fn running_comparison_model(&self) -> Option<ControlModel> {
        self.comparison_model()
    }

    pub fn is_race(&self) -> bool {
        self.active == ActiveSession::Race
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonPhase {
    Running,
    Finished,
}

#[derive(Resource, Debug)]
pub struct ComparisonState {
    pub phase: ComparisonPhase,
    pub elapsed: Duration,
    pub best_by_model: [Option<Duration>; 3],
}

impl Default for ComparisonState {
    fn default() -> Self {
        Self {
            phase: ComparisonPhase::Running,
            elapsed: Duration::ZERO,
            best_by_model: [None; 3],
        }
    }
}

impl ComparisonState {
    pub fn restart(&mut self) {
        self.phase = ComparisonPhase::Running;
        self.elapsed = Duration::ZERO;
    }

    pub fn record_finish(&mut self, model: ControlModel) {
        if self.phase != ComparisonPhase::Running {
            return;
        }
        let best = &mut self.best_by_model[model.index()];
        if best.is_none_or(|current| self.elapsed < current) {
            *best = Some(self.elapsed);
        }
        self.phase = ComparisonPhase::Finished;
    }

    pub fn best_for(&self, model: ControlModel) -> Option<Duration> {
        self.best_by_model[model.index()]
    }
}

#[derive(Component)]
pub struct ComparisonReadout;

pub fn setup_readout(mut commands: Commands) {
    commands.spawn((
        ComparisonReadout,
        Text::new("SHIFT\nTIME 0.00 s\n1/2/3 compare | 4 race | R reset | F3 debug"),
        TextFont {
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            ..default()
        },
    ));
}

type HumanResetData = (
    Entity,
    &'static RacerStart,
    &'static mut Transform,
    &'static mut Rotation,
    &'static mut LinearVelocity,
    &'static mut AngularVelocity,
    &'static mut CenterOfMass,
    &'static mut InternalMassState,
    &'static mut RacerProgress,
);

fn reset_human_comparison(
    commands: &mut Commands,
    human: Single<HumanResetData, With<HumanRacer>>,
    mut camera_transform: Single<&mut Transform, (With<ObservationCamera>, Without<HumanRacer>)>,
    mut intent: ResMut<ControlIntent>,
    comparison: &mut ComparisonState,
) {
    let (
        entity,
        start,
        mut transform,
        mut rotation,
        mut linear,
        mut angular,
        mut com,
        mut mass,
        mut progress,
    ) = human.into_inner();
    *transform = start.transform;
    **rotation = start.transform.rotation;
    *linear = LinearVelocity::ZERO;
    *angular = AngularVelocity::ZERO;
    **com = Vec3::ZERO;
    *mass = InternalMassState::default();
    *progress = RacerProgress::default();
    intent.world_direction = Vec3::ZERO;
    comparison.restart();
    commands.entity(entity).insert(RigidBody::Dynamic);
    camera::snap_to_human_position(&mut camera_transform, start.transform.translation);
}

pub fn initialise_comparison(
    mut commands: Commands,
    human: Single<HumanResetData, With<HumanRacer>>,
    camera_transform: Single<&mut Transform, (With<ObservationCamera>, Without<HumanRacer>)>,
    intent: ResMut<ControlIntent>,
    mut comparison: ResMut<ComparisonState>,
    race_only: Query<Entity, With<RaceOnly>>,
) {
    for entity in &race_only {
        commands.entity(entity).remove::<Collider>();
        commands.entity(entity).remove::<RigidBody>();
        commands.entity(entity).insert(Visibility::Hidden);
    }
    reset_human_comparison(
        &mut commands,
        human,
        camera_transform,
        intent,
        &mut comparison,
    );
}

#[expect(
    clippy::too_many_arguments,
    reason = "This focused transition coordinates the explicit resources required to normalise a physical attempt."
)]
pub fn select_or_reset_session(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut session: ResMut<SessionMode>,
    mut comparison: ResMut<ComparisonState>,
    mut race: ResMut<RaceState>,
    human: Single<HumanResetData, With<HumanRacer>>,
    camera_transform: Single<&mut Transform, (With<ObservationCamera>, Without<HumanRacer>)>,
    intent: ResMut<ControlIntent>,
    race_only: Query<Entity, With<RaceOnly>>,
) {
    let selected = [
        (KeyCode::Digit1, ControlModel::Shift),
        (KeyCode::Digit2, ControlModel::Torque),
        (KeyCode::Digit3, ControlModel::Force),
    ]
    .into_iter()
    .find_map(|(key, model)| keys.just_pressed(key).then_some(model));

    if let Some(model) = selected {
        session.select_comparison(model);
        for entity in &race_only {
            commands.entity(entity).remove::<Collider>();
            commands.entity(entity).remove::<RigidBody>();
            commands.entity(entity).insert(Visibility::Hidden);
        }
        reset_human_comparison(
            &mut commands,
            human,
            camera_transform,
            intent,
            &mut comparison,
        );
        return;
    }

    if keys.just_pressed(KeyCode::Digit4) {
        session.enter_race();
        race.request_reset();
        return;
    }

    if keys.just_pressed(KeyCode::KeyR) && session.comparison_is_running() {
        reset_human_comparison(
            &mut commands,
            human,
            camera_transform,
            intent,
            &mut comparison,
        );
    }
}

pub fn tick_comparison(
    time: Res<Time>,
    session: Res<SessionMode>,
    mut state: ResMut<ComparisonState>,
) {
    if session.comparison_is_running() && state.phase == ComparisonPhase::Running {
        state.elapsed += time.delta();
    }
}

pub fn update_progress(
    session: Res<SessionMode>,
    route: Res<CourseRoute>,
    human: Single<(&Transform, &mut RacerProgress), With<HumanRacer>>,
) {
    if !session.comparison_is_running() {
        return;
    }
    let (transform, mut progress) = human.into_inner();
    if let Some(gate) = route.gates.get(progress.next_gate)
        && gate.contains(transform.translation)
    {
        let gate_index = progress.next_gate;
        race::advance_through_gate(&mut progress, gate_index);
    }
}

pub fn check_finish(
    session: Res<SessionMode>,
    route: Res<CourseRoute>,
    human: Single<(&Transform, &RacerProgress), With<HumanRacer>>,
    mut state: ResMut<ComparisonState>,
) {
    let Some(model) = session.comparison_model() else {
        return;
    };
    let Some(finish) = route.finish else {
        return;
    };
    let (transform, progress) = human.into_inner();
    if progress.next_gate == route.gates.len()
        && race::finish_contains(transform.translation, finish)
    {
        state.record_finish(model);
    }
}

pub fn update_readout(
    session: Res<SessionMode>,
    state: Res<ComparisonState>,
    readout: Single<&mut Text, With<ComparisonReadout>>,
) {
    let mut text = readout.into_inner();
    let Some(model) = session.comparison_model() else {
        text.0.clear();
        return;
    };
    let best = state
        .best_for(model)
        .map(race::format_duration)
        .unwrap_or_else(|| "--".to_owned());
    let status = if state.phase == ComparisonPhase::Finished {
        "FINISHED"
    } else {
        "RUNNING"
    };
    text.0 = format!(
        "{} {status}\nTIME {}\nBEST {}\n1/2/3 compare | 4 race | R reset | F3 debug",
        model.label(),
        race::format_duration(state.elapsed),
        best,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_time_is_kept_per_model() {
        let mut state = ComparisonState {
            elapsed: Duration::from_secs(8),
            ..default()
        };
        state.record_finish(ControlModel::Shift);
        assert_eq!(
            state.best_for(ControlModel::Shift),
            Some(Duration::from_secs(8))
        );
        state.restart();
        state.elapsed = Duration::from_secs(9);
        state.record_finish(ControlModel::Shift);
        assert_eq!(
            state.best_for(ControlModel::Shift),
            Some(Duration::from_secs(8))
        );
        assert_eq!(state.best_for(ControlModel::Torque), None);
    }

    #[test]
    fn restart_clears_attempt_without_clearing_bests() {
        let mut state = ComparisonState {
            elapsed: Duration::from_secs(4),
            ..default()
        };
        state.record_finish(ControlModel::Force);
        state.restart();
        assert_eq!(state.phase, ComparisonPhase::Running);
        assert_eq!(state.elapsed, Duration::ZERO);
        assert_eq!(
            state.best_for(ControlModel::Force),
            Some(Duration::from_secs(4))
        );
    }

    #[test]
    fn session_selection_keeps_race_and_comparison_separate() {
        let mut session = SessionMode::default();
        assert_eq!(session.comparison_model(), Some(ControlModel::Shift));
        session.select_comparison(ControlModel::Force);
        assert_eq!(
            session.running_comparison_model(),
            Some(ControlModel::Force)
        );
        session.enter_race();
        assert!(session.is_race());
        assert_eq!(session.comparison_model(), None);
    }
}
