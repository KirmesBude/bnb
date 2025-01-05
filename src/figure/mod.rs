mod condition;
mod health;

/* What does a Figure need? */
/* Initiative or is that different? Active Ability? */
/* CalculatedHealth: Base + any bonuses as Health(usize) */
/* CalculatedShield: Base + any bonuses as Shield(usize) */
/* CalculatedRetaliate: Base [+] any bonuses as Retaliate(usize, usize) */
/* AttackEffects: Vec of AttackEffects that are added to any attack this figure does */

use bevy::prelude::*;

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
