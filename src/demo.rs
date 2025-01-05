use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::WHITE,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use hexx::{shapes, HexLayout, HexOrientation, PlaneMeshBuilder};

use crate::scenario::{HexGrid, HexLayer, HexPosition};

pub struct DemoPlugin;

const HEX_SIZE: Vec2 = Vec2::splat(20.0);

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let layout = HexLayout {
        orientation: HexOrientation::Flat,
        hex_size: HEX_SIZE,
        ..default()
    };
    let mesh = meshes.add(hexagonal_plane(&layout));

    let entities: Vec<Entity> = shapes::flat_rectangle([-2, 2, -3, 2])
        .map(|hex| {
            let entity = commands
                .spawn((
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(materials.add(Color::from(WHITE))),
                    HexPosition::new(hex, HexLayer::Ground),
                ))
                .with_children(|b| {
                    b.spawn((
                        Text2d(format!("{},{}", hex.x, hex.y)),
                        TextColor(Color::BLACK),
                        TextFont {
                            font_size: 7.0,
                            ..default()
                        },
                        Transform::from_xyz(0.0, 0.0, 10.0),
                    ));
                })
                .id();
            entity
        })
        .collect();

    commands.spawn(HexGrid::new(layout)).add_children(&entities);
}

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
