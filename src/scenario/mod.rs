use bevy::prelude::*;
use map::{hex_position_to_transform, ActiveMap, HexGrid, HexLayer, HexPosition};

pub mod command;
pub mod map;
pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hex_position_to_transform)
            .init_resource::<ActiveMap>();

        app.register_type::<ActiveMap>();
        app.register_type::<HexGrid>();
        app.register_type::<HexLayer>();
        app.register_type::<HexPosition>();
    }
}
