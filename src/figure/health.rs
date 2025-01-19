use bevy::prelude::*;

use crate::scenario::command::{
    ScenarioCommand, ScenarioCommandTrait, ScenarionCommandExecuteResult,
};

#[derive(Debug, Default, Component, Reflect)]
pub struct CalculatedHealth(usize);

#[derive(Debug, Component, Reflect)]
pub struct Health {
    current: usize,
    max: usize,
}

impl Health {
    pub fn new(max: usize) -> Self {
        Self { max, current: max }
    }

    pub fn suffer(&mut self, damage: usize) -> usize {
        let actual_damage = self.current.min(damage);

        self.current = self.current.saturating_sub(damage);

        actual_damage
    }

    pub fn heal(&mut self, heal: usize) -> usize {
        let new_current = self.current.saturating_add(heal);
        let actual_heal = new_current - self.current;

        self.current = new_current;
        actual_heal
    }
}

/* This is fired whenever an entity is healed */
#[derive(Debug, Event, Reflect)]
pub struct Healed {
    pub entity: Entity,
}

#[derive(Debug, Clone, Reflect)]
pub struct SufferDamageCommand {
    source: Entity,
    target: Entity,
    damage: usize,
    actual_damage: Option<usize>,
}

impl SufferDamageCommand {
    pub fn new(source: Entity, target: Entity, damage: usize) -> Self {
        Self {
            source,
            target,
            damage,
            actual_damage: Default::default(),
        }
    }
}

impl ScenarioCommandTrait for SufferDamageCommand {
    fn execute(&mut self, world: &mut World) -> ScenarionCommandExecuteResult {
        let mut target = world.entity_mut(self.target);
        let mut health = target.get_mut::<Health>().unwrap();
        self.actual_damage = Some(health.suffer(self.damage));

        /* TODO: Should be Pending until user input event is received */
        ScenarionCommandExecuteResult::Done(vec![])
    }

    fn undo(self, world: &mut World) -> ScenarioCommand {
        let mut target = world.entity_mut(self.target);
        let mut health = target.get_mut::<Health>().unwrap();

        /* TODO: Might not want to make this heal for semantic reasons */
        health.heal(self.actual_damage.unwrap());

        let command = Self {
            actual_damage: None,
            ..self
        };
        command.into()
    }
}
