pub mod ai;
pub mod attack;
pub mod condition;
pub mod health;
pub mod modifier;
pub mod movement;

/* What does a Figure need? */
/* Initiative or is that different? Active Ability? */
/* CalculatedHealth: Base + any bonuses as Health(usize) */
/* CalculatedShield: Base + any bonuses as Shield(usize) */
/* CalculatedRetaliate: Base [+] any bonuses as Retaliate(usize, usize) */
/* AttackEffects: Vec of AttackEffects that are added to any attack this figure does */

use bevy::{prelude::*, utils::HashMap};
use condition::{ConditionKind, Conditions};
use health::Health;
use modifier::{Modifier, ModifierTray, ModifierTrayColumn, ModifierTrays};

use crate::scenario::map::HexPosition;

pub struct FigurePlugin;

impl Plugin for FigurePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Team>()
            .register_type::<Health>()
            .register_type::<Conditions>()
            .register_type::<ConditionKind>();

        app.register_type::<FigureId>();

        app.register_type::<Modifier>()
            .register_type::<ModifierTrayColumn>()
            .register_type::<ModifierTray>()
            .register_type::<ModifierTrays>();
        app.init_resource::<ModifierTrays>();
    }
}

#[derive(Debug, Bundle)]
pub struct FigureBundle {
    pub mesh_2d: Mesh2d,
    pub mesh_material_2d: MeshMaterial2d<ColorMaterial>,
    pub hex_position: HexPosition,
    pub health: Health,
    pub conditions: Conditions,
    pub id: FigureId,
}

/* TODO: Or should those be marker component to query for? */
#[derive(Debug, Component, Reflect)]
pub enum Team {
    Monster,
    Player,
    Ally,
}

/* This is a list of entities that have bonuses like Health, Shield, Retaliate, AttackEffects */
#[derive(Debug, Component, Reflect)]
pub struct ActiveBonuses {
    bonuses: Vec<Entity>,
}

/* This is an identifier for each type of figure. */
/* e.g. Craigheart might be 0 and Skeleton might be 1 */
/* TODO: Summons should have the same id as the owner */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
pub struct FigureId(u32);

impl FigureId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

#[derive(Debug, Default, Resource, Reflect)]
pub struct Initiatives {
    initiatives: HashMap<FigureId, u8>,
}
