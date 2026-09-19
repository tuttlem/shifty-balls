mod attempt;
mod camera;
mod course;
mod mass_shift;
mod physics;
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
        .init_resource::<mass_shift::InternalMassState>()
        .init_resource::<attempt::AttemptState>()
        .init_resource::<attempt::DevelopmentDisplay>();
    app.add_systems(
        Startup,
        (
            scene::setup_scene,
            course::setup_course,
            camera::setup_camera,
            attempt::setup_readout,
        ),
    )
    .add_systems(
        Update,
        (
            mass_shift::read_keyboard_intent,
            mass_shift::advance_internal_mass.after(mass_shift::read_keyboard_intent),
            attempt::handle_restart.after(mass_shift::advance_internal_mass),
            attempt::check_finish.after(attempt::handle_restart),
            attempt::tick_attempt.after(attempt::check_finish),
            attempt::toggle_development_display.after(attempt::tick_attempt),
            scene::draw_mass_shift_display.after(attempt::toggle_development_display),
            attempt::update_readout.after(attempt::toggle_development_display),
            camera::follow_ball.after(attempt::handle_restart),
        ),
    )
    .add_systems(
        FixedPostUpdate,
        mass_shift::apply_center_of_mass.before(PhysicsSystems::Prepare),
    )
    .run();
}
