pub mod condition;
pub mod health;

/* What does a Figure need? */
/* Initiative or is that different? Active Ability? */
/* CalculatedHealth: Base + any bonuses as Health(usize) */
/* CalculatedShield: Base + any bonuses as Shield(usize) */
/* CalculatedRetaliate: Base [+] any bonuses as Retaliate(usize, usize) */
/* AttackEffects: Vec of AttackEffects that are added to any attack this figure does */

use bevy::{prelude::*, utils::HashMap};
use condition::{ConditionKind, Conditions};
use health::Health;

use crate::scenario::HexPosition;

pub struct FigurePlugin;

impl Plugin for FigurePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Team>()
            .register_type::<Health>()
            .register_type::<Conditions>()
            .register_type::<ConditionKind>();
    }
}

#[derive(Debug, Bundle)]
pub struct FigureBundle {
    pub mesh_2d: Mesh2d,
    pub mesh_material_2d: MeshMaterial2d<ColorMaterial>,
    pub hex_position: HexPosition,
    pub health: Health,
    pub conditions: Conditions,
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
#[derive(Debug, Component, PartialEq, Eq, Hash, Reflect)]
pub struct FigureId(u32);

impl FigureId {
    pub fn _new(id: u32) -> Self {
        Self(id)
    }
}

#[derive(Debug, Default, Resource, Reflect)]
pub struct Initiatives {
    initiatives: HashMap<FigureId, u8>,
}
