mod camera;
mod physics;
mod scene;

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
    app.add_systems(Startup, (scene::setup_scene, camera::setup_camera))
        .add_systems(Update, camera::follow_ball)
        .run();
}
