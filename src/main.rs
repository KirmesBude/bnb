mod creature;
mod demo;
mod figure;
mod game;
mod scenario;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use creature::CreaturePlugin;
use demo::DemoPlugin;
use game::GamePlugin;
use scenario::ScenarioPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            GamePlugin,
            ScenarioPlugin,
            CreaturePlugin,
            DemoPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
