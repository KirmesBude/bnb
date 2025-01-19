use bevy::prelude::*;

use crate::scenario::command::{
    ScenarioCommand, ScenarioCommandQueue, ScenarioCommandTrait, ScenarionCommandExecuteResult,
};

use super::{condition::Conditions, health::SufferDamageCommand, modifier::RollModifierCommand};

/* Lets try having this be a component. Not sure if that is a good idea */

#[derive(Debug, Clone, Copy, Reflect)]
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
    fn execute(&mut self, _world: &mut World) -> ScenarionCommandExecuteResult {
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
}

impl ApplyAttackCommand {
    pub fn new(source: Entity) -> Self {
        Self { source }
    }
}

impl ScenarioCommandTrait for ApplyAttackCommand {
    fn execute(&mut self, world: &mut World) -> ScenarionCommandExecuteResult {
        /* Retrieve last attack command and any modifier commands from queue */
        let queue = world.get_resource::<ScenarioCommandQueue>().unwrap();
        let (attack, modifiers) = {
            let history = queue.history();
            let mut attack = None;
            let mut modifiers = vec![];

            for command in history {
                match command {
                    ScenarioCommand::AttackCommand(attack_command) => {
                        attack = Some(attack_command.attack);
                        break;
                    }
                    ScenarioCommand::RollModifierCommand(roll_modifier_command) => {
                        modifiers.push(roll_modifier_command.modifier().unwrap())
                    }
                    _ => {}
                }
            }

            (attack.unwrap(), modifiers)
        };

        /* Retrieve target entity and conditions */
        let target_conditions = world.get::<Conditions>(attack.target).unwrap();

        /* Attack bonusse and pentalties (e.g. poison and items) */
        let mut damage = attack.value;
        if target_conditions.has(super::condition::ConditionKind::Poison) {
            damage += 1;
        }

        /* Apply attack modifier */
        /* TODO: Fix casting */
        modifiers.iter().fold(damage as i8, |acc, x| x.apply(acc));

        /* Calculate versus target shield */
        /* TODO: Implement this */

        /* Queue up SufferDamageCommand */
        /* TODO: Queue up Retaliate stuff */
        ScenarionCommandExecuteResult::Done(vec![SufferDamageCommand::new(
            self.source,
            attack.target,
            damage,
        )
        .into()])
    }

    fn undo(self, _world: &mut World) -> ScenarioCommand {
        /* TODO: Consider undoing up to AttackCommand? */

        self.into()
    }
}
