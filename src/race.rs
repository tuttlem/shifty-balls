//! The smallest race lifecycle and current-course progression model.
//!
//! Race rules are deliberately pure where practical. They do not decide how a
//! ball moves: controllers request internal mass positions, and physics decides
//! every line, collision, recovery, and finish approach.

use std::{cmp::Ordering, time::Duration};

use avian3d::prelude::{
    AngularVelocity, CenterOfMass, Collider, LinearVelocity, RigidBody, Rotation, WakeBody,
};
use bevy::prelude::*;

use crate::{
    camera,
    comparison::SessionMode,
    mass_shift::InternalMassState,
    physics::{
        BALL_RADIUS_METRES, COUNTDOWN_SECONDS, CourseRoute, HumanRacer, ObservationCamera,
        RACER_COUNT, RaceOnly, Racer, RacerStart,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RacePhase {
    Countdown { remaining: Duration },
    Racing,
    PlayerFinished,
}

impl Default for RacePhase {
    fn default() -> Self {
        Self::Countdown {
            remaining: Duration::from_secs_f32(COUNTDOWN_SECONDS),
        }
    }
}

#[derive(Resource, Debug)]
pub struct RaceState {
    pub phase: RacePhase,
    pub elapsed: Duration,
    next_finish_place: usize,
    release_pending: bool,
    reset_pending: bool,
}

impl Default for RaceState {
    fn default() -> Self {
        Self {
            phase: RacePhase::Countdown {
                remaining: Duration::from_secs_f32(COUNTDOWN_SECONDS),
            },
            elapsed: Duration::ZERO,
            next_finish_place: 1,
            release_pending: false,
            reset_pending: false,
        }
    }
}

impl RaceState {
    pub fn is_racing(&self) -> bool {
        self.phase == RacePhase::Racing
    }

    pub fn tick(&mut self, delta: Duration) {
        match self.phase {
            RacePhase::Countdown { remaining } if delta >= remaining => {
                self.phase = RacePhase::Racing;
                self.release_pending = true;
            }
            RacePhase::Countdown { remaining } => {
                self.phase = RacePhase::Countdown {
                    remaining: remaining - delta,
                };
            }
            RacePhase::Racing => self.elapsed += delta,
            RacePhase::PlayerFinished => {}
        }
    }

    pub fn take_release(&mut self) -> bool {
        std::mem::take(&mut self.release_pending)
    }

    pub fn record_finish(&mut self) -> usize {
        let place = self.next_finish_place;
        self.next_finish_place += 1;
        place
    }

    pub fn finish_player(&mut self) {
        self.phase = RacePhase::PlayerFinished;
    }

    pub fn restart(&mut self) {
        *self = Self::default();
    }

    pub fn request_reset(&mut self) {
        self.restart();
        self.reset_pending = true;
    }

    pub fn take_reset(&mut self) -> bool {
        std::mem::take(&mut self.reset_pending)
    }

    pub fn countdown_text(&self) -> String {
        match self.phase {
            RacePhase::Countdown { remaining } => (remaining.as_secs_f32().ceil() as u32)
                .clamp(1, COUNTDOWN_SECONDS as u32)
                .to_string(),
            RacePhase::Racing => "GO".to_owned(),
            RacePhase::PlayerFinished => "FINISHED".to_owned(),
        }
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct RacerProgress {
    pub next_gate: usize,
    pub finished_place: Option<usize>,
    pub finished_time: Option<Duration>,
}

#[derive(Clone, Copy, Debug)]
pub struct RaceOrderEntry {
    pub id: u8,
    pub finished_place: Option<usize>,
    pub gates_passed: usize,
    pub leg_advance: f32,
}

pub fn ordered_racers(entries: &mut [RaceOrderEntry]) {
    entries.sort_by(
        |left, right| match (left.finished_place, right.finished_place) {
            (Some(left_place), Some(right_place)) => left_place
                .cmp(&right_place)
                .then_with(|| left.id.cmp(&right.id)),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => right
                .gates_passed
                .cmp(&left.gates_passed)
                .then_with(|| {
                    right
                        .leg_advance
                        .partial_cmp(&left.leg_advance)
                        .unwrap_or(Ordering::Equal)
                })
                .then_with(|| left.id.cmp(&right.id)),
        },
    );
}

pub fn advance_through_gate(progress: &mut RacerProgress, gate_index: usize) -> bool {
    if progress.finished_place.is_none() && progress.next_gate == gate_index {
        progress.next_gate += 1;
        true
    } else {
        false
    }
}

pub fn finish_if_eligible(
    progress: &mut RacerProgress,
    gate_count: usize,
    elapsed: Duration,
    finish_place: usize,
) -> bool {
    if progress.finished_place.is_none() && progress.next_gate == gate_count {
        progress.finished_place = Some(finish_place);
        progress.finished_time = Some(elapsed);
        true
    } else {
        false
    }
}

#[derive(Resource, Default)]
pub struct DevelopmentDisplay {
    pub visible: bool,
}

#[derive(Component)]
pub struct RaceReadout;

type RacerResetData = (
    Entity,
    &'static RacerStart,
    &'static mut Transform,
    &'static mut Rotation,
    &'static mut LinearVelocity,
    &'static mut AngularVelocity,
    &'static mut CenterOfMass,
    &'static mut InternalMassState,
    &'static mut RacerProgress,
    Has<HumanRacer>,
);

pub fn setup_readout(mut commands: Commands) {
    commands.spawn((
        RaceReadout,
        Text::new("3\nPLACE 1 / 4\nTIME 0.00 s"),
        TextFont {
            font_size: FontSize::Px(24.0),
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

pub fn tick_race(time: Res<Time>, session: Res<SessionMode>, mut race: ResMut<RaceState>) {
    if session.is_race() {
        race.tick(time.delta());
    }
}

pub fn release_racers(
    mut commands: Commands,
    session: Res<SessionMode>,
    mut race: ResMut<RaceState>,
    mut racers: Query<
        (
            Entity,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut CenterOfMass,
            &mut InternalMassState,
        ),
        With<Racer>,
    >,
) {
    if !session.is_race() || !race.take_release() {
        return;
    }

    for (entity, mut linear, mut angular, mut center_of_mass, mut mass) in &mut racers {
        commands.entity(entity).insert(RigidBody::Dynamic);
        *linear = LinearVelocity::ZERO;
        *angular = AngularVelocity::ZERO;
        **center_of_mass = Vec3::ZERO;
        mass.requested_world = Vec3::ZERO;
        mass.current_world = Vec3::ZERO;
        commands.queue(WakeBody(entity));
    }
}

pub fn update_progress(
    session: Res<SessionMode>,
    race: Res<RaceState>,
    route: Res<CourseRoute>,
    mut racers: Query<(&Transform, &mut RacerProgress), With<Racer>>,
) {
    if !session.is_race() || !race.is_racing() {
        return;
    }

    for (transform, mut progress) in &mut racers {
        if let Some(gate) = route.gates.get(progress.next_gate)
            && gate.contains(transform.translation)
        {
            let gate_index = progress.next_gate;
            advance_through_gate(&mut progress, gate_index);
        }
    }
}

pub fn check_finishes(
    session: Res<SessionMode>,
    mut race: ResMut<RaceState>,
    route: Res<CourseRoute>,
    mut racers: Query<(&Racer, &Transform, &mut RacerProgress, Has<HumanRacer>)>,
) {
    if !session.is_race() || !race.is_racing() {
        return;
    }
    let Some(finish) = route.finish else {
        return;
    };

    let mut eligible: Vec<u8> = racers
        .iter()
        .filter_map(|(racer, transform, progress, _)| {
            (progress.finished_place.is_none()
                && progress.next_gate == route.gates.len()
                && finish_contains(transform.translation, finish))
            .then_some(racer.id)
        })
        .collect();
    eligible.sort_unstable();

    for id in eligible {
        for (racer, transform, mut progress, is_human) in &mut racers {
            if racer.id == id
                && finish_contains(transform.translation, finish)
                && finish_if_eligible(
                    &mut progress,
                    route.gates.len(),
                    race.elapsed,
                    race.record_finish(),
                )
                && is_human
            {
                race.finish_player();
            }
        }
    }
}

pub fn finish_contains(position: Vec3, finish: crate::physics::FinishRegion) -> bool {
    let offset = (position - finish.center).abs();
    offset.x <= finish.half_extents.x
        && offset.y <= finish.half_extents.y
        && offset.z <= finish.half_extents.z
}

pub fn player_place(
    route: &CourseRoute,
    racers: &Query<(&Racer, &Transform, &RacerProgress)>,
) -> usize {
    let mut entries: Vec<RaceOrderEntry> = racers
        .iter()
        .map(|(racer, transform, progress)| RaceOrderEntry {
            id: racer.id,
            finished_place: progress.finished_place,
            gates_passed: progress.next_gate,
            leg_advance: leg_advance(route, transform.translation, progress.next_gate),
        })
        .collect();
    ordered_racers(&mut entries);
    entries
        .iter()
        .position(|entry| entry.id == 0)
        .map_or(1, |index| index + 1)
}

fn leg_advance(route: &CourseRoute, position: Vec3, next_gate: usize) -> f32 {
    route.gates.get(next_gate).map_or(1.0, |gate| {
        let offset = position - gate.center;
        (1.0 - offset.dot(gate.forward.normalize_or_zero()) / 20.0).clamp(-0.25, 1.0)
    })
}

pub fn update_readout(
    session: Res<SessionMode>,
    race: Res<RaceState>,
    route: Res<CourseRoute>,
    racers: Query<(&Racer, &Transform, &RacerProgress)>,
    human: Single<&RacerProgress, With<HumanRacer>>,
    readout: Single<&mut Text, With<RaceReadout>>,
) {
    let mut text = readout.into_inner();
    if !session.is_race() {
        text.0.clear();
        return;
    }
    let place = player_place(&route, &racers);
    text.0 = match race.phase {
        RacePhase::PlayerFinished => format!(
            "{} PLACE\n{}\nR rematch | F3 debug",
            ordinal(human.finished_place.unwrap_or(place)),
            format_duration(human.finished_time.unwrap_or(race.elapsed)),
        ),
        _ => format!(
            "{}\nPLACE {} / {}\nTIME {}\nR rematch | F3 debug",
            race.countdown_text(),
            place,
            RACER_COUNT,
            format_duration(race.elapsed),
        ),
    };
}

pub fn handle_rematch(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    session: Res<SessionMode>,
    mut race: ResMut<RaceState>,
    mut racers: Query<RacerResetData, (With<Racer>, Without<ObservationCamera>)>,
    race_only: Query<Entity, With<RaceOnly>>,
    mut camera_transform: Single<&mut Transform, (With<ObservationCamera>, Without<HumanRacer>)>,
) {
    if !session.is_race() {
        return;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        race.request_reset();
    }
    if !race.take_reset() {
        return;
    }

    let mut human_start = None;
    for (
        entity,
        start,
        mut transform,
        mut rotation,
        mut linear,
        mut angular,
        mut center_of_mass,
        mut mass,
        mut progress,
        is_human,
    ) in &mut racers
    {
        *transform = start.transform;
        **rotation = start.transform.rotation;
        commands.entity(entity).insert(RigidBody::Kinematic);
        *linear = LinearVelocity::ZERO;
        *angular = AngularVelocity::ZERO;
        **center_of_mass = Vec3::ZERO;
        *mass = InternalMassState::default();
        *progress = RacerProgress::default();
        if is_human {
            human_start = Some(start.transform.translation);
        }
    }
    for entity in &race_only {
        commands.entity(entity).insert((
            Collider::sphere(BALL_RADIUS_METRES),
            RigidBody::Kinematic,
            Visibility::Visible,
        ));
    }
    if let Some(position) = human_start {
        camera::snap_to_human_position(&mut camera_transform, position);
    }
}

pub fn toggle_development_display(
    keys: Res<ButtonInput<KeyCode>>,
    mut display: ResMut<DevelopmentDisplay>,
) {
    if keys.just_pressed(KeyCode::F3) {
        display.visible = !display.visible;
    }
}

pub fn format_duration(duration: Duration) -> String {
    let centiseconds = duration.as_millis() / 10;
    format!("{}.{:02} s", centiseconds / 100, centiseconds % 100)
}

fn ordinal(place: usize) -> String {
    match place {
        1 => "1ST".to_owned(),
        2 => "2ND".to_owned(),
        3 => "3RD".to_owned(),
        _ => format!("{place}TH"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::ProgressionGate;

    #[test]
    fn countdown_releases_once_and_only_racing_ticks_time() {
        let mut race = RaceState::default();
        race.tick(Duration::from_secs(2));
        assert!(!race.is_racing());
        assert_eq!(race.elapsed, Duration::ZERO);
        race.tick(Duration::from_secs(1));
        assert!(race.is_racing());
        assert!(race.take_release());
        assert!(!race.take_release());
        race.tick(Duration::from_secs(2));
        assert_eq!(race.elapsed, Duration::from_secs(2));
    }

    #[test]
    fn progression_requires_the_exact_next_gate() {
        let mut progress = RacerProgress::default();
        assert!(!advance_through_gate(&mut progress, 2));
        assert!(advance_through_gate(&mut progress, 0));
        assert!(!advance_through_gate(&mut progress, 0));
        assert_eq!(progress.next_gate, 1);
    }

    #[test]
    fn an_inserted_half_pipe_gate_stays_ordered_and_is_required_for_the_finish() {
        let mut progress = RacerProgress::default();
        const GATE_COUNT_WITH_HALF_PIPE: usize = 5;

        assert!(advance_through_gate(&mut progress, 0));
        assert!(advance_through_gate(&mut progress, 1));
        assert!(!advance_through_gate(&mut progress, 3));
        assert!(advance_through_gate(&mut progress, 2));
        assert!(!finish_if_eligible(
            &mut progress,
            GATE_COUNT_WITH_HALF_PIPE,
            Duration::from_secs(12),
            1,
        ));
        assert!(advance_through_gate(&mut progress, 3));
        assert!(advance_through_gate(&mut progress, 4));
        assert!(finish_if_eligible(
            &mut progress,
            GATE_COUNT_WITH_HALF_PIPE,
            Duration::from_secs(12),
            1,
        ));
    }

    #[test]
    fn finishing_requires_all_gates_and_is_immutable() {
        let mut progress = RacerProgress::default();
        assert!(!finish_if_eligible(
            &mut progress,
            2,
            Duration::from_secs(3),
            1
        ));
        progress.next_gate = 2;
        assert!(finish_if_eligible(
            &mut progress,
            2,
            Duration::from_secs(3),
            1
        ));
        assert!(!finish_if_eligible(
            &mut progress,
            2,
            Duration::from_secs(4),
            2
        ));
        assert_eq!(progress.finished_place, Some(1));
    }

    #[test]
    fn finished_then_progress_then_id_defines_order() {
        let mut entries = [
            RaceOrderEntry {
                id: 2,
                finished_place: None,
                gates_passed: 1,
                leg_advance: 0.5,
            },
            RaceOrderEntry {
                id: 1,
                finished_place: Some(2),
                gates_passed: 0,
                leg_advance: 0.0,
            },
            RaceOrderEntry {
                id: 0,
                finished_place: Some(1),
                gates_passed: 0,
                leg_advance: 0.0,
            },
            RaceOrderEntry {
                id: 3,
                finished_place: None,
                gates_passed: 1,
                leg_advance: 0.5,
            },
        ];
        ordered_racers(&mut entries);
        assert_eq!(entries.map(|entry| entry.id), [0, 1, 2, 3]);
    }

    #[test]
    fn restart_restores_countdown_and_clears_timer() {
        let mut race = RaceState {
            phase: RacePhase::PlayerFinished,
            elapsed: Duration::from_secs(8),
            next_finish_place: 3,
            release_pending: false,
            reset_pending: false,
        };
        race.restart();
        assert!(matches!(race.phase, RacePhase::Countdown { .. }));
        assert_eq!(race.elapsed, Duration::ZERO);
        assert_eq!(race.record_finish(), 1);
    }

    #[test]
    fn oriented_gate_accepts_lateral_line_but_not_shortcut() {
        let gate = ProgressionGate {
            center: Vec3::ZERO,
            forward: Vec3::Z,
            half_width: 4.0,
            half_height: 2.0,
            half_depth: 1.0,
        };
        assert!(gate.contains(Vec3::new(3.5, 1.0, 0.5)));
        assert!(!gate.contains(Vec3::new(4.1, 0.0, 0.0)));
    }
}
