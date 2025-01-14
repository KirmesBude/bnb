mod action;
mod demo;
mod figure;
mod game;
mod player;
mod scenario;

use action::ActionPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use demo::DemoPlugin;
use figure::FigurePlugin;
use game::GamePlugin;
use scenario::{command::CommandPlugin, ScenarioPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            GamePlugin,
            ScenarioPlugin,
            DemoPlugin,
            CommandPlugin,
            FigurePlugin,
            ActionPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
