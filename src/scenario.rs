use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};

pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (map_position_add, change_map_position_update, sync_position_to_transform).chain());
        app.register_type::<MapPosition>();
    }
}

#[derive(Debug, Component)]
#[require(Transform)]
pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Entity>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![Vec::new(); width * height],
        }
    }

    pub fn update(&mut self, entity: Entity, position: &MapPosition) {
        /* remove entity from last position */
        self.remove(entity, position.last_x, position.last_y);

        /* add entity to new position */
        self.add(entity, position.x, position.y);
    }

    pub fn add(&mut self, entity: Entity, x: usize, y: usize) {
        let vec = &mut self.tiles[y * self.width + x];
        vec.push(entity);
    }

    pub fn remove(&mut self, entity: Entity, x: usize, y: usize) {
        let vec = &mut self.tiles[y * self.width + x];
        if let Some(pos) = vec.iter().position(|e| *e == entity) {
            vec.remove(pos);
        }
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
#[component(on_remove = MapPosition::on_remove)]
pub struct MapPosition {
    map: Entity,
    x: usize,
    y: usize,
    last_x: usize,
    last_y: usize,
}

impl MapPosition {
    pub fn new(map: Entity, x: usize, y: usize) -> Self {
        Self {
            map,
            x,
            y,
            last_x: 0,
            last_y: 0,
        }
    }

    pub fn update(&mut self, x: usize, y: usize) {
        self.last_x = self.x;
        self.last_y = self.y;
        self.x = x;
        self.y = y;
    }

    pub fn map(&self) -> Entity {
        self.map
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn on_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        let (x, y) = {
            let component = world.get::<Self>(entity).unwrap();
            (component.x, component.y)
        };
        let mut map = world.get_mut::<Map>(entity).unwrap();

        map.remove(entity, x, y);
    }
}

pub fn map_position_add(
    mut maps: Query<&mut Map>,
    map_positions: Query<(Entity, &MapPosition), Added<MapPosition>>,
) {
    for (entity, map_position) in &map_positions {
        if let Ok(mut map) = maps.get_mut(map_position.map) {
            map.add(entity, map_position.x, map_position.y);
        }
    }
}

pub fn change_map_position_update(
    mut maps: Query<&mut Map>,
    map_positions: Query<(Entity, &MapPosition), Changed<MapPosition>>,
) {
    for (entity, map_position) in &map_positions {
        if let Ok(mut map) = maps.get_mut(map_position.map) {
            map.update(entity, map_position);
        }
    }
}

/* TODO: Should use hierarchy */
fn sync_position_to_transform(
    maps: Query<(&GlobalTransform, &Map)>,
    mut map_positions: Query<(&mut Transform, &MapPosition)>
) {
    for (mut transform, map_position) in &mut map_positions {
        if let Ok((map_transform, map)) = maps.get(map_position.map) {
            /* TODO: Does not support map rotation */
            let tile_size = 50.0;
            transform.translation = map_transform.translation() + Vec3::new(((map_position.x as f32)-((map.width as f32)/2.0)) * tile_size, ((map_position.y as f32)-((map.height as f32)/2.0)) * tile_size, 0.0);
        }
    }
}