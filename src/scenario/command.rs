use std::collections::VecDeque;

use bevy::prelude::*;
use hexx::Hex;

use super::HexPosition;
use crate::figure::{
    condition::{ConditionKind, Conditions},
    health::Health,
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
            command.undo(world);
            self.pending.clear();
        }
    }

    pub fn execute(&mut self, world: &mut World) {
        if let Some(mut command) = self.pending.pop_front() {
            let mut commands: VecDeque<ScenarioCommand> = command.execute(world).into();
            self.history.push(command);

            commands.append(&mut self.pending);
            self.pending = commands;
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

/* TODO: Have a trait for execute/undo */

#[derive(Debug, Clone, Reflect)]
pub enum ScenarioCommand {
    Move(MoveCommand),
    Attack(AttackCommand),
    SufferDamage(SufferDamageCommand),
    AddCondition(AddConditionCommand),
    RemoveCondition(RemoveConditionCommand),
}

impl ScenarioCommand {
    fn execute(&mut self, world: &mut World) -> Vec<Self> {
        match self {
            Self::Move(move_command) => move_command.execute(world),
            Self::Attack(attack_command) => attack_command.execute(world),
            Self::SufferDamage(suffer_damage_command) => suffer_damage_command.execute(world),
            Self::AddCondition(add_condition_command) => add_condition_command.execute(world),
            Self::RemoveCondition(remove_condition_command) => {
                remove_condition_command.execute(world)
            }
        }
    }

    fn undo(&self, world: &mut World) {
        match self {
            Self::Move(move_command) => move_command.undo(world),
            Self::Attack(attack_command) => attack_command.undo(world),
            Self::SufferDamage(suffer_damage_command) => suffer_damage_command.undo(world),
            Self::AddCondition(add_condition_command) => add_condition_command.undo(world),
            Self::RemoveCondition(remove_condition_command) => remove_condition_command.undo(world),
        }
    }
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

    fn execute(&mut self, world: &mut World) -> Vec<ScenarioCommand> {
        let mut entity_world_mut = world.entity_mut(self.entity);
        let mut hex_position = entity_world_mut.get_mut::<HexPosition>().unwrap();
        self.start = Some(hex_position.hex());
        hex_position.update(self.end);

        println!("Move {} to {:?}", self.entity, self.end);

        /* Reactivity how? Via an event that is consumed and someone adds to the queue? */
        vec![]
    }

    fn undo(&self, world: &mut World) {
        let mut entity_world_mut = world.entity_mut(self.entity);
        let mut hex_position = entity_world_mut.get_mut::<HexPosition>().unwrap();

        hex_position.update(self.start.unwrap()); /* TODO: This does not work correctly, because the last_position is lost on HexPosition; But probably not necessary anyways? */
    }
}

impl From<MoveCommand> for ScenarioCommand {
    fn from(value: MoveCommand) -> Self {
        Self::Move(value)
    }
}

#[derive(Debug, Clone, Reflect)]
pub struct AttackCommand {
    source: Entity,
    target: Entity,
}

impl AttackCommand {
    pub fn new(source: Entity, target: Entity) -> Self {
        Self { source, target }
    }

    fn execute(&mut self, _world: &mut World) -> Vec<ScenarioCommand> {
        /* TODO: Store pending attack on one of the entities and add additional commands for modifier deck, etc. */

        vec![SufferDamageCommand::new(self.source, self.target, 2).into()]
    }

    fn undo(&self, _world: &mut World) {
        /* I think you dont do anything? Maybe some resource on the source */
    }
}

impl From<AttackCommand> for ScenarioCommand {
    fn from(value: AttackCommand) -> Self {
        Self::Attack(value)
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

    fn execute(&mut self, world: &mut World) -> Vec<ScenarioCommand> {
        let mut target = world.entity_mut(self.target);
        let mut health = target.get_mut::<Health>().unwrap();
        self.actual_damage = Some(health.suffer(self.damage));

        vec![]
    }

    fn undo(&self, world: &mut World) {
        let mut target = world.entity_mut(self.target);
        let mut health = target.get_mut::<Health>().unwrap();

        /* TODO: Might not want to make this heal for semantic reasons */
        health.heal(self.actual_damage.unwrap());
    }
}

impl From<SufferDamageCommand> for ScenarioCommand {
    fn from(value: SufferDamageCommand) -> Self {
        Self::SufferDamage(value)
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

    fn execute(&mut self, world: &mut World) -> Vec<ScenarioCommand> {
        let mut entity = world.entity_mut(self.entity);
        let mut conditions = entity.get_mut::<Conditions>().unwrap();

        self.added = !conditions.has(self.condition) && !conditions.is_immune(self.condition);
        conditions.add_condition(self.condition);

        vec![]
    }

    fn undo(&self, world: &mut World) {
        /* Only undo if a condition was actually added this way */
        if self.added {
            let mut entity: EntityWorldMut<'_> = world.entity_mut(self.entity);
            let mut conditions = entity.get_mut::<Conditions>().unwrap();

            conditions.remove_condition(self.condition);
        }
    }
}

impl From<AddConditionCommand> for ScenarioCommand {
    fn from(value: AddConditionCommand) -> Self {
        Self::AddCondition(value)
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

    fn execute(&mut self, world: &mut World) -> Vec<ScenarioCommand> {
        let mut entity = world.entity_mut(self.entity);
        let mut conditions = entity.get_mut::<Conditions>().unwrap();

        self.removed = conditions.has(self.condition);
        conditions.remove_condition(self.condition);

        vec![]
    }

    fn undo(&self, world: &mut World) {
        /* Only undo if a condition was actually removed this way */
        if self.removed {
            let mut entity: EntityWorldMut<'_> = world.entity_mut(self.entity);
            let mut conditions = entity.get_mut::<Conditions>().unwrap();

            conditions.add_condition(self.condition);
        }
    }
}

impl From<RemoveConditionCommand> for ScenarioCommand {
    fn from(value: RemoveConditionCommand) -> Self {
        Self::RemoveCondition(value)
    }
}
