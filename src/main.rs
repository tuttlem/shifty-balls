mod ai;
mod camera;
mod comparison;
mod control_model;
mod course;
mod mass_shift;
mod physics;
mod race;
mod scene;

use avian3d::prelude::PhysicsSystems;
use bevy::prelude::*;
use bevy::window::Window;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Shifty Balls".into(),
            ..default()
        }),
        ..default()
    }));
    physics::configure(&mut app);
    app.init_resource::<mass_shift::MassShiftTuning>()
        .init_resource::<control_model::ControlModelTuning>()
        .init_resource::<control_model::ControlIntent>()
        .init_resource::<comparison::SessionMode>()
        .init_resource::<comparison::ComparisonState>()
        .init_resource::<race::RaceState>()
        .init_resource::<race::DevelopmentDisplay>();
    app.add_systems(
        Startup,
        (
            scene::setup_scene,
            course::setup_course,
            camera::setup_camera,
            race::setup_readout,
            comparison::setup_readout,
            comparison::initialise_comparison.after(scene::setup_scene),
        ),
    )
    .add_systems(
        Update,
        (
            comparison::select_or_reset_session,
            race::handle_rematch.after(comparison::select_or_reset_session),
            race::tick_race.after(race::handle_rematch),
            race::release_racers.after(race::tick_race),
            control_model::read_keyboard_intent.after(race::release_racers),
            ai::set_ai_mass_intent.after(race::release_racers),
            mass_shift::advance_internal_mass
                .after(control_model::read_keyboard_intent)
                .after(ai::set_ai_mass_intent),
            race::update_progress.after(mass_shift::advance_internal_mass),
            race::check_finishes.after(race::update_progress),
            comparison::tick_comparison.after(comparison::select_or_reset_session),
            comparison::update_progress.after(mass_shift::advance_internal_mass),
            comparison::check_finish.after(comparison::update_progress),
            race::toggle_development_display.after(race::check_finishes),
            scene::draw_mass_shift_display.after(race::toggle_development_display),
            race::update_readout.after(race::check_finishes),
            comparison::update_readout.after(comparison::check_finish),
            camera::follow_ball.after(race::handle_rematch),
        ),
    )
    .add_systems(
        FixedPostUpdate,
        (
            mass_shift::apply_center_of_mass,
            control_model::apply_selected_physics,
        )
            .before(PhysicsSystems::Prepare),
    )
    .run();
}
