use bevy::prelude::*;
use hexx::Hex;

use crate::figure::health::Health;
use dyn_clone::DynClone;
use super::HexPosition;

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

        app.add_systems(Update, step_commands);
    }
}

/* TODO: Could be non_send_resource */
pub trait Command: Sync + Send + DynClone {
    fn execute(&mut self, world: &mut World) -> Vec<Box<dyn Command>>;

    fn undo(&self, world: &mut World);
}

#[derive(Default, Resource)]
pub struct CommandQueue {
    queue: Vec<Box<dyn Command>>,
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

    pub fn queue(&mut self, command: Box<dyn Command>) {
        if self.queue.is_empty() {
            self.queue.push(command);
        } else {
            self.queue.insert(self.cursor + 1, command);
        }
    }

    pub fn execute(&mut self, world: &mut World) {
        if let Some(command) = self.queue.get_mut(self.cursor) {
            let commands = command.execute(world);

            self.queue
                .splice(self.cursor + 1..self.cursor + 1, commands);
            self.cursor += 1;

            println!("{} {}", self.queue.len(), self.cursor);
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum MovementKind {
    #[default]
    Default,
    Jump,
    Fly,
}

#[derive(Clone)]
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
}

impl Command for MoveCommand {
    fn execute(&mut self, world: &mut World) -> Vec<Box<dyn Command>> {
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

#[derive(Clone)]
pub struct AttackCommand {
    source: Entity,
    target: Entity,
}

impl AttackCommand {
    pub fn new(source: Entity, target: Entity) -> Self {
        Self { source, target }
    }
}

impl Command for AttackCommand {
    fn execute(&mut self, world: &mut World) -> Vec<Box<dyn Command>> {
        /* TODO: Store pending attack on one of the entities and add additional commands for modifier deck, etc. */

        vec![Box::new(SufferDamageCommand::new(
            self.source,
            self.target,
            2,
        ))]
    }

    fn undo(&self, world: &mut World) {
        /* I think you dont do anything? Maybe some resource on the source */
    }
}

#[derive(Clone)]
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

impl Command for SufferDamageCommand {
    fn execute(&mut self, world: &mut World) -> Vec<Box<dyn Command>> {
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
        let mut command = dyn_clone::clone_box(&*command_queue.queue[command_queue.cursor]);
        let commands = command.execute(world);

        let mut command_queue = world.get_resource_mut::<CommandQueue>().unwrap();
        let cursor = command_queue.cursor;
        command_queue.queue
        .splice(cursor + 1..cursor + 1, commands);
        command_queue.cursor += 1;
    }
}
