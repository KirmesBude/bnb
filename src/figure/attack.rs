use bevy::prelude::*;

use crate::scenario::command::{
    ScenarioCommand, ScenarioCommandTrait, ScenarionCommandExecuteResult, SufferDamageCommand,
};

use super::{
    condition::Conditions,
    modifier::{ModifierStack, RollModifierCommand},
};

/* Lets try having this be a component. Not sure if that is a good idea */

#[derive(Debug, Default, Clone, Component, Reflect)]
pub struct PendingAttack {
    attack: Option<Attack>,
    modifiers: ModifierStack,
}

impl PendingAttack {
    pub fn new(attack: Attack) -> Self {
        Self {
            attack: Some(attack),
            modifiers: Default::default(),
        }
    }

    pub fn get_modifiers_mut(&mut self) -> &mut ModifierStack {
        &mut self.modifiers
    }

    pub fn get_attack(&self) -> Option<&Attack> {
        self.attack.as_ref()
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct Attack {
    target: Entity,
    value: usize,
}

impl Attack {
    pub fn new(target: Entity, value: usize) -> Self {
        Self { target, value }
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct AttackCommand {
    source: Entity,
    attack: Attack,
}

impl AttackCommand {
    pub fn new(source: Entity, attack: Attack) -> Self {
        Self { source, attack }
    }
}

impl ScenarioCommandTrait for AttackCommand {
    fn execute(&mut self, world: &mut World) -> ScenarionCommandExecuteResult {
        /* Initialize Pending Attack */
        let mut pending_attack = world.get_mut::<PendingAttack>(self.source).unwrap();
        pending_attack.attack = Some(self.attack.clone());
        pending_attack.modifiers.clear();

        /* Queue up RollModifierCommand */
        /* Queue up ApplyAttackCommand */
        ScenarionCommandExecuteResult::Done(vec![
            RollModifierCommand::new(self.source).into(),
            ApplyAttackCommand::new(self.source).into(),
        ])
    }

    fn undo(self, _world: &mut World) -> ScenarioCommand {
        /* PendingAttack is already reset by ApplyAttackCommand, should be good? */

        self.into()
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct ApplyAttackCommand {
    source: Entity,
    pending_attack: Option<PendingAttack>,
}

impl ApplyAttackCommand {
    pub fn new(source: Entity) -> Self {
        Self {
            source,
            pending_attack: None,
        }
    }
}

impl ScenarioCommandTrait for ApplyAttackCommand {
    fn execute(&mut self, world: &mut World) -> ScenarionCommandExecuteResult {
        /* Retrieve pending attack from source entity */
        let pending_attack = world.get::<PendingAttack>(self.source).unwrap();
        self.pending_attack = Some(pending_attack.clone());

        /* Retrieve target entity and conditions */
        let target = pending_attack.get_attack().unwrap().target;
        let target_conditions = world.get::<Conditions>(target).unwrap();

        /* Attack bonusse and pentalties (e.g. poison and items) */
        let mut damage = pending_attack.get_attack().unwrap().value;
        if target_conditions.has(super::condition::ConditionKind::Poison) {
            damage += 1;
        }

        /* Apply attack modifier */
        /* TODO: Fix this casting madness */
        damage = pending_attack.modifiers.apply(damage);

        /* Calculate versus target shield */
        /* TODO: Implement this */

        /* Queue up SufferDamageCommand */
        /* TODO: Queue up Retaliate stuff */
        ScenarionCommandExecuteResult::Done(vec![SufferDamageCommand::new(
            self.source,
            target,
            damage,
        )
        .into()])
    }

    fn undo(self, world: &mut World) -> ScenarioCommand {
        let mut pending_attack = world.get_mut::<PendingAttack>(self.source).unwrap();
        *pending_attack = self.pending_attack.unwrap();

        let command = Self {
            pending_attack: None,
            ..self
        };
        command.into()
    }
}
