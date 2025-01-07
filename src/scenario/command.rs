use bevy::prelude::*;
use hexx::Hex;

use super::HexPosition;
use crate::{figure::health::Health, scenario::command};

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
        app.init_resource::<CommandQueue>();
        app.register_type::<Command>()
            .register_type::<MovementKind>()
            .register_type::<CommandQueue>();

        app.add_systems(Update, step_commands);
    }
}

#[derive(Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct CommandQueue {
    queue: Vec<Command>,
    cursor: usize,
}

impl CommandQueue {
    pub fn undo(&mut self, world: &mut World) {
        if !self.queue.is_empty() {
            /* Any commands after the current one dont matter */
            self.queue.truncate(self.cursor + 1);
            /* Reduce cursor */
            self.cursor -= 1;
            /* Pop the last one and undo */
            self.queue.pop().unwrap().undo(world);
        }
    }

    pub fn queue(&mut self, commands: Vec<Command>) {
        if self.queue.is_empty() {
            self.queue = commands;
        } else {
            self.queue
                .splice(self.cursor + 1..self.cursor + 1, commands);
        }
    }
}

#[derive(Debug, Clone, Reflect)]
pub enum Command {
    MoveCommand(MoveCommand),
    AttackCommand(AttackCommand),
    SufferDamageCommand(SufferDamageCommand),
}

impl Command {
    fn execute(&mut self, world: &mut World) -> Vec<Command> {
        match self {
            Command::MoveCommand(move_command) => move_command.execute(world),
            Command::AttackCommand(attack_command) => attack_command.execute(world),
            Command::SufferDamageCommand(suffer_damage_command) => {
                suffer_damage_command.execute(world)
            }
        }
    }

    fn undo(&self, world: &mut World) {
        match self {
            Command::MoveCommand(move_command) => move_command.undo(world),
            Command::AttackCommand(attack_command) => attack_command.undo(world),
            Command::SufferDamageCommand(suffer_damage_command) => {
                suffer_damage_command.undo(world)
            }
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

    pub fn with_kind(mut self, kind: MovementKind) -> Self {
        self.kind = kind;
        self
    }

    fn execute(&mut self, world: &mut World) -> Vec<Command> {
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

#[derive(Debug, Clone, Reflect)]
pub struct AttackCommand {
    source: Entity,
    target: Entity,
}

impl AttackCommand {
    pub fn new(source: Entity, target: Entity) -> Self {
        Self { source, target }
    }

    fn execute(&mut self, world: &mut World) -> Vec<Command> {
        /* TODO: Store pending attack on one of the entities and add additional commands for modifier deck, etc. */

        vec![Command::SufferDamageCommand(SufferDamageCommand::new(
            self.source,
            self.target,
            2,
        ))]
    }

    fn undo(&self, world: &mut World) {
        /* I think you dont do anything? Maybe some resource on the source */
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

    fn execute(&mut self, world: &mut World) -> Vec<Command> {
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

fn step_commands(world: &mut World) {
    let keyboard_input = world.get_resource::<ButtonInput<KeyCode>>().unwrap();

    if keyboard_input.just_pressed(KeyCode::Enter) {
        println!("Enter");

        let command_queue = world.get_resource::<CommandQueue>().unwrap();
        let cursor = command_queue.cursor;
        if let Some(mut command) = command_queue.queue.get(cursor).cloned() {
            let commands = command.execute(world);

            let mut command_queue = world.get_resource_mut::<CommandQueue>().unwrap();
            command_queue.queue[cursor] = command;
            command_queue.queue(commands);

            command_queue.cursor += 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::Backspace) {
        println!("Backspace");

        let mut command_queue = world.get_resource_mut::<CommandQueue>().unwrap();
        command_queue.cursor = command_queue.cursor.saturating_sub(1);

        if let Some(command) = command_queue.queue.pop() {
            command.undo(world);
        }
    }
}
