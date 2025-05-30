use bevy::{
    color::palettes::css::{AQUA, BLACK, WHITE},
    platform::collections::{HashMap, HashSet},
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
    window::PrimaryWindow,
};
use hexx::{algorithms::a_star, *};
use scenario_map::{
    handle_input, reset_counter2, reset_material, setup_map, update_counter, update_counter2,
    update_pick_display, update_range_display,
};

pub mod scenario_map;

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(14.0);
const MAP_RADIUS: u32 = 20;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_map))
        .add_systems(
            Update,
            (
                handle_input,
                update_counter,
                reset_counter2,
                update_counter2,
                reset_material,
                update_range_display,
                update_pick_display,
            )
                .chain(),
        )
        .run();
}

/// 2D camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_scale(Vec3::new(0.5, 0.5, 1.0))));
}
