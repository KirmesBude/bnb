mod creature;
mod demo;
mod scenario;

use bevy::prelude::*;
use creature::CreaturePlugin;
use demo::DemoPlugin;
use scenario::ScenarioPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ScenarioPlugin, CreaturePlugin, DemoPlugin))
        .run();
}
