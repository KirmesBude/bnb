use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
    utils::HashMap,
};
use hexx::{Hex, HexLayout};

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
#[component(on_remove = HexPosition::on_remove)]
#[require(Transform)]
pub struct HexPosition {
    hex: Hex,
    layer: HexLayer,
}

impl HexPosition {
    pub fn new(hex: Hex, layer: HexLayer) -> Self {
        Self { hex, layer }
    }

    pub fn hex(&self) -> Hex {
        self.hex
    }

    pub fn layer(&self) -> HexLayer {
        self.layer
    }

    pub fn update(&mut self, hex: Hex, entity: Entity, hex_grid: &mut HexGrid) {
        /* Remove entity if already present */
        hex_grid.remove(&self.hex, &self.layer);

        /* Insert at new place */
        hex_grid.insert(self.hex, &self.layer, entity);

        /* Update the actual position */
        self.hex = hex;
    }

    fn on_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        let (hex, layer, parent) = {
            let hex_position = world.get::<HexPosition>(entity).unwrap();
            let parent = world.get::<Parent>(entity).unwrap();

            (hex_position.hex, hex_position.layer, parent)
        };
        let mut hex_grid = world.get_mut::<HexGrid>(parent.get()).unwrap();

        hex_grid.remove(&hex, &layer);
    }
}

/* TODO: Easing */
pub fn hex_position_to_transform(
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
