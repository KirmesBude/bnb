use bevy::prelude::*;
use map::{
    hex_position_to_transform, update_hex_position_hashmap, ActiveMap, HexGrid, HexLayer,
    HexPosition,
};

pub mod command;
pub mod map;
pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_hex_position_hashmap, hex_position_to_transform).chain(),
        )
        .init_resource::<ActiveMap>();

        app.register_type::<ActiveMap>();
        app.register_type::<HexGrid>();
        app.register_type::<HexLayer>();
        app.register_type::<HexPosition>();
    }
}
