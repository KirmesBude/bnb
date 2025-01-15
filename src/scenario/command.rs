use std::collections::VecDeque;

use bevy::prelude::*;
use enum_dispatch::enum_dispatch;
use hexx::Hex;

use super::HexPosition;
use crate::figure::{
    attack::{ApplyAttackCommand, AttackCommand},
    condition::{ConditionKind, Conditions},
    health::Health,
    modifier::RollModifierCommand,
};

/* Everything that happens in the scenario needs to be recorded (and maybe this is the source of truth?) */
/* Every "action" needs to be reversible */
/* E.g. move, attack need enough information to be 2 way */
/* Do I even want to be able to undo every move? Should I have the possibility? */
/* It definitely needs to be atomic commands like move, but for redo they should be grouped? */
/* Should moving be a group of move commands or should the move command include a list of hexes to move through? */
/* The problem is that a single hex move might have a reaction that needs processing, which in turn creates another command on the stack */

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ScenarioCommandQueue>();
        app.register_type::<ScenarioCommand>()
            .register_type::<MovementKind>()
            .register_type::<ScenarioCommandQueue>()
            .register_type::<MoveCommand>()
            .register_type::<AttackCommand>()
            .register_type::<AddConditionCommand>()
            .register_type::<RemoveConditionCommand>();

        app.add_systems(Update, step_commands);
    }
}

#[derive(Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct ScenarioCommandQueue {
    history: Vec<ScenarioCommand>,
    pending: VecDeque<ScenarioCommand>,
}

impl ScenarioCommandQueue {
    pub fn undo(&mut self, world: &mut World) {
        if let Some(command) = self.history.pop() {
            self.pending.clear();

            let command = command.undo(world);
            self.pending.push_back(command);
        }
    }

    pub fn execute(&mut self, world: &mut World) {
        if let Some(mut command) = self.pending.pop_front() {
            match command.execute(world) {
                ScenarionCommandExecuteResult::Pending => (),
                ScenarionCommandExecuteResult::Done(commands) => {
                    let mut commands: VecDeque<_> = commands.into();
                    self.history.push(command);

                    commands.append(&mut self.pending);
                    self.pending = commands;
                }
            }
        }
    }

    pub fn queue(&mut self, commands: Vec<ScenarioCommand>) {
        self.pending.extend(commands);
    }
}

fn step_commands(world: &mut World) {
    /* SAFETY: Need to ensure disjoint queries, more importantly commands are not allowed to modify ButtonInput<KeyCode> or CommandQueue through World */
    unsafe {
        let world = world.as_unsafe_world_cell();
        let keyboard_input = world.get_resource::<ButtonInput<KeyCode>>().unwrap();
        let mut command_queue = world.get_resource_mut::<ScenarioCommandQueue>().unwrap();

        if keyboard_input.just_pressed(KeyCode::Enter) {
            println!("Enter");

            command_queue.execute(world.world_mut());
        } else if keyboard_input.just_pressed(KeyCode::Backspace) {
            println!("Backspace");

            command_queue.undo(world.world_mut());
        }
    }
}

pub enum ScenarionCommandExecuteResult {
    Pending,
    Done(Vec<ScenarioCommand>),
}

#[enum_dispatch(ScenarioCommand)]
pub trait ScenarioCommandTrait {
    fn execute(&mut self, world: &mut World) -> ScenarionCommandExecuteResult;

    fn undo(self, world: &mut World) -> ScenarioCommand;
}

#[allow(clippy::enum_variant_names)]
#[enum_dispatch]
#[derive(Debug, Clone, Reflect)]
pub enum ScenarioCommand {
    MoveCommand,
    AttackCommand,
    ApplyAttackCommand,
    SufferDamageCommand,
    AddConditionCommand,
    RemoveConditionCommand,
    RollModifierCommand,
}

#[derive(Debug, Default, Clone, Copy, Reflect)]
pub enum MovementKind {
    #[default]
    Default,
    Jump,
    Fly,
}

#[derive(Debug, Clone, Reflect)]
pub struct MoveCommand {
    entity: Entity,
    start: Option<Hex>,
    end: Hex,
    kind: MovementKind,
}

impl MoveCommand {
    pub fn new(entity: Entity, hex: Hex) -> Self {
        Self {
            entity,
            end: hex,
            start: Default::default(),
            kind: Default::default(),
        }
    }

    pub fn _with_kind(mut self, kind: MovementKind) -> Self {
        self.kind = kind;
        self
    }
}

impl ScenarioCommandTrait for MoveCommand {
    fn execute(&mut self, world: &mut World) -> ScenarionCommandExecuteResult {
        let mut entity_world_mut = world.entity_mut(self.entity);
        let mut hex_position = entity_world_mut.get_mut::<HexPosition>().unwrap();
        self.start = Some(hex_position.hex());
        hex_position.update(self.end);

        println!("Move {} to {:?}", self.entity, self.end);

        /* Reactivity how? Via an event that is consumed and someone adds to the queue? */
        ScenarionCommandExecuteResult::Done(vec![])
    }

    fn undo(self, world: &mut World) -> ScenarioCommand {
        let mut entity_world_mut = world.entity_mut(self.entity);
        let mut hex_position = entity_world_mut.get_mut::<HexPosition>().unwrap();

        hex_position.update(self.start.unwrap()); /* TODO: This does not work correctly, because the last_position is lost on HexPosition; But probably not necessary anyways? */

        let command = Self {
            start: None,
            ..self
        };
        command.into()
    }
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

#[derive(Debug, Clone, Reflect)]
pub struct AddConditionCommand {
    entity: Entity,
    condition: ConditionKind,
    added: bool,
}

impl AddConditionCommand {
    pub fn new(entity: Entity, condition: ConditionKind) -> Self {
        Self {
            entity,
            condition,
            added: Default::default(),
        }
    }
}

impl ScenarioCommandTrait for AddConditionCommand {
    fn execute(&mut self, world: &mut World) -> ScenarionCommandExecuteResult {
        let mut entity = world.entity_mut(self.entity);
        let mut conditions = entity.get_mut::<Conditions>().unwrap();

        self.added = !conditions.has(self.condition) && !conditions.is_immune(self.condition);
        conditions.add_condition(self.condition);

        ScenarionCommandExecuteResult::Done(vec![])
    }

    fn undo(self, world: &mut World) -> ScenarioCommand {
        /* Only undo if a condition was actually added this way */
        if self.added {
            let mut entity: EntityWorldMut<'_> = world.entity_mut(self.entity);
            let mut conditions = entity.get_mut::<Conditions>().unwrap();

            conditions.remove_condition(self.condition);
        }

        let command = Self {
            added: false,
            ..self
        };
        command.into()
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct RemoveConditionCommand {
    entity: Entity,
    condition: ConditionKind,
    removed: bool,
}

impl RemoveConditionCommand {
    pub fn new(entity: Entity, condition: ConditionKind) -> Self {
        Self {
            entity,
            condition,
            removed: Default::default(),
        }
    }
}

impl ScenarioCommandTrait for RemoveConditionCommand {
    fn execute(&mut self, world: &mut World) -> ScenarionCommandExecuteResult {
        let mut entity = world.entity_mut(self.entity);
        let mut conditions = entity.get_mut::<Conditions>().unwrap();

        self.removed = conditions.has(self.condition);
        conditions.remove_condition(self.condition);

        ScenarionCommandExecuteResult::Done(vec![])
    }

    fn undo(self, world: &mut World) -> ScenarioCommand {
        /* Only undo if a condition was actually removed this way */
        if self.removed {
            let mut entity: EntityWorldMut<'_> = world.entity_mut(self.entity);
            let mut conditions = entity.get_mut::<Conditions>().unwrap();

            conditions.add_condition(self.condition);
        }

        let command = Self {
            removed: false,
            ..self
        };
        command.into()
    }
}
