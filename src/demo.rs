use bevy::{color::palettes::css::{BLACK, BLUE, RED, WHITE}, prelude::*};

use crate::scenario::{Map, MapPosition};

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    let map = commands.spawn(Map::new(8, 8)).id();

    commands.spawn((Sprite {
        color: WHITE.into(),
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
    }, MapPosition::new(map, 3, 4)));

    commands.spawn((Sprite {
        color: BLACK.into(),
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
    }, MapPosition::new(map, 4, 4)));

    commands.spawn((Sprite {
        color: BLUE.into(),
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
    }, MapPosition::new(map, 3, 3)));

    commands.spawn((Sprite {
        color: RED.into(),
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
    }, MapPosition::new(map, 3, 2)));
}