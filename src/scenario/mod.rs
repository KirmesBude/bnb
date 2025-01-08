use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
    utils::HashMap,
};
use hexx::{Hex, HexLayout};

pub mod command;
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

/* This resource is used to retrieve the active map entity for hierarchy reasons of overlay tiles spawning */
/* Can also be used to despawn the whole map entity */
#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct ActiveMap {
    entity: Option<Entity>,
}

/* This component is inserted on the map entity */
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
#[require(Transform, Visibility)]
pub struct HexGrid {
    layout: HexLayout,
    /* This can be used to check whether it is a valid hex at all */
    ground_entities: HashMap<Hex, Entity>,
    overlay_entities: HashMap<Hex, Entity>,
    figure_entities: HashMap<Hex, Entity>,
}

impl HexGrid {
    pub fn new(layout: HexLayout) -> Self {
        Self {
            layout,
            ground_entities: HashMap::new(),
            overlay_entities: HashMap::new(),
            figure_entities: HashMap::new(),
        }
    }

    pub fn remove(&mut self, hex: &Hex, layer: &HexLayer) {
        let map = self.get_layer_map_mut(layer);

        map.remove(hex);
    }

    pub fn insert(&mut self, hex: Hex, layer: &HexLayer, entity: Entity) {
        let map = self.get_layer_map_mut(layer);

        map.insert(hex, entity);
    }

    fn _get_layer_map(&self, layer: &HexLayer) -> &HashMap<Hex, Entity> {
        match layer {
            HexLayer::Ground => &self.ground_entities,
            HexLayer::Overlay => &self.overlay_entities,
            HexLayer::Figure => &self.figure_entities,
        }
    }

    fn get_layer_map_mut(&mut self, layer: &HexLayer) -> &mut HashMap<Hex, Entity> {
        match layer {
            HexLayer::Ground => &mut self.ground_entities,
            HexLayer::Overlay => &mut self.overlay_entities,
            HexLayer::Figure => &mut self.figure_entities,
        }
    }
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum HexLayer {
    Ground,
    Overlay,
    Figure,
}

impl HexLayer {
    pub fn z(&self) -> f32 {
        match self {
            HexLayer::Ground => 0.0,
            HexLayer::Overlay => 1.0,
            HexLayer::Figure => 2.0,
        }
    }

    pub fn scale(&self) -> Vec3 {
        match self {
            HexLayer::Ground => Vec3::splat(0.98),
            HexLayer::Overlay => Vec3::splat(0.95),
            HexLayer::Figure => Vec3::splat(0.9),
        }
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
#[component(on_remove = hex_position_on_remove)]
#[require(Transform)]
pub struct HexPosition {
    hex: Hex,
    previous_hex: Option<Hex>, /* TODO: Might not be needed if encoded in CommandQueue */
    layer: HexLayer,
}

impl HexPosition {
    pub fn new(hex: Hex, layer: HexLayer) -> Self {
        Self {
            hex,
            layer,
            previous_hex: None,
        }
    }

    pub fn update(&mut self, hex: Hex) {
        self.previous_hex = Some(self.hex);
        self.hex = hex;
    }

    pub fn hex(&self) -> Hex {
        self.hex
    }
}

fn hex_position_on_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
    let (hex, layer, parent) = {
        let hex_position = world.get::<HexPosition>(entity).unwrap();
        let parent = world.get::<Parent>(entity).unwrap();

        (hex_position.hex, hex_position.layer, parent)
    };
    let mut hex_grid = world.get_mut::<HexGrid>(parent.get()).unwrap();

    hex_grid.remove(&hex, &layer);
}

/* TODO: Easing */
fn hex_position_to_transform(
    hex_grids: Query<&HexGrid>,
    mut hex_positions: Query<(&HexPosition, &mut Transform, &Parent), Changed<HexPosition>>,
) {
    for (hex_position, mut transform, parent) in &mut hex_positions {
        if let Ok(hex_grid) = hex_grids.get(parent.get()) {
            let pos = hex_grid.layout.hex_to_world_pos(hex_position.hex);
            let z = hex_position.layer.z();
            let scale = hex_position.layer.scale();

            *transform = Transform::from_xyz(pos.x, pos.y, z).with_scale(scale);
        }
    }
}

fn update_hex_position_hashmap(
    mut hex_grids: Query<&mut HexGrid>,
    hex_positions: Query<(Entity, &HexPosition, &Parent), Changed<HexPosition>>,
) {
    for (entity, hex_position, parent) in &hex_positions {
        if let Ok(mut hex_grid) = hex_grids.get_mut(parent.get()) {
            if let Some(previous_hex) = hex_position.previous_hex {
                hex_grid.remove(&previous_hex, &hex_position.layer);
            }
            hex_grid.insert(hex_position.hex, &hex_position.layer, entity);
        }
    }
}
