mod camera;
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
        .init_resource::<mass_shift::InternalMassState>();
    app.add_systems(Startup, (scene::setup_scene, camera::setup_camera))
        .add_systems(
            Update,
            (
                mass_shift::read_keyboard_intent,
                mass_shift::advance_internal_mass.after(mass_shift::read_keyboard_intent),
                scene::draw_mass_shift_display.after(mass_shift::advance_internal_mass),
                camera::follow_ball,
            ),
        )
        .add_systems(
            FixedPostUpdate,
            mass_shift::apply_center_of_mass.before(PhysicsSystems::Prepare),
        )
        .run();
}
