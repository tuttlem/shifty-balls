//! The intentionally small replay loop for the first course.

use std::time::Duration;

use avian3d::prelude::{AngularVelocity, CenterOfMass, LinearVelocity};
use bevy::prelude::*;

use crate::camera;
use crate::mass_shift::InternalMassState;
use crate::physics::{FinishRegion, ObservationCamera, PlayerBall, course_start_transform};

#[derive(Resource, Debug, Default)]
pub struct AttemptState {
    pub phase: AttemptPhase,
    pub elapsed: Duration,
    pub completed_time: Option<Duration>,
    pub best_time: Option<Duration>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AttemptPhase {
    #[default]
    Running,
    Completed,
}

impl AttemptState {
    pub fn tick(&mut self, delta: Duration) {
        if self.phase == AttemptPhase::Running {
            self.elapsed += delta;
        }
    }

    pub fn complete(&mut self) {
        if self.phase == AttemptPhase::Completed {
            return;
        }

        self.phase = AttemptPhase::Completed;
        self.completed_time = Some(self.elapsed);
        self.best_time = Some(
            self.best_time
                .map_or(self.elapsed, |best| best.min(self.elapsed)),
        );
    }

    pub fn restart(&mut self) {
        self.phase = AttemptPhase::Running;
        self.elapsed = Duration::ZERO;
        self.completed_time = None;
    }

    pub fn displayed_time(&self) -> Duration {
        self.completed_time.unwrap_or(self.elapsed)
    }
}

#[derive(Resource)]
pub struct DevelopmentDisplay {
    pub visible: bool,
}

impl Default for DevelopmentDisplay {
    fn default() -> Self {
        Self { visible: true }
    }
}

#[derive(Component)]
pub struct AttemptReadout;

pub fn setup_readout(mut commands: Commands) {
    commands.spawn((
        AttemptReadout,
        Text::new("RUN 0.00 s\nBEST --\nSPD 0.0 m/s\nR restart | F3 debug"),
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

pub fn tick_attempt(time: Res<Time>, mut attempt: ResMut<AttemptState>) {
    attempt.tick(time.delta());
}

pub fn finish_contains(position: Vec3, finish: FinishRegion) -> bool {
    let offset = (position - finish.center).abs();
    offset.x <= finish.half_extents.x
        && offset.y <= finish.half_extents.y
        && offset.z <= finish.half_extents.z
}

pub fn check_finish(
    ball: Single<&Transform, With<PlayerBall>>,
    finish: Single<&FinishRegion>,
    mut attempt: ResMut<AttemptState>,
) {
    if finish_contains(ball.translation, **finish) {
        attempt.complete();
    }
}

pub fn handle_restart(
    keys: Res<ButtonInput<KeyCode>>,
    mut attempt: ResMut<AttemptState>,
    mut mass: ResMut<InternalMassState>,
    ball: Single<
        (
            &mut Transform,
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut CenterOfMass,
        ),
        With<PlayerBall>,
    >,
    mut camera_transform: Single<&mut Transform, (With<ObservationCamera>, Without<PlayerBall>)>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    let (mut transform, mut linear_velocity, mut angular_velocity, mut center_of_mass) =
        ball.into_inner();
    *transform = course_start_transform();
    *linear_velocity = LinearVelocity::ZERO;
    *angular_velocity = AngularVelocity::ZERO;
    **center_of_mass = Vec3::ZERO;
    mass.requested_world = Vec3::ZERO;
    mass.current_world = Vec3::ZERO;
    attempt.restart();
    camera::snap_to_course_start(&mut camera_transform);
}

pub fn toggle_development_display(
    keys: Res<ButtonInput<KeyCode>>,
    mut display: ResMut<DevelopmentDisplay>,
) {
    if keys.just_pressed(KeyCode::F3) {
        display.visible = !display.visible;
    }
}

pub fn update_readout(
    attempt: Res<AttemptState>,
    display: Res<DevelopmentDisplay>,
    ball: Single<&LinearVelocity, With<PlayerBall>>,
    readout: Single<(&mut Text, &mut Node), With<AttemptReadout>>,
) {
    let (mut text, mut node) = readout.into_inner();
    node.display = if display.visible {
        Display::Flex
    } else {
        Display::None
    };
    text.0 = format!(
        "{} {}\nBEST {}\nSPD {:.1} m/s\nR restart | F3 debug",
        if attempt.phase == AttemptPhase::Running {
            "RUN"
        } else {
            "FINISHED"
        },
        format_duration(attempt.displayed_time()),
        attempt
            .best_time
            .map_or_else(|| "--".to_owned(), format_duration),
        ball.0.length(),
    );
}

pub fn format_duration(duration: Duration) -> String {
    let total_centiseconds = duration.as_millis() / 10;
    format!(
        "{}.{:02} s",
        total_centiseconds / 100,
        total_centiseconds % 100
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn running_attempt_ticks_and_completed_attempt_stops() {
        let mut attempt = AttemptState::default();
        attempt.tick(Duration::from_secs(2));
        attempt.complete();
        attempt.tick(Duration::from_secs(3));

        assert_eq!(attempt.completed_time, Some(Duration::from_secs(2)));
        assert_eq!(attempt.elapsed, Duration::from_secs(2));
    }

    #[test]
    fn restart_preserves_best_and_clears_active_attempt() {
        let mut attempt = AttemptState {
            elapsed: Duration::from_secs(8),
            best_time: Some(Duration::from_secs(7)),
            ..default()
        };
        attempt.complete();
        attempt.restart();

        assert_eq!(attempt.phase, AttemptPhase::Running);
        assert_eq!(attempt.elapsed, Duration::ZERO);
        assert_eq!(attempt.completed_time, None);
        assert_eq!(attempt.best_time, Some(Duration::from_secs(7)));
    }

    #[test]
    fn completion_is_one_shot_and_only_faster_runs_replace_best() {
        let mut attempt = AttemptState {
            elapsed: Duration::from_secs(10),
            best_time: Some(Duration::from_secs(8)),
            ..default()
        };
        attempt.complete();
        attempt.complete();
        assert_eq!(attempt.best_time, Some(Duration::from_secs(8)));

        attempt.restart();
        attempt.tick(Duration::from_secs(6));
        attempt.complete();
        assert_eq!(attempt.best_time, Some(Duration::from_secs(6)));
    }

    #[test]
    fn finish_bounds_are_inclusive() {
        let finish = FinishRegion {
            center: Vec3::new(5.0, 2.0, -1.0),
            half_extents: Vec3::new(2.0, 1.0, 3.0),
        };
        assert!(finish_contains(Vec3::new(7.0, 3.0, 2.0), finish));
        assert!(!finish_contains(Vec3::new(7.01, 3.0, 2.0), finish));
    }
}
