use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::{AQUA, GREEN, RED, WHITE},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use bevy_inspector_egui::egui::collapsing_header::HeaderResponse;
use hexx::{shapes, Hex, HexLayout, HexOrientation, PlaneMeshBuilder};

use crate::{creature::Health, scenario::{command::{AttackCommand, CommandQueue, MoveCommand}, HexGrid, HexLayer, HexPosition}};

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
    mut command_queue: ResMut<CommandQueue>,
) {
    commands.spawn(Camera2d);

    let layout = HexLayout {
        orientation: HexOrientation::Flat,
        hex_size: HEX_SIZE,
        ..default()
    };
    let mesh = meshes.add(hexagonal_plane(&layout));

    let aqua_material = materials.add(Color::from(AQUA));
    let white_material = materials.add(Color::from(WHITE));
    let red_material = materials.add(Color::from(RED));
    let green_material = materials.add(Color::from(GREEN));

    let entities: Vec<Entity> = shapes::flat_rectangle([-2, 2, -3, 2])
        .map(|hex| {
            let entity = commands
                .spawn((
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(white_material.clone()),
                    HexPosition::new(hex, HexLayer::Ground),
                ))
                .observe(update_material_on::<Pointer<Over>>(aqua_material.clone()))
                .observe(update_material_on::<Pointer<Out>>(white_material.clone()))
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

    let figure_a = commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(green_material.clone()),
        HexPosition::new(Hex::new(0, 0), HexLayer::Figure),
        Health::new(12),
    )).id();

    let figure_b = commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(red_material.clone()),
        HexPosition::new(Hex::new(2, 1), HexLayer::Figure),
        Health::new(12),
    )).id();

    commands.spawn(HexGrid::new(layout)).add_children(&entities).add_children(&[figure_a, figure_b]);

    command_queue.queue(Box::new(MoveCommand::new(figure_a, Hex::new(1, 0))));
    command_queue.queue(Box::new(MoveCommand::new(figure_a, Hex::new(1, 1))));
    command_queue.queue(Box::new(AttackCommand::new(figure_a, figure_b)));
}

fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = PlaneMeshBuilder::new(hex_layout)
        .facing(Vec3::Z)
        .with_scale(Vec3::splat(1.0))
        .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

fn update_material_on<E>(
    new_material: Handle<ColorMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial2d<ColorMaterial>>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.entity()) {
            material.0 = new_material.clone();
        }
    }
}
