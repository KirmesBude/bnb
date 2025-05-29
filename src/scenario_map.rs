use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::{BLACK, WHITE},
    platform::collections::{HashMap, HashSet},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use hexx::{Hex, HexLayout, PlaneMeshBuilder, shapes::PointyRectangle};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OverlayTile {
    Obstacle,
}

#[derive(Debug, Resource)]
pub struct OverlayTileMaterials {
    pub materials: HashMap<OverlayTile, Handle<ColorMaterial>>,
}

impl OverlayTileMaterials {
    pub fn new(materials: &mut Assets<ColorMaterial>) -> Self {
        let mut map = HashMap::new();

        /* TODO: For each */
        map.insert(OverlayTile::Obstacle, materials.add(Color::Srgba(BLACK)));
        Self { materials: map }
    }
}

#[derive(Debug, Resource)]
pub struct ScenarioMap {
    pub layout: HexLayout,           /* Layout so we can operate on the HexGrid */
    pub base: HashMap<Hex, Entity>,  /* Basically ground entities */
    pub runes: HashMap<Hex, Entity>, /* runes */
}

#[derive(Debug, Resource)]
pub struct ScenarioMapMaterials {
    pub base_material: Handle<ColorMaterial>,
}

/* Path highlighting via material change on ground entity */

const HEX_SIZE: Vec2 = Vec2::splat(14.0);
const MAP_RADIUS: u32 = 20;
const REMOVE_COORDS: [Hex; 3] = [Hex::new(5, 5), Hex::new(6, 3), Hex::new(7, 1)];
const RUNE_LOCATIONS: [Hex; 12] = [
    Hex::new(0, 6),
    Hex::new(2, 6),
    Hex::new(-1, 5),
    Hex::new(-2, 4),
    Hex::new(4, 4),
    Hex::new(1, 3),
    Hex::new(3, 3),
    Hex::new(0, 2),
    Hex::new(6, 2),
    Hex::new(5, 1),
    Hex::new(2, 0),
    Hex::new(4, 0),
];

pub fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let layout = HexLayout {
        scale: HEX_SIZE,
        orientation: hexx::HexOrientation::Pointy,
        ..default()
    };
    let mesh = meshes.add(hexagonal_plane(&layout));

    let base_material = materials.add(Color::Srgba(WHITE));
    let overlay_materials = OverlayTileMaterials::new(&mut materials);
    let shape = PointyRectangle {
        left: 0,
        right: 7,
        top: 0,
        bottom: 6,
    };

    let base = shape
        .coords()
        .enumerate()
        .filter_map(|(i, coord)| {
            if REMOVE_COORDS.contains(&coord) {
                return None;
            }
            let pos = layout.hex_to_world_pos(coord);
            let material = base_material.clone();
            let entity = commands
                .spawn((
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(material.clone_weak()),
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                    children![(
                        Text2d(format!("{},{}", coord.x, coord.y)),
                        TextColor(Color::BLACK),
                        TextFont {
                            font_size: 7.0,
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, 10.0),
                    )],
                ))
                .id();
            Some((coord, entity))
        })
        .collect();

    let runes = RUNE_LOCATIONS
        .into_iter()
        .map(|coord| {
            let pos = layout.hex_to_world_pos(coord);
            let material = overlay_materials
                .materials
                .get(&OverlayTile::Obstacle)
                .unwrap()
                .clone();
            let entity = commands
                .spawn((
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(material.clone_weak()),
                    Transform::from_xyz(pos.x, pos.y, 1.0),
                ))
                .id();
            (coord, entity)
        })
        .collect();

    commands.insert_resource(ScenarioMapMaterials { base_material });
    commands.insert_resource(overlay_materials);
    commands.insert_resource(ScenarioMap {
        layout,
        base,
        runes,
    });
}

/// Compute a bevy mesh from the layout
fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .with_scale(Vec3::splat(0.9))
        .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
