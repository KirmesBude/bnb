use bevy::prelude::*;

use crate::figure::condition::ConditionKind;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, _app: &mut App) {}
}

pub struct PlayerCard {
    top: Action,
    botton: Action,
}

/* TODO: Technically Action has too many fields for this purpose */
pub struct MonsterCard([Action; 3]);

pub struct Action {
    abilities: Vec<Ability>,
    loss: bool,
    kind: ActionKind, /* TODO: Not sure if correct here, because some abilities might apply, some might not */
}

pub enum ActionKind {
    Instant,
    Round,
    Persistent,
}

pub struct Ability {
    steps: Vec<ConditionalAbilityStep>,
}

/* TODO: These are all more complicated e.g. MovementType, AttackEffects/Target/Range/AreaOfEffect */
pub enum AbilityStep {
    Move(usize),
    Attack(usize),
    Push(usize),
    Pull(usize),
    InfuseElement(Element),
    Heal(usize),
    Shield(usize),
    Retaliate(usize),
    Control,
    SufferDamage(usize),
    Recover,
    AddCondition(ConditionKind),
    RemoveCondition(ConditionKind),
}

pub struct ConditionalAbilityStep {
    condition: AbilityCondition,
    step: AbilityStep,
}

pub enum AbilityCondition {
    None,
    Element(Vec<Element>),
}

pub enum Element {
    Fire,
    Ice,
    Air,
    Earth,
    Light,
    Dark,
    Wild,
}

pub enum TargetKind {
    Ally,
    Enemy,
    Selbst,
}
