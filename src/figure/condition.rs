use bevy::{prelude::*, utils::HashSet};

use crate::{
    game::{EndOfTurn, StartOfTurn},
    scenario::command::{ScenarioCommandQueue, SufferDamageCommand},
};

/* Each figure has a set of possible conditions and  */
/* TODO: You need some way to track end of next turn */
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Reflect)]
pub enum ConditionKind {
    Invisible,
    Strengthen,
    Wound,
    Poison,
    Immobilize,
    Disarm,
    Muddle,
}

/* You want conditions and immunities on the same struct */
#[derive(Debug, Component, Reflect)]
pub struct Conditions {
    conditions: HashSet<ConditionKind>,
    /* Immunities are immutable and can only be specified at creation */
    immunities: HashSet<ConditionKind>,
}

impl Conditions {
    pub fn new(immunities: &[ConditionKind]) -> Self {
        Self {
            conditions: HashSet::new(),
            immunities: immunities.iter().copied().collect(),
        }
    }

    pub fn add_condition(&mut self, condition: ConditionKind) {
        if !self.immunities.contains(&condition) {
            self.conditions.insert(condition);
        }
    }

    pub fn remove_condition(&mut self, condition: ConditionKind) {
        self.conditions.remove(&condition);
    }

    pub fn has(&self, condition: ConditionKind) -> bool {
        self.conditions.contains(&condition)
    }

    pub fn is_immune(&self, condition: ConditionKind) -> bool {
        self.immunities.contains(&condition)
    }
}

pub fn _take_wound_damage(
    mut command_queue: ResMut<ScenarioCommandQueue>,
    mut start_of_turn: EventReader<StartOfTurn>,
) {
    for event in start_of_turn.read() {
        /* TODO: I dont have a proper source here */
        let commands = vec![SufferDamageCommand::new(event.entity, event.entity, 1).into()];
        command_queue.queue(commands);
    }
}

pub fn _remove_condition_on_heal() {}

pub fn _remove_condition_on_end_of_turn(
    mut command_queue: ResMut<ScenarioCommandQueue>,
    mut end_of_turn: EventReader<EndOfTurn>,
) {
    for event in end_of_turn.read() {
        /* TODO: I dont have a proper source here */
        let commands = vec![SufferDamageCommand::new(event.entity, event.entity, 1).into()];
        command_queue.queue(commands);
    }
}
